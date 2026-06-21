#!/usr/bin/env python3
"""Convert flat binary to Verilog $readmemh hex format (32-bit words, little-endian)."""
import sys

def main():
    if len(sys.argv) < 2:
        print(f"Usage: {sys.argv[0]} <input.bin> [output.hex]", file=sys.stderr)
        sys.exit(1)

    with open(sys.argv[1], 'rb') as f:
        data = f.read()

    # Pad to multiple of 4
    while len(data) % 4 != 0:
        data += b'\x00'

    lines = []
    for i in range(0, len(data), 4):
        word = data[i] | (data[i+1] << 8) | (data[i+2] << 16) | (data[i+3] << 24)
        lines.append(f'{word:08x}')

    out = '\n'.join(lines) + '\n'

    if len(sys.argv) >= 3:
        with open(sys.argv[2], 'w') as f:
            f.write(out)
    else:
        print(out, end='')

if __name__ == '__main__':
    main()
