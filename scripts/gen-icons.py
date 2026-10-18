#!/usr/bin/env python3
"""Generate the Mornfold app icon master, tray icon variants and UI logo mark.

The mark: a half sun (filled semicircle) rising above a soft chevron "fold"
line -- "the place where the morning gathers". Original artwork owned by the
Mornfold project.

Everything is produced by a tiny deterministic rasterizer (signed distance
functions + 4x4 supersampling on edge pixels) and a minimal PNG RGBA writer
built on zlib, so re-running the script yields byte-identical files.

Outputs (relative to the repository root):
    src-tauri/icons/app-icon-1024.png            1024x1024 app icon master
    src-tauri/icons/tray/{dark,light}/{normal,unread,paused,error}.png  32x32
    ui/src/assets/mornfold-mark.svg              same mark as SVG paths

Usage (stdlib only, run isolated):
    python3 -I scripts/gen-icons.py [--root PATH]
"""

import argparse
import math
import struct
import sys
import zlib
from pathlib import Path

DEFAULT_ROOT = Path(__file__).resolve().parent.parent

# --------------------------------------------------------------------------
# Mark geometry (design units, 64 x 64 box; y grows downwards)
# --------------------------------------------------------------------------

SUN_CX = 32.0
SUN_CY = 30.0
SUN_R = 15.0
CHEVRON = ((9.0, 39.0), (32.0, 48.0), (55.0, 39.0))
CHEVRON_HALF_W = 2.5  # stroke width 5 with round caps and a round outer join


def _chevron_bottom():
    (ax, ay), (bx, by), _ = CHEVRON
    # Lowest point of the round join at the chevron vertex.
    return by + CHEVRON_HALF_W


def mark_bbox():
    (ax, ay), _, (cx, cy) = CHEVRON
    left = min(SUN_CX - SUN_R, ax - CHEVRON_HALF_W)
    right = max(SUN_CX + SUN_R, cx + CHEVRON_HALF_W)
    top = min(SUN_CY - SUN_R, ay - CHEVRON_HALF_W, cy - CHEVRON_HALF_W)
    bottom = _chevron_bottom()
    return left, top, right, bottom


def _centered_shift():
    """Shift that centres the mark's bounding box in the 64x64 design box."""
    left, top, right, bottom = mark_bbox()
    return 32.0 - (left + right) / 2.0, 32.0 - (top + bottom) / 2.0


SHIFT_X, SHIFT_Y = _centered_shift()


def _sd_segment(px, py, ax, ay, bx, by):
    abx, aby = bx - ax, by - ay
    apx, apy = px - ax, py - ay
    t = (apx * abx + apy * aby) / (abx * abx + aby * aby)
    t = 0.0 if t < 0.0 else (1.0 if t > 1.0 else t)
    dx, dy = apx - abx * t, apy - aby * t
    return math.sqrt(dx * dx + dy * dy)


def mark_sdf(u, v):
    """Signed distance (design units) to the centred mark; negative inside.

    The value never overestimates the true distance, which is all the
    rasterizer needs to decide whether a pixel can skip supersampling.
    """
    u -= SHIFT_X
    v -= SHIFT_Y
    du, dv = u - SUN_CX, v - SUN_CY
    sun = max(math.sqrt(du * du + dv * dv) - SUN_R, v - SUN_CY)
    (ax, ay), (bx, by), (cx, cy) = CHEVRON
    chevron = (
        min(_sd_segment(u, v, ax, ay, bx, by), _sd_segment(u, v, bx, by, cx, cy))
        - CHEVRON_HALF_W
    )
    return min(sun, chevron)


# --------------------------------------------------------------------------
# Rasterizer
# --------------------------------------------------------------------------

SS = 4  # 4x4 supersampling
SS_OFFSETS = tuple((i + 0.5) / SS for i in range(SS))
EDGE_PX = 0.75  # > half a pixel diagonal: beyond this a pixel is fully in/out


def coverage(sdf_px, x, y):
    """Coverage of pixel (x, y) by a shape given as an SDF in pixel space."""
    d = sdf_px(x + 0.5, y + 0.5)
    if d > EDGE_PX:
        return 0.0
    if d < -EDGE_PX:
        return 1.0
    inside = 0
    for oy in SS_OFFSETS:
        for ox in SS_OFFSETS:
            if sdf_px(x + ox, y + oy) <= 0.0:
                inside += 1
    return inside / (SS * SS)


def mark_in_pixels(center_x, center_y, scale):
    """Mark SDF in pixel space: design box centre maps to (center_x, center_y)."""

    def sdf(px, py):
        u = (px - center_x) / scale + 32.0
        v = (py - center_y) / scale + 32.0
        return mark_sdf(u, v) * scale

    return sdf


def disk_in_pixels(cx, cy, r):
    def sdf(px, py):
        dx, dy = px - cx, py - cy
        return math.sqrt(dx * dx + dy * dy) - r

    return sdf


def rounded_square_in_pixels(size, radius):
    half = size / 2.0
    inner = half - radius

    def sdf(px, py):
        qx = abs(px - half) - inner
        qy = abs(py - half) - inner
        ox, oy = max(qx, 0.0), max(qy, 0.0)
        return math.sqrt(ox * ox + oy * oy) + min(max(qx, qy), 0.0) - radius

    return sdf


def hex_rgb(value):
    value = value.lstrip("#")
    return tuple(int(value[i : i + 2], 16) for i in (0, 2, 4))


def to_byte(v):
    v = int(round(v))
    return 0 if v < 0 else (255 if v > 255 else v)


# --------------------------------------------------------------------------
# PNG writer
# --------------------------------------------------------------------------


def _chunk(kind, data):
    body = kind + data
    return struct.pack(">I", len(data)) + body + struct.pack(">I", zlib.crc32(body) & 0xFFFFFFFF)


def write_png(path, width, height, rows):
    """rows: list of bytes objects, each width*4 RGBA bytes."""
    raw = b"".join(b"\x00" + row for row in rows)  # filter type 0 per row
    ihdr = struct.pack(">IIBBBBB", width, height, 8, 6, 0, 0, 0)  # 8-bit RGBA
    png = (
        b"\x89PNG\r\n\x1a\n"
        + _chunk(b"IHDR", ihdr)
        + _chunk(b"IDAT", zlib.compress(raw, 9))
        + _chunk(b"IEND", b"")
    )
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_bytes(png)


# --------------------------------------------------------------------------
# App icon
# --------------------------------------------------------------------------

APP_SIZE = 1024
APP_CORNER = 0.22
APP_TOP = hex_rgb("#1B1A3A")
APP_BOTTOM = hex_rgb("#4C3BCF")
APP_MARK_WIDTH = 0.56  # mark width as a fraction of the canvas width


def render_app_icon():
    size = APP_SIZE
    left, _, right, _ = mark_bbox()
    scale = APP_MARK_WIDTH * size / (right - left)
    mark = mark_in_pixels(size / 2.0, size / 2.0, scale)
    square = rounded_square_in_pixels(size, APP_CORNER * size)
    rows = []
    for y in range(size):
        t = (y + 0.5) / size
        grad = tuple(APP_TOP[c] + (APP_BOTTOM[c] - APP_TOP[c]) * t for c in range(3))
        row = bytearray()
        for x in range(size):
            a = coverage(square, x, y)
            if a == 0.0:
                row += b"\x00\x00\x00\x00"
                continue
            m = coverage(mark, x, y)
            r = grad[0] + (255.0 - grad[0]) * m
            g = grad[1] + (255.0 - grad[1]) * m
            b = grad[2] + (255.0 - grad[2]) * m
            row += bytes((to_byte(r), to_byte(g), to_byte(b), to_byte(a * 255.0)))
        rows.append(bytes(row))
    return size, rows


# --------------------------------------------------------------------------
# Tray icons
# --------------------------------------------------------------------------

TRAY_SIZE = 32
TRAY_MARK_WIDTH_PX = 30.0
TRAY_COLORS = {"dark": hex_rgb("#FFFFFF"), "light": hex_rgb("#1F1F1F")}
UNREAD_DOT = hex_rgb("#8B7CFF")
ERROR_DOT = hex_rgb("#EF4444")
DOT_R = 5.0
DOT_RING = 1.5
DOT_CX = TRAY_SIZE - DOT_R - 0.5
DOT_CY = DOT_R + 0.5
PAUSED_ALPHA = 0.40


def render_tray(color, variant):
    size = TRAY_SIZE
    left, _, right, _ = mark_bbox()
    scale = TRAY_MARK_WIDTH_PX / (right - left)
    mark = mark_in_pixels(size / 2.0, size / 2.0, scale)
    dot_color = {"unread": UNREAD_DOT, "error": ERROR_DOT}.get(variant)
    ring = disk_in_pixels(DOT_CX, DOT_CY, DOT_R + DOT_RING)
    dot = disk_in_pixels(DOT_CX, DOT_CY, DOT_R)
    rows = []
    for y in range(size):
        row = bytearray()
        for x in range(size):
            ma = coverage(mark, x, y)
            if variant == "paused":
                ma *= PAUSED_ALPHA
            if dot_color is None:
                row += bytes((color[0], color[1], color[2], to_byte(ma * 255.0)))
                continue
            # Transparent ring separates the badge dot from the mark.
            ma *= 1.0 - coverage(ring, x, y)
            da = coverage(dot, x, y)
            out_a = da + ma * (1.0 - da)
            if out_a <= 0.0:
                row += b"\x00\x00\x00\x00"
                continue
            rgb = tuple(
                (dot_color[c] * da + color[c] * ma * (1.0 - da)) / out_a for c in range(3)
            )
            row += bytes((to_byte(rgb[0]), to_byte(rgb[1]), to_byte(rgb[2]), to_byte(out_a * 255.0)))
        rows.append(bytes(row))
    return size, rows


# --------------------------------------------------------------------------
# SVG mark
# --------------------------------------------------------------------------


def _fmt(v):
    s = f"{v:.2f}".rstrip("0").rstrip(".")
    return "0" if s in ("-0", "") else s


def svg_mark():
    sx, sy = SHIFT_X, SHIFT_Y
    cx, cy, r = SUN_CX + sx, SUN_CY + sy, SUN_R
    sun = (
        f"M{_fmt(cx - r)} {_fmt(cy)}"
        f"A{_fmt(r)} {_fmt(r)} 0 0 1 {_fmt(cx + r)} {_fmt(cy)}Z"
    )

    (ax, ay), (bx, by), (qx, qy) = ((x + sx, y + sy) for x, y in CHEVRON)
    h = CHEVRON_HALF_W

    def up_normal(x0, y0, x1, y1):
        dx, dy = x1 - x0, y1 - y0
        length = math.hypot(dx, dy)
        ux, uy = dx / length, dy / length
        return uy, -ux  # perpendicular pointing up (negative y)

    n1 = up_normal(ax, ay, bx, by)
    n2 = up_normal(bx, by, qx, qy)
    # Inner (top) corner: intersection of the two upper offset lines.
    p1 = (ax + h * n1[0], ay + h * n1[1])
    d1 = (bx - ax, by - ay)
    p2 = (bx + h * n2[0], by + h * n2[1])
    d2 = (qx - bx, qy - by)
    denom = d1[0] * d2[1] - d1[1] * d2[0]
    t = ((p2[0] - p1[0]) * d2[1] - (p2[1] - p1[1]) * d2[0]) / denom
    inner = (p1[0] + d1[0] * t, p1[1] + d1[1] * t)

    def pt(p):
        return f"{_fmt(p[0])} {_fmt(p[1])}"

    arc = f"A{_fmt(h)} {_fmt(h)} 0 0 1 "
    chevron = (
        f"M{pt((ax + h * n1[0], ay + h * n1[1]))}"
        f"L{pt(inner)}"
        f"L{pt((qx + h * n2[0], qy + h * n2[1]))}"
        f"{arc}{pt((qx - h * n2[0], qy - h * n2[1]))}"
        f"L{pt((bx - h * n2[0], by - h * n2[1]))}"
        f"{arc}{pt((bx - h * n1[0], by - h * n1[1]))}"
        f"L{pt((ax - h * n1[0], ay - h * n1[1]))}"
        f"{arc}{pt((ax + h * n1[0], ay + h * n1[1]))}Z"
    )
    return (
        '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64" '
        'fill="currentColor" role="img" aria-label="Mornfold">\n'
        f'  <path fill="currentColor" d="{sun}"/>\n'
        f'  <path fill="currentColor" d="{chevron}"/>\n'
        "</svg>\n"
    )


# --------------------------------------------------------------------------


def main(argv=None):
    parser = argparse.ArgumentParser(description="Generate Mornfold icons and logo mark.")
    parser.add_argument("--root", type=Path, default=DEFAULT_ROOT, help="repository root")
    args = parser.parse_args(argv)
    root = args.root

    icons = root / "src-tauri" / "icons"
    size, rows = render_app_icon()
    write_png(icons / "app-icon-1024.png", size, size, rows)
    print(f"wrote {icons / 'app-icon-1024.png'}")

    for theme, color in sorted(TRAY_COLORS.items()):
        for variant in ("normal", "unread", "paused", "error"):
            size, rows = render_tray(color, variant)
            out = icons / "tray" / theme / f"{variant}.png"
            write_png(out, size, size, rows)
            print(f"wrote {out}")

    svg_path = root / "ui" / "src" / "assets" / "mornfold-mark.svg"
    svg_path.parent.mkdir(parents=True, exist_ok=True)
    svg_path.write_text(svg_mark(), encoding="utf-8", newline="\n")
    print(f"wrote {svg_path}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
