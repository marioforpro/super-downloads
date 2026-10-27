#!/usr/bin/env bash
# Rebuild the Apple Silicon ffmpeg + ffprobe bundled in the app (GPL, NO nonfree).
# Output: $T/out/bin/{ffmpeg,ffprobe}. Install step is printed at the end (not automatic).
# Needs: Xcode command line tools, python3, network. No system changes (venv in the temp dir).
set -euo pipefail

FFMPEG_VER=7.1.5
LAME_VER=3.100
DAV1D_VER=1.5.4
X264_COMMIT=b35605ace3ddf7c1a5d67a2eb553f034aef41d55   # videolan/x264 branch "stable"
FFMPEG_KEY=FCF986EA15E6E293A5644F10B4322F04D67658D8      # FFmpeg release signing key

SHA_LAME=ddfe36cab873794038ae2c1210557ad34857a4b6bdc515785d1da9e175b1da1e
SHA_DAV1D=686616b7c69eb88d44459391ab25cac13b6647a3b288835c5784e71c1514a5c5
SHA_FFMPEG=de668509caf9e35e3cd162473441fdb29538c6d96ed080292b3cf9e6fc5d558f

T=${T:-$(mktemp -d)}; P=$T/prefix; mkdir -p "$T/src" "$T/b" "$P"
echo "work dir: $T"
export MACOSX_DEPLOYMENT_TARGET=12.0
F="-arch arm64 -mmacosx-version-min=12.0"
JOBS=$(sysctl -n hw.ncpu)

python3 -m venv "$T/venv"
"$T/venv/bin/pip" -q install meson ninja pkgconf
export PATH=$T/venv/bin:$PATH PKG_CONFIG_PATH=$P/lib/pkgconfig

cd "$T/src"
check() { echo "$2  $1" | shasum -a 256 -c -; }

curl -fsSL -o lame.tgz  "https://downloads.sourceforge.net/project/lame/lame/$LAME_VER/lame-$LAME_VER.tar.gz"
curl -fsSL -o dav1d.tar.xz "https://downloads.videolan.org/pub/videolan/dav1d/$DAV1D_VER/dav1d-$DAV1D_VER.tar.xz"
curl -fsSL -o ffmpeg.tar.xz "https://ffmpeg.org/releases/ffmpeg-$FFMPEG_VER.tar.xz"
curl -fsSL -o ffmpeg.tar.xz.asc "https://ffmpeg.org/releases/ffmpeg-$FFMPEG_VER.tar.xz.asc"
check lame.tgz "$SHA_LAME"; check dav1d.tar.xz "$SHA_DAV1D"; check ffmpeg.tar.xz "$SHA_FFMPEG"
# GPG signature of the FFmpeg tarball (isolated keyring)
export GNUPGHOME=$T/gnupg; mkdir -m700 "$GNUPGHOME"
curl -fsSL https://ffmpeg.org/ffmpeg-devel.asc | gpg --import 2>/dev/null
gpg --verify ffmpeg.tar.xz.asc ffmpeg.tar.xz 2>&1 | grep -q "Good signature" && gpg --verify ffmpeg.tar.xz.asc ffmpeg.tar.xz 2>&1 | grep -q "$FFMPEG_KEY"
git clone -q --branch stable https://code.videolan.org/videolan/x264.git x264
git -C x264 checkout -q "$X264_COMMIT"
tar xf lame.tgz; tar xf dav1d.tar.xz; tar xf ffmpeg.tar.xz

echo "== LAME =="
( cd lame-$LAME_VER && ./configure --prefix="$P" --disable-shared --enable-static --disable-frontend --disable-decoder \
    CFLAGS="$F -O2" LDFLAGS="$F" && make -j"$JOBS" && make install ) >"$T/b/lame.log" 2>&1
echo "== x264 =="
( cd x264 && ./configure --prefix="$P" --enable-static --disable-cli --disable-opencl \
    --extra-cflags="$F" --extra-ldflags="$F" && make -j"$JOBS" && make install ) >"$T/b/x264.log" 2>&1
echo "== dav1d =="
( cd dav1d-$DAV1D_VER && meson setup "$T/b/dav1d" --prefix="$P" --libdir=lib --default-library=static \
    -Denable_tools=false -Denable_tests=false -Dc_args="$F" -Dc_link_args="$F" --buildtype=release \
    && ninja -C "$T/b/dav1d" && ninja -C "$T/b/dav1d" install ) >"$T/b/dav1d.log" 2>&1
echo "== FFmpeg =="
( cd ffmpeg-$FFMPEG_VER && ./configure --prefix="$T/out" --arch=arm64 \
    --enable-gpl --enable-version3 --enable-libx264 --enable-libmp3lame --enable-libdav1d \
    --enable-videotoolbox --enable-audiotoolbox \
    --disable-ffplay --disable-sdl2 --disable-doc --disable-debug \
    --enable-static --disable-shared --pkg-config-flags=--static \
    --extra-cflags="$F -I$P/include" --extra-ldflags="$F -L$P/lib" \
    && make -j"$JOBS" && make install ) >"$T/b/ffmpeg.log" 2>&1

B=$T/out/bin
echo "== verify =="
"$B/ffmpeg" -L 2>&1 | grep -qi nonfree && { echo "FAIL: nonfree"; exit 1; }
"$B/ffmpeg" -version | grep -q -- "--enable-nonfree" && { echo "FAIL: nonfree config"; exit 1; }
if otool -L "$B/ffmpeg" "$B/ffprobe" | grep -E '^\s' | awk '{print $1}' | grep -v -e '^/usr/lib' -e '^/System/Library'; then
  echo "FAIL: non-system dylib"; exit 1; fi
"$B/ffmpeg" -hide_banner -encoders | grep -E ' (h264_videotoolbox|aac|libmp3lame|libx264) '
"$B/ffmpeg" -hide_banner -decoders | grep -E ' (libdav1d|vp9|h264|hevc|opus) '
echo "OK: $B"
cat <<EOF
Install (from src-tauri/; the two dirs are hard links, keep it that way):
  for n in ffmpeg ffprobe; do
    cp $B/\$n binaries.nosync/\$n-aarch64-apple-darwin.new && chmod 700 binaries.nosync/\$n-aarch64-apple-darwin.new
    mv -f binaries.nosync/\$n-aarch64-apple-darwin.new binaries.nosync/\$n-aarch64-apple-darwin
    ln -f binaries.nosync/\$n-aarch64-apple-darwin binaries/\$n-aarch64-apple-darwin
  done
Then: cd src-tauri && cargo test e2e_downloads_live -- --ignored --nocapture
Keep the source tarballs in $T/src (GPL: offered on request).
EOF
