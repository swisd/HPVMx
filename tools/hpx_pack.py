#!/usr/bin/env python3
"""Package a flat x86-64 image in the HPX ABI v1 container format."""
import argparse
import struct
from pathlib import Path

HEADER_SIZE = 108


def parse_version_to_u32(version: str) -> int:
    """Parse a version string like '1.2.3' into a packed u32 (0x010203)."""
    parts = version.split(".")
    major = int(parts[0].strip()) if len(parts) > 0 and parts[0].strip() else 0
    minor = int(parts[1].strip()) if len(parts) > 1 and parts[1].strip() else 0
    patch = int(parts[2].strip()) if len(parts) > 2 and parts[2].strip() else 0
    return (major << 16) | (minor << 8) | patch


def parse_project_toml(path: Path) -> dict:
    """Simple TOML parser that extracts name, version, and description from [package]."""
    in_package = False
    name = ""
    version = ""
    description = ""

    for line in path.read_text().splitlines():
        trimmed = line.strip()
        if trimmed.startswith("[") and trimmed.endswith("]"):
            section = trimmed[1:-1].strip()
            in_package = section == "package"
            continue
        if not in_package or not trimmed or trimmed.startswith("#"):
            continue
        if "=" in trimmed:
            key, value = trimmed.split("=", 1)
            key = key.strip()
            value = value.strip()
            # Strip quotes
            if (value.startswith('"') and value.endswith('"')) or \
               (value.startswith("'") and value.endswith("'")):
                value = value[1:-1]
            if key == "name":
                name = value
            elif key == "version":
                version = value
            elif key == "description":
                description = value

    if not name:
        raise SystemExit("project.toml missing [package] name")
    return {"name": name, "version": version, "description": description}


def fill_description(text: str) -> bytes:
    """Return a 64-byte null-terminated description."""
    b = text.encode("utf-8")[:63]
    return b + b"\x00" * (64 - len(b))


parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument("image", type=Path, help="flat, position-independent code/data image")
parser.add_argument("output", type=Path, help="destination .hpx path")
parser.add_argument("--kind", choices=("app", "background"), required=True)
parser.add_argument("--state-size", type=int, default=0)
parser.add_argument("--step", type=lambda x: int(x, 0), required=True)
parser.add_argument("--draw", type=lambda x: int(x, 0), default=0)
parser.add_argument("--input", type=lambda x: int(x, 0), default=0)
parser.add_argument("--width", type=int, default=640)
parser.add_argument("--height", type=int, default=480)
parser.add_argument("--project-toml", type=Path, default=None,
                    help="optional project.toml for name, version, description metadata")
args = parser.parse_args()
image = args.image.read_bytes()
if not image or len(image) > 64 * 1024 * 1024:
    raise SystemExit("image must be between 1 byte and 64 MiB")
if not 0 <= args.state_size <= 16 * 1024 * 1024:
    raise SystemExit("state size must be between 0 and 16 MiB")
for label, offset, optional in (("step", args.step, False), ("draw", args.draw, True), ("input", args.input, True)):
    if optional and offset == 0:
        continue
    if not 0 <= offset < len(image):
        raise SystemExit(f"{label} callback offset must fall inside the image")
if args.kind == "app" and (args.width <= 0 or args.height <= 0):
    raise SystemExit("app dimensions must be positive")
kind = 1 if args.kind == "app" else 2

# Read optional project.toml metadata
file_version = 0
description_bytes = b"\x00" * 64
if args.project_toml is not None:
    meta = parse_project_toml(args.project_toml)
    file_version = parse_version_to_u32(meta["version"])
    description_bytes = fill_description(meta["description"])
    print(f"project.toml: name={meta['name']} version={meta['version']} desc={meta['description']}")

# 108-byte header: magic(4) + abi(2) + kind(2) + 10x u32 (header_size, image_size, state_size,
#   step, draw, input, width, height, file_version) + description[64]
header = struct.pack("<4sHH10I", b"HPX1", 1, kind, HEADER_SIZE, len(image), args.state_size,
                     args.step, args.draw, args.input, args.width, args.height, file_version)
header += description_bytes
args.output.write_bytes(header + image)
