#!/usr/bin/env python3
"""
Comprehensive test script for HPVMx HTTP server on port 8080.
Tests all endpoints, HTTP methods, response structure, timing, and connection handling.
Provides detailed diagnostics to identify server issues.

Usage:
    python test_http_server.py <server_ip>
    Example: python test_http_server.py 10.0.2.4
"""

import sys
import socket
import time
import struct
import hashlib


# ─── Configuration ───────────────────────────────────────────────────────────

DEFAULT_PORT = 8080
TIMEOUT_CONNECT = 10       # Seconds to wait for TCP connection
TIMEOUT_DATA = 10          # Seconds to wait for response data
RETRY_COUNT = 1            # Retries per test (0 = no retry)
DELAY_BETWEEN_REQUESTS = 2  # Seconds between sequential requests


# ─── Result Tracking ─────────────────────────────────────────────────────────

class TestResult:
    def __init__(self):
        self.passed = 0
        self.failed = 0
        self.skipped = 0
        self.details = []  # List of (name, passed, message) tuples

    def add(self, name: str, passed: bool, message: str = ""):
        self.details.append((name, passed, message))
        if passed:
            self.passed += 1
        else:
            self.failed += 1

    def skip(self, name: str, message: str = ""):
        self.details.append((name, None, message))
        self.skipped += 1


results = TestResult()


# ─── Helper Functions ────────────────────────────────────────────────────────

def section_header(title: str):
    print(f"\n{'=' * 70}")
    print(f"  {title}")
    print(f"{'=' * 70}")


def sub_header(title: str):
    print(f"\n--- {title} ---")


def pass_msg(msg: str):
    print(f"  ✓ PASS: {msg}")


def fail_msg(msg: str):
    print(f"  ✗ FAIL: {msg}")


def info_msg(msg: str):
    print(f"  ℹ  INFO: {msg}")


def parse_http_response(data: bytes):
    """Parse raw HTTP response into status line, headers dict, and body."""
    if b"\r\n\r\n" not in data:
        return None, None, None, data

    header_end = data.index(b"\r\n\r\n") + 4
    header_text = data[:header_end].decode("utf-8", errors="replace")
    body = data[header_end:]

    lines = header_text.split("\r\n")
    if not lines:
        return None, None, None, data

    status_line = lines[0]
    headers = {}
    for line in lines[1:]:
        if ":" in line:
            key, value = line.split(":", 1)
            headers[key.strip().lower()] = value.strip()

    # Parse status code
    status_code = None
    status_text = ""
    parts = status_line.split(" ", 2)
    if len(parts) >= 2:
        try:
            status_code = int(parts[1])
        except ValueError:
            pass
        if len(parts) >= 3:
            status_text = parts[2]

    return status_line, status_code, headers, body


def send_raw_http_request(server_ip, port, request_bytes, label=""):
    """Send raw HTTP request via socket and return (response_data, timing_info)."""
    sock = None
    try:
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        sock.settimeout(TIMEOUT_CONNECT)

        # Measure TCP handshake time
        t0 = time.monotonic()
        sock.connect((server_ip, port))
        tcp_handshake_time = (time.monotonic() - t0) * 1000

        t1 = time.monotonic()
        sock.sendall(request_bytes)
        send_time = (time.monotonic() - t1) * 1000

        # Receive response
        sock.settimeout(TIMEOUT_DATA)
        response_data = b""
        first_byte_time = None
        while True:
            try:
                sock.settimeout(TIMEOUT_DATA)
                chunk = sock.recv(4096)
                if not chunk:
                    break
                if first_byte_time is None and response_data:
                    first_byte_time = (time.monotonic() - t1) * 1000
                response_data += chunk

                # Check for complete response
                if b"\r\n\r\n" in response_data:
                    header_end = response_data.index(b"\r\n\r\n") + 4
                    headers_raw = response_data[:header_end].decode("utf-8", errors="replace").lower()

                    # Check Content-Length
                    content_length = None
                    if "content-length:" in headers_raw:
                        for line in headers_raw.split("\r\n"):
                            if line.startswith("content-length:"):
                                try:
                                    content_length = int(line.split(":")[1].strip())
                                except ValueError:
                                    pass

                    body_so_far = response_data[header_end:]
                    if content_length and len(body_so_far) >= content_length:
                        break
                    elif "connection: close" in headers_raw and first_byte_time:
                        # Wait a bit more for the connection to close
                        try:
                            sock.settimeout(1)
                            more = sock.recv(4096)
                            if more:
                                response_data += more
                            break
                        except socket.timeout:
                            break

            except socket.timeout:
                break

        total_time = (time.monotonic() - t0) * 1000

        timing = {
            "tcp_handshake_ms": tcp_handshake_time,
            "send_ms": send_time,
            "first_byte_ms": first_byte_time,
            "total_ms": total_time,
        }
        return response_data, timing

    finally:
        if sock:
            try:
                sock.close()
            except Exception:
                pass


def build_http_request(method, path, body=None, http_version="1.1", extra_headers=None):
    """Build a raw HTTP request byte string."""
    req = f"{method} {path} HTTP/{http_version}\r\n"
    req += "Host: test-host\r\n"
    req += "User-Agent: HPVMx-Test/2.0\r\n"
    req += "Connection: close\r\n"

    if extra_headers:
        for k, v in extra_headers.items():
            req += f"{k}: {v}\r\n"

    if body:
        if isinstance(body, str):
            body = body.encode("utf-8")
        req += f"Content-Length: {len(body)}\r\n"
        req += "\r\n"
        return req.encode("utf-8") + body
    else:
        req += "\r\n"
        return req.encode("utf-8")


# ─── Diagnostic Tests ────────────────────────────────────────────────────────

def test_tcp_connectivity(server_ip, port):
    """Test basic TCP connectivity to the server."""
    section_header("DIAGNOSTIC: TCP Connectivity")
    info_msg(f"Attempting TCP connection to {server_ip}:{port}")

    try:
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        sock.settimeout(TIMEOUT_CONNECT)
        t0 = time.monotonic()
        sock.connect((server_ip, port))
        elapsed = (time.monotonic() - t0) * 1000
        sock.close()
        pass_msg(f"TCP connection established in {elapsed:.0f}ms")
        results.add("TCP Connectivity", True, f"{elapsed:.0f}ms")
        return True
    except socket.timeout:
        fail_msg(f"Connection timed out after {TIMEOUT_CONNECT}s")
        results.add("TCP Connectivity", False, "Timeout")
        return False
    except ConnectionRefusedError:
        fail_msg("Connection refused — is the server running?")
        results.add("TCP Connectivity", False, "Refused")
        return False
    except Exception as e:
        fail_msg(f"Connection error: {type(e).__name__}: {e}")
        results.add("TCP Connectivity", False, str(e))
        return False


def test_tcp_reset_detection(server_ip, port):
    """Test whether the server sends RST for invalid connections."""
    section_header("DIAGNOSTIC: RST Detection")
    info_msg("Sending invalid TCP packet to check for RST response")

    try:
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        sock.settimeout(3)
        sock.connect((server_ip, port))

        # Send garbage data (not a valid HTTP request)
        sock.sendall(b"GARBAGE\r\n\r\n")
        time.sleep(0.5)

        # Try to receive — if we get RST, recv will raise an error
        try:
            sock.settimeout(2)
            data = sock.recv(1024)
            if data:
                info_msg(f"Server sent data instead of RST: {data[:50]}")
                results.add("RST Detection", False, "No RST, got data")
            else:
                info_msg("Server closed connection (acceptable)")
                results.add("RST Detection", True, "Connection closed")
        except ConnectionResetError:
            pass_msg("Server sent RST (expected for invalid data)")
            results.add("RST Detection", True, "RST received")
        except socket.timeout:
            info_msg("No response to garbage data (server may ignore invalid requests)")
            results.add("RST Detection", True, "Timeout (acceptable)")

        sock.close()
        return True
    except Exception as e:
        fail_msg(f"Error: {type(e).__name__}: {e}")
        results.add("RST Detection", False, str(e))
        return False


# ─── Endpoint Tests ──────────────────────────────────────────────────────────

def test_get_root(server_ip, port):
    """Test GET / endpoint."""
    section_header("TEST: GET / (Root Page)")

    request = build_http_request("GET", "/")
    info_msg(f"Request: {request.decode().strip()}")

    for attempt in range(RETRY_COUNT + 1):
        if attempt > 0:
            info_msg(f"Retry {attempt}/{RETRY_COUNT}...")
            time.sleep(DELAY_BETWEEN_REQUESTS)

        response, timing = send_raw_http_request(server_ip, port, request, "GET /")

        if not response:
            fail_msg("No response received")
            results.add("GET /", False, "No response")
            continue

        status_line, status_code, headers, body = parse_http_response(response)

        # Timing report
        sub_header("Timing")
        info_msg(f"TCP handshake: {timing['tcp_handshake_ms']:.0f}ms")
        if timing['first_byte_ms']:
            info_msg(f"Time to first byte: {timing['first_byte_ms']:.0f}ms")
        info_msg(f"Total time: {timing['total_ms']:.0f}ms")

        # Validate status code
        sub_header("Validation")
        checks_passed = True

        if status_code == 200:
            pass_msg(f"Status code: {status_code} OK")
        else:
            fail_msg(f"Status code: {status_code} (expected 200)")
            checks_passed = False

        # Validate headers
        if headers and "content-type" in headers:
            ct = headers["content-type"]
            if "text/html" in ct:
                pass_msg(f"Content-Type: {ct}")
            else:
                fail_msg(f"Content-Type: {ct} (expected text/html)")
                checks_passed = False
        else:
            fail_msg("Missing Content-Type header")
            checks_passed = False

        if headers and "content-length" in headers:
            declared_len = int(headers["content-length"])
            actual_len = len(body)
            if declared_len == actual_len:
                pass_msg(f"Content-Length: {declared_len} (matches body)")
            else:
                fail_msg(f"Content-Length: {declared_len} but body is {actual_len} bytes")
                checks_passed = False
        else:
            fail_msg("Missing Content-Length header")
            checks_passed = False

        if headers and "server" in headers:
            pass_msg(f"Server header: {headers['server']}")
        else:
            info_msg("Missing Server header (optional)")

        # Validate body content
        if body:
            body_text = body.decode("utf-8", errors="replace")
            if "HPVMx" in body_text:
                pass_msg("Body contains 'HPVMx'")
            else:
                fail_msg("Body does not contain 'HPVMx'")
                checks_passed = False

            if "<!DOCTYPE" in body_text or "<html" in body_text.lower():
                pass_msg("Body is valid HTML")
            else:
                fail_msg("Body does not appear to be HTML")
                checks_passed = False

            info_msg(f"Body length: {len(body)} bytes")
            if len(body) < 300:
                sub_header("Body Preview")
                print(f"  {body_text[:300]}")
        else:
            fail_msg("Empty body")
            checks_passed = False

        results.add("GET /", checks_passed, f"Status {status_code}, {len(body) if body else 0} bytes")
        return checks_passed

    return False


def test_get_status(server_ip, port):
    """Test GET /status endpoint."""
    section_header("TEST: GET /status")

    request = build_http_request("GET", "/status")

    for attempt in range(RETRY_COUNT + 1):
        if attempt > 0:
            info_msg(f"Retry {attempt}/{RETRY_COUNT}...")
            time.sleep(DELAY_BETWEEN_REQUESTS)

        response, timing = send_raw_http_request(server_ip, port, request, "GET /status")

        if not response:
            fail_msg("No response received")
            results.add("GET /status", False, "No response")
            continue

        status_line, status_code, headers, body = parse_http_response(response)

        checks_passed = True
        if status_code == 200:
            pass_msg(f"Status code: {status_code}")
        else:
            fail_msg(f"Status code: {status_code} (expected 200)")
            checks_passed = False

        if body:
            body_text = body.decode("utf-8", errors="replace")
            if "Status" in body_text or "status" in body_text.lower():
                pass_msg("Body contains status information")
            else:
                fail_msg("Body does not contain expected status text")
                checks_passed = False
            info_msg(f"Body: {len(body)} bytes")
        else:
            fail_msg("Empty body")
            checks_passed = False

        results.add("GET /status", checks_passed, f"Status {status_code}")
        return checks_passed

    return False


def test_post_echo(server_ip, port):
    """Test POST /echo endpoint with body content."""
    section_header("TEST: POST /echo")

    echo_body = "Hello, HPVMx! <test>&'quote'</test>"
    request = build_http_request("POST", "/echo", echo_body)

    for attempt in range(RETRY_COUNT + 1):
        if attempt > 0:
            info_msg(f"Retry {attempt}/{RETRY_COUNT}...")
            time.sleep(DELAY_BETWEEN_REQUESTS)

        response, timing = send_raw_http_request(server_ip, port, request, "POST /echo")

        if not response:
            fail_msg("No response received")
            results.add("POST /echo", False, "No response")
            continue

        status_line, status_code, headers, body = parse_http_response(response)

        checks_passed = True
        if status_code == 200:
            pass_msg(f"Status code: {status_code}")
        else:
            fail_msg(f"Status code: {status_code} (expected 200)")
            checks_passed = False

        if body:
            body_text = body.decode("utf-8", errors="replace")
            # Check if echo content is present (may be HTML-escaped)
            if "Hello" in body_text:
                pass_msg("Echo body contains 'Hello'")
            else:
                fail_msg("Echo body does not contain expected content")
                checks_passed = False

            # Check HTML escaping
            if "&lt;" in body_text or "<test>" in body_text:
                pass_msg("HTML escaping handled")
            else:
                info_msg("HTML escaping not verified")

            info_msg(f"Body: {len(body)} bytes")
        else:
            fail_msg("Empty body")
            checks_passed = False

        results.add("POST /echo", checks_passed, f"Status {status_code}")
        return checks_passed

    return False


def test_patch_config(server_ip, port):
    """Test PATCH /config endpoint."""
    section_header("TEST: PATCH /config")

    request = build_http_request("PATCH", "/config", '{"theme": "dark"}')

    for attempt in range(RETRY_COUNT + 1):
        if attempt > 0:
            info_msg(f"Retry {attempt}/{RETRY_COUNT}...")
            time.sleep(DELAY_BETWEEN_REQUESTS)

        response, timing = send_raw_http_request(server_ip, port, request, "PATCH /config")

        if not response:
            fail_msg("No response received")
            results.add("PATCH /config", False, "No response")
            continue

        status_line, status_code, headers, body = parse_http_response(response)

        checks_passed = True
        if status_code == 200:
            pass_msg(f"Status code: {status_code}")
        else:
            fail_msg(f"Status code: {status_code} (expected 200)")
            checks_passed = False

        if body:
            body_text = body.decode("utf-8", errors="replace")
            if "Config" in body_text or "config" in body_text.lower() or "Updated" in body_text:
                pass_msg("Body contains config update confirmation")
            else:
                info_msg(f"Body content: {body_text[:100]}")

            info_msg(f"Body: {len(body)} bytes")
        else:
            fail_msg("Empty body")
            checks_passed = False

        results.add("PATCH /config", checks_passed, f"Status {status_code}")
        return checks_passed

    return False


def test_404_not_found(server_ip, port):
    """Test that non-existent paths return 404."""
    section_header("TEST: GET /nonexistent (404)")

    request = build_http_request("GET", "/nonexistent")

    for attempt in range(RETRY_COUNT + 1):
        if attempt > 0:
            info_msg(f"Retry {attempt}/{RETRY_COUNT}...")
            time.sleep(DELAY_BETWEEN_REQUESTS)

        response, timing = send_raw_http_request(server_ip, port, request, "GET /nonexistent")

        if not response:
            fail_msg("No response received")
            results.add("GET /nonexistent (404)", False, "No response")
            continue

        status_line, status_code, headers, body = parse_http_response(response)

        checks_passed = True
        if status_code == 404:
            pass_msg(f"Status code: {status_code} (correct)")
        else:
            fail_msg(f"Status code: {status_code} (expected 404)")
            checks_passed = False

        if body:
            body_text = body.decode("utf-8", errors="replace")
            if "404" in body_text or "Not Found" in body_text:
                pass_msg("Body contains 404 message")
            else:
                info_msg(f"Body: {body_text[:100]}")
        else:
            info_msg("Empty body (acceptable for 404)")

        results.add("GET /nonexistent (404)", checks_passed, f"Status {status_code}")
        return checks_passed

    return False


def test_405_method_not_allowed(server_ip, port):
    """Test that wrong HTTP method returns 405."""
    section_header("TEST: DELETE / (405)")

    request = build_http_request("DELETE", "/")

    for attempt in range(RETRY_COUNT + 1):
        if attempt > 0:
            info_msg(f"Retry {attempt}/{RETRY_COUNT}...")
            time.sleep(DELAY_BETWEEN_REQUESTS)

        response, timing = send_raw_http_request(server_ip, port, request, "DELETE /")

        if not response:
            fail_msg("No response received")
            results.add("DELETE / (405)", False, "No response")
            continue

        status_line, status_code, headers, body = parse_http_response(response)

        checks_passed = True
        if status_code == 405:
            pass_msg(f"Status code: {status_code} (correct)")
        elif status_code == 404:
            # Server may return 404 for DELETE as well
            info_msg(f"Status code: {status_code} (404 is acceptable for unsupported method)")
        else:
            fail_msg(f"Status code: {status_code} (expected 405 or 404)")
            checks_passed = False

        results.add("DELETE / (405)", checks_passed, f"Status {status_code}")
        return checks_passed

    return False


# ─── HTTP/1.0 Compatibility Test ─────────────────────────────────────────────

def test_http10_compatibility(server_ip, port):
    """Test HTTP/1.0 compatibility."""
    section_header("TEST: HTTP/1.0 Compatibility")

    request = build_http_request("GET", "/", http_version="1.0")

    for attempt in range(RETRY_COUNT + 1):
        if attempt > 0:
            info_msg(f"Retry {attempt}/{RETRY_COUNT}...")
            time.sleep(DELAY_BETWEEN_REQUESTS)

        response, timing = send_raw_http_request(server_ip, port, request, "HTTP/1.0 GET /")

        if not response:
            fail_msg("No response received")
            results.add("HTTP/1.0 GET /", False, "No response")
            continue

        status_line, status_code, headers, body = parse_http_response(response)

        checks_passed = True
        if status_code == 200:
            pass_msg(f"Status code: {status_code}")
        else:
            fail_msg(f"Status code: {status_code} (expected 200)")
            checks_passed = False

        if body and len(body) > 0:
            pass_msg(f"Received {len(body)} bytes of body")
        else:
            fail_msg("Empty body")
            checks_passed = False

        results.add("HTTP/1.0 GET /", checks_passed, f"Status {status_code}")
        return checks_passed

    return False


# ─── Connection Handling Tests ────────────────────────────────────────────────

def test_multiple_sequential_requests(server_ip, port):
    """Test multiple sequential requests to check connection slot management."""
    section_header("TEST: Multiple Sequential Requests")

    paths = ["/", "/status", "/", "/echo", "/status", "/"]
    success_count = 0
    timings = []

    for i, path in enumerate(paths):
        method = "POST" if path == "/echo" else "GET"
        body = "test" if path == "/echo" else None
        request = build_http_request(method, path, body)

        t0 = time.monotonic()
        response, timing = send_raw_http_request(server_ip, port, request, f"{method} {path}")
        elapsed = (time.monotonic() - t0) * 1000
        timings.append(elapsed)

        if response:
            status_line, status_code, headers, body = parse_http_response(response)
            if status_code == 200:
                success_count += 1
                pass_msg(f"Request {i+1}: {method} {path} -> {status_code} ({elapsed:.0f}ms)")
            else:
                fail_msg(f"Request {i+1}: {method} {path} -> {status_code} ({elapsed:.0f}ms)")
        else:
            fail_msg(f"Request {i+1}: {method} {path} -> No response ({elapsed:.0f}ms)")

        # Delay between requests to allow connection cleanup
        if i < len(paths) - 1:
            time.sleep(DELAY_BETWEEN_REQUESTS)

    # Timing statistics
    if timings:
        sub_header("Timing Statistics")
        info_msg(f"Min: {min(timings):.0f}ms, Max: {max(timings):.0f}ms, "
                 f"Avg: {sum(timings)/len(timings):.0f}ms")

        # Check for increasing latency (sign of connection slot exhaustion)
        if len(timings) >= 3:
            first_half_avg = sum(timings[:3]) / 3
            second_half_avg = sum(timings[3:]) / 3
            if second_half_avg > first_half_avg * 2:
                fail_msg("Latency increased significantly — possible connection slot exhaustion")
                results.add("Multiple Sequential", False, "Latency increase detected")
            else:
                pass_msg("Latency remained stable across requests")
                results.add("Multiple Sequential", success_count == len(paths),
                           f"{success_count}/{len(paths)} succeeded")
        else:
            results.add("Multiple Sequential", success_count == len(paths),
                       f"{success_count}/{len(paths)} succeeded")

    return success_count == len(paths)


def test_connection_close_behavior(server_ip, port):
    """Test that server properly closes connections after response."""
    section_header("TEST: Connection Close Behavior")

    request = build_http_request("GET", "/")

    for attempt in range(RETRY_COUNT + 1):
        if attempt > 0:
            info_msg(f"Retry {attempt}/{RETRY_COUNT}...")
            time.sleep(DELAY_BETWEEN_REQUESTS)

        sock = None
        try:
            sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
            sock.settimeout(TIMEOUT_CONNECT)
            sock.connect((server_ip, port))
            sock.sendall(request)

            # Receive full response
            sock.settimeout(TIMEOUT_DATA)
            response = b""
            while True:
                try:
                    sock.settimeout(TIMEOUT_DATA)
                    chunk = sock.recv(4096)
                    if not chunk:
                        pass_msg("Server closed connection as expected")
                        results.add("Connection Close", True, "Closed properly")
                        return True
                    response += chunk
                except socket.timeout:
                    # After receiving response, wait briefly then check if connection is still open
                    if b"\r\n\r\n" in response:
                        info_msg("Response received, checking if connection closes...")
                        try:
                            sock.settimeout(2)
                            more = sock.recv(1024)
                            if more:
                                response += more
                                info_msg("Server sent additional data")
                            else:
                                pass_msg("Server closed connection after response")
                                results.add("Connection Close", True, "Closed after response")
                                return True
                        except socket.timeout:
                            fail_msg("Server did not close connection within 2s")
                            results.add("Connection Close", False, "Connection not closed")
                            return False
                    else:
                        fail_msg("Timeout before receiving complete response")
                        results.add("Connection Close", False, "Timeout")
                        return False

        except Exception as e:
            fail_msg(f"Error: {type(e).__name__}: {e}")
            results.add("Connection Close", False, str(e))
            return False
        finally:
            if sock:
                try:
                    sock.close()
                except Exception:
                    pass

    return False


# ─── Response Structure Tests ────────────────────────────────────────────────

def test_response_headers(server_ip, port):
    """Test that response headers are properly formatted."""
    section_header("TEST: Response Header Structure")

    request = build_http_request("GET", "/")

    for attempt in range(RETRY_COUNT + 1):
        if attempt > 0:
            info_msg(f"Retry {attempt}/{RETRY_COUNT}...")
            time.sleep(DELAY_BETWEEN_REQUESTS)

        response, _ = send_raw_http_request(server_ip, port, request, "GET /")

        if not response:
            fail_msg("No response received")
            results.add("Response Headers", False, "No response")
            continue

        status_line, status_code, headers, body = parse_http_response(response)

        checks_passed = True
        required_headers = ["content-type", "content-length"]
        optional_headers = ["server", "connection"]

        sub_header("Required Headers")
        for h in required_headers:
            if h in headers:
                pass_msg(f"{h}: {headers[h]}")
            else:
                fail_msg(f"Missing required header: {h}")
                checks_passed = False

        sub_header("Optional Headers")
        for h in optional_headers:
            if h in headers:
                pass_msg(f"{h}: {headers[h]}")
            else:
                info_msg(f"Missing optional header: {h}")

        # Check CRLF line endings
        raw_headers = response.split(b"\r\n\r\n")[0]
        if b"\r\n" in raw_headers and b"\n\n" not in raw_headers.replace(b"\r\n", b""):
            pass_msg("Headers use CRLF line endings")
        else:
            fail_msg("Headers may not use proper CRLF line endings")
            checks_passed = False

        # Check status line format
        if status_line and status_line.startswith("HTTP/"):
            pass_msg(f"Status line format: {status_line}")
        else:
            fail_msg(f"Invalid status line: {status_line}")
            checks_passed = False

        results.add("Response Headers", checks_passed, f"{len(headers)} headers")
        return checks_passed

    return False


def test_content_length_accuracy(server_ip, port):
    """Test that Content-Length header matches actual body length."""
    section_header("TEST: Content-Length Accuracy")

    test_paths = ["/", "/status"]
    all_passed = True

    for path in test_paths:
        request = build_http_request("GET", path)
        response, _ = send_raw_http_request(server_ip, port, request, f"GET {path}")

        if not response:
            fail_msg(f"{path}: No response")
            all_passed = False
            results.add(f"Content-Length {path}", False, "No response")
            continue

        status_line, status_code, headers, body = parse_http_response(response)

        if "content-length" in headers:
            declared = int(headers["content-length"])
            actual = len(body)
            if declared == actual:
                pass_msg(f"{path}: Content-Length {declared} matches body {actual} bytes")
                results.add(f"Content-Length {path}", True, f"{declared} bytes")
            else:
                fail_msg(f"{path}: Content-Length {declared} but body is {actual} bytes")
                all_passed = False
                results.add(f"Content-Length {path}", False, f"Mismatch: {declared} vs {actual}")
        else:
            info_msg(f"{path}: No Content-Length header")
            results.add(f"Content-Length {path}", True, "No header")

        time.sleep(DELAY_BETWEEN_REQUESTS)

    return all_passed


# ─── Stress Test ─────────────────────────────────────────────────────────────

def test_rapid_requests(server_ip, port, count=5):
    """Send rapid sequential requests to test connection slot management."""
    section_header(f"TEST: Rapid Sequential Requests ({count} requests)")

    success = 0
    failures = 0
    timeouts = 0

    for i in range(count):
        request = build_http_request("GET", "/")
        t0 = time.monotonic()
        response, _ = send_raw_http_request(server_ip, port, request, f"Rapid {i+1}")
        elapsed = (time.monotonic() - t0) * 1000

        if response:
            status_line, status_code, headers, body = parse_http_response(response)
            if status_code == 200:
                success += 1
                info_msg(f"Request {i+1}: 200 OK ({elapsed:.0f}ms)")
            else:
                failures += 1
                fail_msg(f"Request {i+1}: Status {status_code} ({elapsed:.0f}ms)")
        else:
            timeouts += 1
            fail_msg(f"Request {i+1}: No response ({elapsed:.0f}ms)")

        # Small delay between requests
        time.sleep(DELAY_BETWEEN_REQUESTS)

    sub_header("Results")
    info_msg(f"Success: {success}, Failures: {failures}, Timeouts: {timeouts}")

    if success == count:
        pass_msg(f"All {count} requests succeeded")
        results.add("Rapid Requests", True, f"{success}/{count}")
    else:
        fail_msg(f"Only {success}/{count} requests succeeded")
        results.add("Rapid Requests", False, f"{success}/{count}")

    return success == count


# ─── Raw Socket Diagnostic ───────────────────────────────────────────────────

def test_raw_socket_diagnostic(server_ip, port):
    """Low-level diagnostic: show exact bytes sent and received."""
    section_header("DIAGNOSTIC: Raw Socket Byte-Level Analysis")

    request = (
        b"GET / HTTP/1.0\r\n"
        b"Host: test\r\n"
        b"\r\n"
    )

    info_msg(f"Sending {len(request)} bytes via raw socket")
    sub_header("Request Bytes (hex)")
    hex_str = " ".join(f"{b:02x}" for b in request)
    print(f"  {hex_str}")
    sub_header("Request Bytes (text)")
    print(f"  {request.decode('utf-8', errors='replace').replace(chr(10), '\\n' + '  ')}")

    for attempt in range(RETRY_COUNT + 1):
        if attempt > 0:
            info_msg(f"Retry {attempt}/{RETRY_COUNT}...")
            time.sleep(DELAY_BETWEEN_REQUESTS)

        sock = None
        try:
            sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
            sock.settimeout(TIMEOUT_CONNECT)
            t0 = time.monotonic()
            sock.connect((server_ip, port))
            info_msg(f"TCP handshake: {(time.monotonic() - t0) * 1000:.0f}ms")

            sock.sendall(request)
            info_msg("Request sent, waiting for response...")

            sock.settimeout(TIMEOUT_DATA)
            response = b""
            while True:
                try:
                    sock.settimeout(TIMEOUT_DATA)
                    chunk = sock.recv(4096)
                    if not chunk:
                        info_msg("Server closed connection")
                        break
                    response += chunk
                    info_msg(f"Received {len(chunk)} bytes (total: {len(response)})")
                except socket.timeout:
                    info_msg("Socket timeout")
                    break

            if response:
                sub_header("Response Bytes (hex, first 200)")
                hex_str = " ".join(f"{b:02x}" for b in response[:200])
                print(f"  {hex_str}")

                sub_header("Response Bytes (text)")
                text = response.decode("utf-8", errors="replace")
                for line in text.split("\n"):
                    print(f"  {line}")

                status_line, status_code, headers, body = parse_http_response(response)
                if status_code:
                    pass_msg(f"Raw socket test: Status {status_code}")
                    results.add("Raw Socket Diagnostic", True, f"Status {status_code}")
                    return True
                else:
                    fail_msg("Raw socket test: Could not parse HTTP response")
                    results.add("Raw Socket Diagnostic", False, "Parse failed")
                    return False
            else:
                fail_msg("No response received")
                results.add("Raw Socket Diagnostic", False, "No response")
                return False

        except Exception as e:
            fail_msg(f"Error: {type(e).__name__}: {e}")
            results.add("Raw Socket Diagnostic", False, str(e))
            return False
        finally:
            if sock:
                try:
                    sock.close()
                except Exception:
                    pass

    return False


# ─── Main ─────────────────────────────────────────────────────────────────────

def print_summary():
    """Print test summary."""
    section_header("TEST SUMMARY")

    total = results.passed + results.failed
    info_msg(f"Total tests: {total}")
    pass_msg(f"Passed: {results.passed}")
    if results.failed:
        fail_msg(f"Failed: {results.failed}")
    if results.skipped:
        info_msg(f"Skipped: {results.skipped}")

    print("\nDetailed Results:")
    for name, passed, msg in results.details:
        if passed is True:
            status = "✓ PASS"
        elif passed is False:
            status = "✗ FAIL"
        else:
            status = "⊘ SKIP"
        if msg:
            print(f"  {status}: {name} — {msg}")
        else:
            print(f"  {status}: {name}")

    if results.failed == 0 and total > 0:
        print("\n✓ All tests passed!")
    elif results.failed > 0:
        print(f"\n✗ {results.failed} test(s) failed.")
        print("\nTroubleshooting tips:")
        print("  1. Ensure the VM is running and network is configured (host-only or NAT)")
        print("  2. Check that port 8080 is accessible from the host")
        print("  3. Verify the server IP address is correct")
        print("  4. Check VM serial/console output for HTTPD debug logs")
    else:
        print("\n⊘ No tests were executed.")


def main():
    if len(sys.argv) < 2:
        print("Usage: python test_http_server.py <server_ip> [port]")
        print("Example: python test_http_server.py 10.0.2.4")
        print("         python test_http_server.py 192.168.56.101 8080")
        print()
        print("Comprehensive HTTP server test suite for HPVMx.")
        print("Tests endpoints, methods, response structure, and connection handling.")
        sys.exit(1)

    server_ip = sys.argv[1]
    port = int(sys.argv[2]) if len(sys.argv) > 2 else DEFAULT_PORT

    print("=" * 70)
    print("  HPVMx HTTP Server Comprehensive Test Suite v2.0")
    print("=" * 70)
    print(f"  Target: {server_ip}:{port}")
    print(f"  Time: {time.strftime('%Y-%m-%d %H:%M:%S')}")
    print(f"  Connect timeout: {TIMEOUT_CONNECT}s, Data timeout: {TIMEOUT_DATA}s")
    print(f"  Retries per test: {RETRY_COUNT}")

    # ─── Phase 1: Diagnostics ─────────────────────────────────────────────
    section_header("PHASE 1: Network Diagnostics")

    if not test_tcp_connectivity(server_ip, port):
        fail_msg("Cannot connect to server. Aborting remaining tests.")
        results.skip("All endpoint tests", "TCP connectivity failed")
        print_summary()
        sys.exit(1)

    test_tcp_reset_detection(server_ip, port)

    # ─── Phase 2: Endpoint Tests ──────────────────────────────────────────
    section_header("PHASE 2: Endpoint Tests")

    test_get_root(server_ip, port)
    time.sleep(DELAY_BETWEEN_REQUESTS)

    test_get_status(server_ip, port)
    time.sleep(DELAY_BETWEEN_REQUESTS)

    test_post_echo(server_ip, port)
    time.sleep(DELAY_BETWEEN_REQUESTS)

    test_patch_config(server_ip, port)
    time.sleep(DELAY_BETWEEN_REQUESTS)

    test_404_not_found(server_ip, port)
    time.sleep(DELAY_BETWEEN_REQUESTS)

    test_405_method_not_allowed(server_ip, port)
    time.sleep(DELAY_BETWEEN_REQUESTS)

    test_http10_compatibility(server_ip, port)
    time.sleep(DELAY_BETWEEN_REQUESTS)

    # ─── Phase 3: Connection Handling ─────────────────────────────────────
    section_header("PHASE 3: Connection Handling")

    test_connection_close_behavior(server_ip, port)
    time.sleep(DELAY_BETWEEN_REQUESTS)

    test_multiple_sequential_requests(server_ip, port)
    time.sleep(DELAY_BETWEEN_REQUESTS)

    test_rapid_requests(server_ip, port, count=5)
    time.sleep(DELAY_BETWEEN_REQUESTS)

    # ─── Phase 4: Response Structure ──────────────────────────────────────
    section_header("PHASE 4: Response Structure")

    test_response_headers(server_ip, port)
    time.sleep(DELAY_BETWEEN_REQUESTS)

    test_content_length_accuracy(server_ip, port)
    time.sleep(DELAY_BETWEEN_REQUESTS)

    # ─── Phase 5: Low-Level Diagnostic ────────────────────────────────────
    section_header("PHASE 5: Low-Level Diagnostic")

    test_raw_socket_diagnostic(server_ip, port)

    # ─── Summary ──────────────────────────────────────────────────────────
    print_summary()

    sys.exit(0 if results.failed == 0 else 1)


if __name__ == "__main__":
    main()
