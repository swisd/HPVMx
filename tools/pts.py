import socket


def format_hex_dump(data: bytes, bytes_per_group: int = 1, groups_per_line: int = 16) -> str:
    """Formats raw bytes into a traditional hex dump with 2-byte grouping."""
    bytes_per_line = bytes_per_group * groups_per_line
    lines = []

    for i in range(0, len(data), bytes_per_line):
        chunk = data[i:i + bytes_per_line]

        # Format Hex representation grouped by 2-bytes (4 hex characters)
        hex_groups = []
        for j in range(0, len(chunk), bytes_per_group):
            group = chunk[j:j + bytes_per_group]
            hex_groups.append(group.hex().upper())

        # Pad the hex portion for inconsistent/short terminal lines
        hex_str = " ".join(hex_groups)
        expected_width = (groups_per_line * (bytes_per_group * 2)) + (groups_per_line - 1)
        hex_str = hex_str.ljust(expected_width)

        # Format readable ASCII sidebar representation
        ascii_chars = []
        for b in chunk:
            # Keep printable ASCII characters, replace control characters/binary with a dot
            if 32 <= b <= 126:
                ascii_chars.append(chr(b))
            else:
                ascii_chars.append(".")
        ascii_str = "".join(ascii_chars)

        # Display memory offset | Hex grouped dump | Readable ASCII characters
        lines.append(f"{i:04X}  {hex_str}  |{ascii_str}|")

    return "\n".join(lines)


def start_server(host: str = '0.0.0.0', port: int = 8080):
    # Set up a generic TCP Socket Server
    server_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    server_socket.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
    server_socket.bind((host, port))
    server_socket.listen(5)

    print(f"[*] Raw Request Monitoring Server listening on {host}:{port}")

    try:
        while True:
            client_socket, client_address = server_socket.accept()
            print(f"\n[+] Connection established from {client_address[0]}:{client_address[1]}")

            # Read incoming request data (up to 4KB)
            request_bytes = client_socket.recv(4096)#4096

            if request_bytes:
                print("\n--- [ RAW ASCII REQUEST TRACE ] ---")
                try:
                    # Decode as ASCII, replacing unprintable characters with placeholders
                    print(request_bytes.decode('ascii', errors='replace'))
                except Exception as e:
                    print(f"Error rendering plain text: {e}")

                print("\n--- [ 2-BYTE HEX DUMP SCHEMA ] ---")
                print(format_hex_dump(request_bytes))
                print("-" * 50)

                # Send a basic valid HTTP response just in case a browser or API tool connects
                response = b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\n\r\nOK"
                client_socket.sendall(response)

            client_socket.close()

    except KeyboardInterrupt:
        print("\n[*] Shutting down packet trace server safely.")
    finally:
        server_socket.close()


if __name__ == "__main__":
    start_server()
