#!/usr/bin/env python3
"""Package a flat x86-64 image in the HPX ABI v1 container format."""
import argparse
import struct
from pathlib import Path

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
header = struct.pack("<4sHH8I", b"HPX1", 1, kind, 40, len(image), args.state_size,
                     args.step, args.draw, args.input, args.width, args.height)
args.output.write_bytes(header + image)
