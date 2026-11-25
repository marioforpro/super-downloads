#!/usr/bin/env bash
# stage-gpl-sources.sh <binaries_dir> <out_dir> — stage the corresponding source
# for EVERY bundled GPL ffmpeg/ffprobe, both architectures (GPLv3 §6, SD-03).
#
# For each arch the version comes from the binary's own `-version` output at
# release time. Staged into <out_dir> (one path per line on stdout):
#   arm64   : the kept source set in <binaries_dir>/ffmpeg-arm64-sources-*
#             (FFmpeg + x264 + LAME + dav1d) as ffmpeg-arm64-source-<file>
#   x86_64  : the ffmpeg.org release tarball for that version, fetched, as
#             ffmpeg-x86_64-source-ffmpeg-<ver>.tar.xz
#   both    : ffmpeg-<arch>-source-build-config.txt (the `-version` output, which
#             carries the build configuration)
# The gate: exits 1 if a GPL binary ends up without its source asset or if the
# version cannot be determined / ffmpeg and ffprobe disagree.
# SD_FETCH overrides the downloader (called as: $SD_FETCH <url> <dest>); tests use it.
set -euo pipefail

BIN="${1:?usage: stage-gpl-sources.sh <binaries_dir> <out_dir>}"
OUT="${2:?usage: stage-gpl-sources.sh <binaries_dir> <out_dir>}"

fetch() {
  if [[ -n "${SD_FETCH:-}" ]]; then "$SD_FETCH" "$1" "$2"; else curl -fsSL -o "$2" "$1"; fi
}
die() { echo "FAIL: $*" >&2; exit 1; }

rm -rf "$OUT" && mkdir -p "$OUT"

# arch label|target triple
for spec in "arm64|aarch64-apple-darwin" "x86_64|x86_64-apple-darwin"; do
  label="${spec%%|*}"; triple="${spec##*|}"
  gpl=0; version=""; config=""
  for tool in ffmpeg ffprobe; do
    bin="$BIN/${tool}-${triple}"
    [[ -x "$bin" ]] || die "missing bundled binary $bin"
    info="$("$bin" -hide_banner -version 2>/dev/null)" || die "$bin -version failed"
    if grep -q -- --enable-gpl <<<"$info"; then gpl=1; fi
    v="$(head -1 <<<"$info" | sed -nE 's/^[a-z]+ version n?([0-9]+\.[0-9]+(\.[0-9]+)?).*/\1/p')"
    if [[ -z "$v" ]]; then
      [[ "$gpl" == 1 ]] && die "cannot read the ${label} ${tool} version"
      continue
    fi
    [[ -z "$version" || "$version" == "$v" ]] || die "${label} ffmpeg ($version) and ffprobe ($v) versions differ"
    version="$v"
    config+="# ${tool}-${triple} -version"$'\n'"${info}"$'\n\n'
  done
  if [[ "$gpl" != 1 ]]; then
    echo "  ${label}: not a GPL build, no source needed" >&2
    continue
  fi
  [[ -n "$version" ]] || die "cannot read the ${label} ffmpeg version"

  if [[ "$label" == "arm64" ]]; then
    src_dir="$(ls -d "$BIN"/ffmpeg-arm64-sources-* 2>/dev/null | sort | tail -1 || true)"
    [[ -n "$src_dir" ]] || die "arm64 ffmpeg is GPL but no ${BIN}/ffmpeg-arm64-sources-* to attach"
    for f in "$src_dir"/*; do
      cp -c "$f" "$OUT/ffmpeg-arm64-source-$(basename "$f")" 2>/dev/null \
        || cp "$f" "$OUT/ffmpeg-arm64-source-$(basename "$f")"
    done
    core="$OUT/ffmpeg-arm64-source-ffmpeg.tar.xz"
  else
    core="$OUT/ffmpeg-${label}-source-ffmpeg-${version}.tar.xz"
    echo "  ${label}: fetching FFmpeg ${version} source" >&2
    fetch "https://ffmpeg.org/releases/ffmpeg-${version}.tar.xz" "$core.part" \
      || die "could not fetch FFmpeg ${version} source for ${label}"
    mv "$core.part" "$core"
  fi
  printf '%s' "$config" > "$OUT/ffmpeg-${label}-source-build-config.txt"

  # Gate: the source asset and the config must exist and be non-empty.
  [[ -s "$core" ]] || die "GPL ${label} ffmpeg has no source asset (${core##*/})"
  [[ -s "$OUT/ffmpeg-${label}-source-build-config.txt" ]] || die "GPL ${label} has no build config asset"
done

ls -d "$OUT"/* 2>/dev/null || true
