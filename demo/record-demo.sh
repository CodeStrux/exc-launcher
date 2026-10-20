#!/usr/bin/env bash
# Rebuild exc and re-record the README animation (demo/exc-launcher-demo.gif).
# Run from anywhere; the script moves to the demo/ folder itself.
set -euo pipefail

cd "$(dirname "${BASH_SOURCE[0]}")"

missing=()
for tool in cargo vhs ttyd ffmpeg; do
  command -v "$tool" >/dev/null 2>&1 || missing+=("$tool")
done
if [ "${#missing[@]}" -gt 0 ]; then
  echo "error: missing required tools: ${missing[*]}" >&2
  echo "  vhs (needs ttyd + ffmpeg on PATH): https://github.com/charmbracelet/vhs#installation" >&2
  echo "  macOS:  brew install vhs" >&2
  exit 1
fi

echo "==> Building exc (release)"
(cd .. && cargo build --release --locked)

echo "==> Recording demo.tape"
vhs demo.tape

echo "==> Done"
ls -lh exc-launcher-demo.gif
echo "Tip: GitHub renders GIFs up to ~10 MB inline; shrink with gifsicle -O3 --lossy=80 if needed."
