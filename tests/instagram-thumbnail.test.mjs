// The Instagram thumbnail cache ran an extra yt-dlp before every download, but
// the cached local file can never be shown: the asset protocol is not enabled
// and the CSP img-src does not allow it. The https thumbnail from the metadata
// run is displayed instead.
import test from "node:test";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";

const lib = readFileSync(new URL("../src-tauri/src/lib.rs", import.meta.url), "utf8");
const conf = JSON.parse(readFileSync(new URL("../src-tauri/tauri.conf.json", import.meta.url), "utf8"));

test("asset protocol is off, so cached local thumbnails cannot display", () => {
  assert.ok(!conf.app?.security?.assetProtocol?.enable);
});

test("no extra yt-dlp thumbnail run before download-started", () => {
  assert.ok(!lib.includes("cache_thumbnail_for_download"));
});
