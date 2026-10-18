#!/usr/bin/env python3
"""Generate (or validate) the Mornfold notification sound.

The sound written by this script is an ORIGINAL synthesis owned by the
Mornfold project (decision D-09). It is built from three decaying sine
partials computed below; it is not a copy, sample or derivative of any
third-party sound (including Discord's notification sound, whose short,
soft "ping" character was only a stylistic reference).

Output: 44100 Hz, mono, 16-bit PCM WAV, 0.45 s, peak normalised to 0.7
(about -3 dBFS). The synthesis is fully deterministic (no randomness), so
re-running the script reproduces the same file.

Usage (stdlib only, run isolated):
    python3 -I scripts/gen-ping.py                 # write the default file
    python3 -I scripts/gen-ping.py --out PATH      # write elsewhere
    python3 -I scripts/gen-ping.py --check PATH    # validate an existing WAV
"""

import argparse
import math
import struct
import sys
import wave
from pathlib import Path

SAMPLE_RATE = 44100
DURATION_S = 0.45
PEAK = 0.7
ATTACK_S = 0.004
FADE_OUT_S = 0.020
GLIDE_S = 0.060
GLIDE_RATIO = 0.015  # +1.5 % upward pitch glide on partial 1

# (frequency Hz, amplitude, decay time constant tau in seconds)
PARTIALS = (
    (1046.5, 0.60, 0.11),  # C6
    (1568.0, 0.30, 0.07),  # G6
    (2093.0, 0.08, 0.04),  # C7
)

# Validation limits for --check
CHECK_MAX_DURATION_S = 1.0
CHECK_MAX_PEAK = 0.75

DEFAULT_OUT = (
    Path(__file__).resolve().parent.parent
    / "src-tauri"
    / "resources"
    / "sounds"
    / "mornfold-ping.wav"
)


def envelope(t, tau):
    """4 ms linear attack followed by exponential decay."""
    attack = t / ATTACK_S if t < ATTACK_S else 1.0
    return attack * math.exp(-t / tau)


def synthesize():
    n = int(round(SAMPLE_RATE * DURATION_S))
    dt = 1.0 / SAMPLE_RATE
    fade_n = int(round(SAMPLE_RATE * FADE_OUT_S))
    phases = [0.0 for _ in PARTIALS]
    samples = []
    for i in range(n):
        t = i * dt
        value = 0.0
        for k, (freq, amp, tau) in enumerate(PARTIALS):
            value += amp * envelope(t, tau) * math.sin(phases[k])
            f = freq
            if k == 0:
                # Slight upward glide over the first 60 ms (phase-continuous).
                f = freq * (1.0 + GLIDE_RATIO * min(t / GLIDE_S, 1.0))
            phases[k] += 2.0 * math.pi * f * dt
        # 20 ms cosine fade-out at the very end.
        remaining = n - 1 - i
        if remaining < fade_n:
            value *= 0.5 - 0.5 * math.cos(math.pi * remaining / fade_n)
        samples.append(value)

    peak = max(abs(s) for s in samples)
    gain = PEAK / peak if peak > 0 else 0.0
    return [s * gain for s in samples]


def write_wav(path, samples):
    path.parent.mkdir(parents=True, exist_ok=True)
    frames = b"".join(
        struct.pack("<h", max(-32767, min(32767, int(round(s * 32767)))))
        for s in samples
    )
    with wave.open(str(path), "wb") as w:
        w.setnchannels(1)
        w.setsampwidth(2)
        w.setframerate(SAMPLE_RATE)
        w.setcomptype("NONE", "not compressed")
        w.writeframes(frames)


def check_wav(path):
    errors = []
    try:
        with wave.open(str(path), "rb") as w:
            comptype = w.getcomptype()
            sampwidth = w.getsampwidth()
            channels = w.getnchannels()
            rate = w.getframerate()
            nframes = w.getnframes()
            frames = w.readframes(nframes)
    except (OSError, EOFError, wave.Error) as exc:
        print(f"FAIL: cannot read {path}: {exc}", file=sys.stderr)
        return 1

    if comptype != "NONE":
        errors.append(f"not PCM (comptype={comptype})")
    if sampwidth != 2:
        errors.append(f"not 16-bit (sampwidth={sampwidth})")
    if channels != 1:
        errors.append(f"not mono (channels={channels})")
    if rate != SAMPLE_RATE:
        errors.append(f"sample rate {rate} != {SAMPLE_RATE}")
    duration = nframes / rate if rate else float("inf")
    if not duration < CHECK_MAX_DURATION_S:
        errors.append(f"duration {duration:.3f}s is not < {CHECK_MAX_DURATION_S}s")
    peak = 0.0
    if sampwidth == 2 and frames:
        count = len(frames) // 2
        values = struct.unpack("<%dh" % count, frames[: count * 2])
        peak = max(abs(v) for v in values) / 32767.0
        if peak > CHECK_MAX_PEAK:
            errors.append(f"peak {peak:.3f} > {CHECK_MAX_PEAK}")
    elif not frames:
        errors.append("no audio frames")

    if errors:
        for e in errors:
            print(f"FAIL: {e}", file=sys.stderr)
        return 1
    print(
        f"OK: {path} PCM 16-bit mono {rate} Hz, {duration:.3f}s, peak {peak:.3f}"
    )
    return 0


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    group = parser.add_mutually_exclusive_group()
    group.add_argument("--out", type=Path, default=DEFAULT_OUT, help="output WAV path")
    group.add_argument("--check", type=Path, metavar="PATH", help="validate a WAV file")
    args = parser.parse_args(argv)

    if args.check is not None:
        return check_wav(args.check)

    write_wav(args.out, synthesize())
    print(f"wrote {args.out}")
    return check_wav(args.out)


if __name__ == "__main__":
    sys.exit(main())
