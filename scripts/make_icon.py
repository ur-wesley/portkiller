import struct
import zlib
from pathlib import Path

w = h = 512
raw = b"".join(b"\x00" + bytes([26, 26, 26]) * w for _ in range(h))
comp = zlib.compress(raw, 9)

def chunk(tag: bytes, data: bytes) -> bytes:
    crc = zlib.crc32(tag + data) & 0xFFFFFFFF
    return struct.pack(">I", len(data)) + tag + data + struct.pack(">I", crc)

ihdr = struct.pack(">IIBBBBB", w, h, 8, 2, 0, 0, 0)
png = b"\x89PNG\r\n\x1a\n" + chunk(b"IHDR", ihdr) + chunk(b"IDAT", comp) + chunk(b"IEND", b"")
Path(__file__).resolve().parents[1].joinpath("app-icon.png").write_bytes(png)
print("wrote app-icon.png")
