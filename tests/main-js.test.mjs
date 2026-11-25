// Unit tests for small pure-ish helpers of src/main.js, loaded by extracting
// the function source (main.js is a DOM-bound browser module).
import test from "node:test";
import assert from "node:assert/strict";
import vm from "node:vm";
import { readFileSync } from "node:fs";

const src = readFileSync(new URL("../src/main.js", import.meta.url), "utf8");

function extract(name) {
  const m = src.match(new RegExp(`^(async )?function ${name}\\(.*?^}`, "ms"));
  assert.ok(m, `function ${name} not found`);
  return m[0];
}

function load(names, globals) {
  const ctx = vm.createContext(globals);
  vm.runInContext(names.map(extract).join("\n"), ctx);
  return ctx;
}

test("escapeHtml escapes quotes so titles cannot break out of attributes", () => {
  const { escapeHtml } = load(["escapeHtml"], {
    document: {
      createElement: () => {
        const div = { set textContent(t) { this._t = String(t); } };
        Object.defineProperty(div, "innerHTML", {
          get() { return this._t.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;"); },
        });
        return div;
      },
    },
  });
  const out = escapeHtml(`x" onerror="alert(1)' <b>&`);
  assert.ok(!out.includes('"') && !out.includes("'"), out);
  assert.ok(out.includes("&lt;b&gt;&amp;"));
});

function historyGlobals(checked) {
  const calls = [];
  return {
    calls,
    keepHistoryEnabled: { checked },
    window: { __TAURI__: { core: { invoke: async (cmd) => { calls.push(cmd); } } } },
    console,
    clearTimeout() {},
    saveSettings() {},
    scheduleSaveDownloads() { calls.push("schedule"); },
    isHistoryEnabled() { return checked; },
    saveDownloadsTimer: null,
  };
}

test("turning history off deletes the saved history file", async () => {
  const g = historyGlobals(false);
  const ctx = load(["deleteSavedHistory", "onKeepHistoryChange"], g);
  await ctx.onKeepHistoryChange();
  assert.deepEqual(g.calls, ["delete_download_history"]);
});

test("turning history on saves instead of deleting", async () => {
  const g = historyGlobals(true);
  const ctx = load(["deleteSavedHistory", "onKeepHistoryChange"], g);
  await ctx.onKeepHistoryChange();
  assert.deepEqual(g.calls, ["schedule"]);
});

test("Clear list with history off deletes the saved file", async () => {
  const g = {
    ...historyGlobals(false),
    downloads: [{ status: "completed", thumbnail: "" }],
    queueThumbnailCleanup() {},
    transferStats: { clear() {} },
    progressUiState: { clear() {} },
    conversionState: new Map(),
    renderDownloadList() {},
  };
  const ctx = load(["deleteSavedHistory", "clearDownloadList"], g);
  await ctx.clearDownloadList();
  assert.deepEqual(g.calls, ["delete_download_history"]);
});

test("Clear list with history on saves the empty list", async () => {
  const g = {
    ...historyGlobals(true),
    downloads: [{ status: "completed", thumbnail: "" }],
    queueThumbnailCleanup() {},
    transferStats: { clear() {} },
    progressUiState: { clear() {} },
    conversionState: new Map(),
    renderDownloadList() {},
  };
  const ctx = load(["deleteSavedHistory", "clearDownloadList"], g);
  await ctx.clearDownloadList();
  assert.deepEqual(g.calls, ["schedule"]);
});
