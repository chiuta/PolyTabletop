#!/usr/bin/env bash
# Install PolyTabletop from a GitHub release. Usage: install.sh [version-tag] ; env: PREFIX (default ~/.local), REPO.
set -euo pipefail
REPO="${REPO:-chiuta/PolyTabletop}"
PREFIX="${PREFIX:-$HOME/.local}"
tag="${1:-latest}"

case "$(uname -s)-$(uname -m)" in
  Linux-x86_64) plat=linux-x86_64 ;;
  Linux-aarch64|Linux-arm64) plat=linux-aarch64 ;;
  Darwin-arm64) plat=macos-arm64 ;;
  Darwin-x86_64) plat=macos-x86_64 ;;
  *) echo "unsupported platform: $(uname -sm) (on Windows download the .zip)" >&2; exit 1 ;;
esac

if [ "$tag" = latest ]; then
  tag="$(curl -fsSL "https://api.github.com/repos/$REPO/releases/latest" | sed -n 's/.*"tag_name": *"\([^"]*\)".*/\1/p' | head -n1)"
  [ -n "$tag" ] || { echo "cannot determine latest release" >&2; exit 1; }
fi

name="polytabletop-${tag}-${plat}"
tmp="$(mktemp -d)"; trap 'rm -rf "$tmp"' EXIT
base="https://github.com/$REPO/releases/download/$tag"
curl -fsSL -o "$tmp/$name.tar.gz" "$base/$name.tar.gz"
curl -fsSL -o "$tmp/$name.tar.gz.sha256" "$base/$name.tar.gz.sha256"
( cd "$tmp" && if command -v sha256sum >/dev/null; then sha256sum -c "$name.tar.gz.sha256"; else shasum -a 256 -c "$name.tar.gz.sha256"; fi )
tar xzf "$tmp/$name.tar.gz" -C "$tmp"

mkdir -p "$PREFIX/bin" "$PREFIX/share/man/man1"
install -m 755 "$tmp/$name/polytabletop" "$tmp/$name/polytabletop-gui" "$tmp/$name/polytabletop-rpg" "$PREFIX/bin/"
ln -sf polytabletop "$PREFIX/bin/ptt"
install -m 644 "$tmp/$name/man/polytabletop.1" "$PREFIX/share/man/man1/"
echo "Installed polytabletop $tag in $PREFIX/bin (add it to PATH if needed)."
