// Behaviour tests for the download-lifecycle guards in src/main.js
// (Update confirm, Remove from List). Functions are extracted from the source
// and run in a vm sandbox with stubbed globals.
import test from "node:test";
import assert from "node:assert/strict";
import vm from "node:vm";
import { readFileSync } from "node:fs";

const src = readFileSync(new URL("../src/main.js", import.meta.url), "utf8");

function extract(name) {
  const re = new RegExp(`^(async )?function ${name}\\(`, "m");
  const m = re.exec(src);
  assert.ok(m, `function ${name} not found`);
  let i = src.indexOf("{", m.index);
  let depth = 0;
  for (let j = i; j < src.length; j++) {
    if (src[j] === "{") depth++;
    else if (src[j] === "}" && --depth === 0) return src.slice(m.index, j + 1);
  }
  throw new Error("unbalanced");
}

function sandbox({ rows, confirmAnswer = true, cancelResult = true }) {
  const calls = [];
  const ctx = {
    calls,
    window: { __TAURI__: { core: { invoke: async (c) => { calls.push(`invoke:${c}`); } } } },
    localStorage: { setItem() {} },
    document: { getElementById: () => null },
    sdPendingUpdate: null,
    showToast() {},
    showConfirm: async (m) => { calls.push(`confirm:${m}`); return confirmAnswer; },
    cancelDownload: async (id) => { calls.push(`cancel:${id}`); return cancelResult; },
    queueThumbnailCleanup() {},
    transferStats: { delete() {} },
    progressUiState: { delete() {} },
    stopConversionProgress() {},
    renderDownloadList() { calls.push("render"); },
    scheduleSaveDownloads() {},
    console,
  };
  vm.createContext(ctx);
  const code = [
    `let downloads = ${JSON.stringify(rows)}; let sdUpdateInProgress = false;`,
    ...["isTerminalStatus", "activeDownloadCount", "updateConfirmMessage", "startUpdate", "removeDownload"].map(extract),
    "this.fns = { startUpdate, removeDownload, updateConfirmMessage, activeDownloadCount, rows: () => downloads };",
  ].join("\n");
  vm.runInContext(code, ctx);
  return ctx;
}

test("update with running downloads asks first and aborts on decline", async () => {
  const ctx = sandbox({ rows: [{ id: "a", status: "downloading" }, { id: "b", status: "completed" }], confirmAnswer: false });
  await ctx.fns.startUpdate();
  assert.ok(ctx.calls[0].startsWith("confirm:1 download in progress will be cancelled"));
  assert.ok(!ctx.calls.includes("invoke:install_update"));
});

test("update proceeds after confirmation", async () => {
  const ctx = sandbox({ rows: [{ id: "a", status: "queued" }, { id: "b", status: "merging" }] });
  await ctx.fns.startUpdate();
  assert.ok(ctx.calls[0].startsWith("confirm:2 downloads in progress"));
  assert.ok(ctx.calls.includes("invoke:install_update"));
});

test("update with nothing running does not ask", async () => {
  const ctx = sandbox({ rows: [{ id: "b", status: "error" }] });
  await ctx.fns.startUpdate();
  assert.deepEqual(ctx.calls, ["invoke:install_update"]);
});

test("Remove from List cancels a running download first", async () => {
  const ctx = sandbox({ rows: [{ id: "a", status: "downloading" }] });
  await ctx.fns.removeDownload("a");
  assert.equal(ctx.calls[0], "cancel:a");
  assert.equal(ctx.fns.rows().length, 0);
});

test("Remove from List keeps the row when the cancel failed", async () => {
  const ctx = sandbox({ rows: [{ id: "a", status: "downloading" }], cancelResult: false });
  await ctx.fns.removeDownload("a");
  assert.equal(ctx.fns.rows().length, 1);
});

test("Remove from List on a finished row does not cancel", async () => {
  const ctx = sandbox({ rows: [{ id: "a", status: "completed" }] });
  await ctx.fns.removeDownload("a");
  assert.ok(!ctx.calls.some((c) => c.startsWith("cancel:")));
  assert.equal(ctx.fns.rows().length, 0);
});
