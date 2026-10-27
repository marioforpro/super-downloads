# Third-Party Notices

This file records the third-party binaries bundled with Super Downloads, what was verified about their build and license identity, and how they were checked. Created 2026-08-03 under LifeOS audit finding **SD-F-008** (the app had shipped these binaries in public DMGs since March with zero license attribution).

This document is a **factual record of current posture**, not a legal opinion. Where evidence was incomplete, that is stated explicitly rather than assumed.

---

## FFmpeg

**What it is:** A multimedia framework used by the app to transcode downloaded video into Premiere Pro–compatible H.264/AAC/MP4. Declared as a Tauri `externalBin` in `src-tauri/tauri.conf.json` (`binaries/ffmpeg`) and vendored at `src-tauri/binaries.nosync/ffmpeg-{aarch64,x86_64}-apple-darwin`.
Upstream project: https://ffmpeg.org/

**Verified build identity — the two architecture variants are NOT the same build:**

| Arch | Version | Origin string | Config highlights (from `-version` / `-L`) |
|---|---|---|---|
| `aarch64-apple-darwin` | 7.1.5 | rebuilt 2026-10-27 from the official FFmpeg tarball (`scripts/build-ffmpeg-arm64.sh`; Apple clang 21) | `--enable-gpl --enable-version3 --enable-libx264 --enable-libmp3lame --enable-libdav1d --enable-videotoolbox --enable-audiotoolbox` (static, arm64, min macOS 12.0; **no** `--enable-nonfree`) |
| `x86_64-apple-darwin` | 6.1.1-tessus | `https://evermeet.cx/ffmpeg/` (Apple clang 11.0.0) | `--enable-gpl --enable-version3` — **no** `--enable-nonfree` |

**License identity as verified — divergent per architecture:**

- **aarch64 build:** rebuilt 2026-10-27 as FFmpeg 7.1.5 (official tarball from ffmpeg.org, GPG signature verified) with `--enable-gpl --enable-version3` and **without** `--enable-nonfree`. `ffmpeg-aarch64-apple-darwin -L` prints the GPL v3-or-later text and no nonfree warning. Statically linked components: x264 (stable branch, commit b35605ac, GPLv2+), LAME 3.100 (LGPL), dav1d 1.5.4 (BSD-2-Clause); Apple VideoToolbox/AudioToolbox are system frameworks. Effective license: **GPL v3 or later**. This replaces the earlier 6.0 build that was compiled with `--enable-nonfree`.
  **Corresponding source (GPL):** the exact source tarballs used for this build (FFmpeg 7.1.5, x264, LAME 3.100, dav1d 1.5.4) are kept and available on request from support@superdownloads.app.
- **x86_64 build:** compiled with `--enable-gpl` and `--enable-version3`, without `--enable-nonfree`. `-L` output prints GPL v3 license text (no nonfree warning). Effective license for this build: **GPL v3 or later**.

**Provenance of each claim:** both rows verified directly by executing the vendored binaries with `-version` and `-L` on 2026-08-03 (both archs run natively on the audit machine; no `strings`/quarantine fallback was needed). The x86_64 binary self-identifies its origin (evermeet.cx / "tessus" build tag) in its own version string; the aarch64 binary does not self-identify a build origin beyond its build path and compiler.

**Canonical upstream pointers (no license text pasted here):**
- https://www.ffmpeg.org/legal.html
- https://www.gnu.org/licenses/old-licenses/lgpl-2.1.html (baseline FFmpeg license without `--enable-gpl`/`--enable-version3` — not the mode either vendored build uses)
- https://www.gnu.org/licenses/gpl-3.0.html (applies to both builds, per `--enable-version3` and its own `-L` output)

## FFprobe

**What it is:** FFmpeg's companion media-inspection tool, used to read metadata/duration/codec info from downloaded video. Declared as a Tauri `externalBin` (`binaries/ffprobe`), vendored at `src-tauri/binaries.nosync/ffprobe-{aarch64,x86_64}-apple-darwin`. Ships from the same FFmpeg project and source tree as the ffmpeg binaries above.
Upstream project: https://ffmpeg.org/ffprobe.html

**Verified build identity:** identical lineage split to ffmpeg above — confirmed by running `-version` on both archs.

| Arch | Version | Origin string | Config highlights |
|---|---|---|---|
| `aarch64-apple-darwin` | 7.1.5 | rebuilt 2026-10-27 (same build as ffmpeg above) | `--enable-gpl --enable-version3 --enable-libx264 --enable-libmp3lame --enable-libdav1d --enable-videotoolbox --enable-audiotoolbox` (static, arm64, min macOS 12.0; **no** `--enable-nonfree`) |
| `x86_64-apple-darwin` | 6.1.1-tessus | `https://evermeet.cx/ffmpeg/` (Apple clang 11.0.0) | `--enable-gpl --enable-version3`, no `--enable-nonfree` |

**License identity as verified:** the aarch64 `ffprobe` is the same 7.1.5 GPL build as ffmpeg above (rebuilt 2026-10-27, no `--enable-nonfree`); its `-L` prints GPL v3-or-later text. The x86_64 `ffprobe -L` also prints GPL v3 text, no nonfree warning (GPL v3 or later). Corresponding source for the aarch64 build (FFmpeg 7.1.5, x264, LAME 3.100, dav1d 1.5.4) is kept and available on request from support@superdownloads.app.

**Canonical upstream pointers:** same as FFmpeg above (ffprobe ships under the same license terms as the ffmpeg binary from the same build).

## yt-dlp

**What it is:** A command-line media downloader/extractor; the core engine behind the app's download functionality. Declared as a Tauri `externalBin` (`binaries/yt-dlp`), vendored at `src-tauri/binaries.nosync/yt-dlp-{aarch64,x86_64}-apple-darwin`.
Upstream project: https://github.com/yt-dlp/yt-dlp

**Verified build identity:** both architecture binaries report version `2026.07.04` via `--version`, and are byte-identical in size (38,256,544 bytes each), consistent with a single cross-platform build rather than per-arch native compiles.

**License identity:** yt-dlp is distributed by its upstream project under **The Unlicense** (public-domain equivalent). This is stated per the upstream project's own LICENSE file pointer below — no license text was extracted from the vendored binary itself, and none is pasted here.

**Canonical upstream pointer:** https://github.com/yt-dlp/yt-dlp/blob/master/LICENSE

---

## Provenance — how the binaries were obtained

No acquisition record was found in-repo:

- `src-tauri/binaries.nosync/` and `src-tauri/binaries/` are both **gitignored** (`.gitignore` lines 27–31) — these binaries have never been committed and have no git history to inspect (`git log --oneline --all -- src-tauri/binaries.nosync src-tauri/binaries` returns nothing).
- No download/fetch script for ffmpeg, ffprobe, or yt-dlp exists under `scripts/` (checked `check-release-artifacts.sh`, `make-release.sh`, `platform-health-check.sh`, `platform-health-notify.sh` — none fetch or install these binaries; `platform-health-check.sh` only reads a *separately* self-updated yt-dlp copy under `~/Library/Application Support/com.supermac.super-downloads/bin/yt-dlp`, not the bundled one).
- `docs/DECISIONS.md` and both README files contain no record of where/when the vendored binaries were downloaded.
- The binaries' own version strings are the only surviving provenance: the x86_64 ffmpeg/ffprobe self-identify as the evermeet.cx ("tessus") public build; the aarch64 ffmpeg/ffprobe do not self-identify a distributor, only a build path (`/Volumes/tempdisk/sw`) and toolchain (Apple clang 13.1.6) consistent with a CI/ephemeral build environment.

**Conclusion: acquisition source is not recorded in-repo for any of the three tools; lineage beyond each binary's own version string was not assessed further (e.g. no checksum comparison against known public builds was performed).**

## Existing legal/docs surface checked

- `web/src/pages/terms.astro`, `web/src/pages/copyright.astro`, `web/src/pages/privacy.astro` (Astro source files, not built output) were checked for third-party component mentions.
  - `privacy.astro` §5 mentions **yt-dlp** by name ("the open-source download component runs locally on your computer... the App downloads its yt-dlp component from GitHub") but does not name ffmpeg/ffprobe, does not name a license (GPL/LGPL/Unlicense), and does not link to this notice.
  - `terms.astro` and `copyright.astro` contain no mention of ffmpeg, yt-dlp, or GPL/LGPL/open-source licensing.
- None of the three legal pages currently link to or reference this THIRD-PARTY-NOTICES.md file.

## Known gaps / next steps

1. **This file covers the repository only.** The *distributed* artifacts — the public DMG bundle and the live website legal pages — do not yet surface these notices. Carrying this attribution into the shipped bundle and into `privacy.astro`/`terms.astro`/`copyright.astro` is deferred to the next release, alongside the signing/notarization decision (**SD-F-009**, founder-open).
2. ~~The aarch64 ffmpeg/ffprobe build was compiled with `--enable-nonfree`~~ — resolved 2026-10-27 by the GPL rebuild (FFmpeg 7.1.5, no nonfree); takes effect for users in the first release that ships it.
3. **LGPL §6 / GPL relink and object-availability obligations** for the shipped FFmpeg builds have not been assessed — this requires legal review before further commercial distribution, for both GPLv3 builds (x86_64 and aarch64).
4. **No legal advice.** This document records current posture and verified facts only (build configuration strings, binary self-reported license statements, and what does/doesn't exist in-repo). It does not constitute a compliance determination.
