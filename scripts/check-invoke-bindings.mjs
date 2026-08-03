#!/usr/bin/env node
// Static regression check for Tauri invoke bindings (SD-F-007).
// Verifies every `invoke("cmd", { key })` in src/main.js maps key→snake_case
// onto a real parameter of the matching #[tauri::command] fn in lib.rs.
// Tauri v2 matches JS camelCase args to Rust snake_case params; a key with no
// matching param is silently dropped and the command fails at runtime.

import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const root = join(dirname(fileURLToPath(import.meta.url)), "..");
const mainJs = readFileSync(join(root, "src", "main.js"), "utf8");
const libRs = readFileSync(join(root, "src-tauri", "src", "lib.rs"), "utf8");

const toSnake = (s) => s.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toLowerCase();

// Framework-injected params the frontend never passes.
const INJECTED_TYPE = /AppHandle|WebviewWindow|Window|State\s*<|tauri::/;

// --- Rust side: command name -> set of caller-supplied param names ---
const commands = new Map();
const cmdRe = /#\[tauri::command\]\s*(?:pub\s+)?(?:async\s+)?fn\s+(\w+)\s*\(([^)]*)\)/g;
for (const m of libRs.matchAll(cmdRe)) {
  const [, name, rawParams] = m;
  const params = new Set();
  for (const p of rawParams.split(",")) {
    const idx = p.indexOf(":");
    if (idx === -1) continue;
    const pname = p.slice(0, idx).trim().replace(/^mut\s+/, "");
    const ptype = p.slice(idx + 1).trim();
    if (!pname || INJECTED_TYPE.test(ptype)) continue;
    params.add(pname);
  }
  commands.set(name, params);
}

// --- JS side: each invoke("cmd", { ...keys }) call ---
// Key positions only exist right after "{" or a top-level "," (expectKey);
// everything else at depth 1 is a value expression and is skipped.
function topLevelKeys(objSrc) {
  const keys = [];
  let depth = 0;
  let inStr = null;
  let expectKey = false;
  for (let i = 0; i < objSrc.length; i++) {
    const c = objSrc[i];
    if (inStr) {
      if (c === "\\") i++;
      else if (c === inStr) inStr = null;
      continue;
    }
    if (c === '"' || c === "'" || c === "`") { inStr = c; continue; }
    if (c === "{") { depth++; if (depth === 1) expectKey = true; continue; }
    if (c === "(" || c === "[") { depth++; continue; }
    if (c === "}" || c === ")" || c === "]") { depth--; continue; }
    if (depth === 1 && c === ",") { expectKey = true; continue; }
    if (depth === 1 && expectKey && /\w/.test(c)) {
      const rest = objSrc.slice(i);
      const km = rest.match(/^(\w+)\s*:/);
      const sm = km ? null : rest.match(/^(\w+)\s*[,}]/);
      const id = km?.[1] ?? sm?.[1];
      if (id) { keys.push(id); i += id.length - 1; }
      expectKey = false;
    } else if (depth === 1 && expectKey && !/\s/.test(c)) {
      expectKey = false; // spread, computed key, etc. — not a plain key
    }
  }
  return keys;
}

// Known-missing commands, pinned so the gate stays green while the debt is
// tracked on a lifecycle surface. Anything NEW still fails the check.
// copy_to_clipboard: dead fallback behind navigator.clipboard.writeText —
// recorded 2026-08-03 in ROADMAP.md Backlog; remove or implement there.
const KNOWN_MISSING = new Set(["copy_to_clipboard"]);

const failures = [];
let checked = 0;
const invokeRe = /invoke\(\s*["'](\w+)["']\s*,\s*(\{)/g;
for (const m of mainJs.matchAll(invokeRe)) {
  const cmd = m[1];
  // capture balanced object literal starting at the "{"
  const start = m.index + m[0].length - 1;
  let depth = 0, end = start, inStr = null;
  for (let i = start; i < mainJs.length; i++) {
    const c = mainJs[i];
    if (inStr) {
      if (c === "\\") i++;
      else if (c === inStr) inStr = null;
      continue;
    }
    if (c === '"' || c === "'" || c === "`") inStr = c;
    else if (c === "{") depth++;
    else if (c === "}") { depth--; if (depth === 0) { end = i + 1; break; } }
  }
  const keys = topLevelKeys(mainJs.slice(start, end));
  const params = commands.get(cmd);
  if (!params) {
    if (KNOWN_MISSING.has(cmd)) {
      console.warn(`  ! known-missing command "${cmd}" (tracked in ROADMAP.md Backlog) — skipped`);
    } else {
      failures.push(`invoke("${cmd}"): no #[tauri::command] fn ${cmd} found in lib.rs`);
    }
    continue;
  }
  for (const key of keys) {
    checked++;
    if (!params.has(toSnake(key)) && !params.has(key)) {
      failures.push(
        `invoke("${cmd}"): arg key "${key}" → "${toSnake(key)}" does not match any param of fn ${cmd}(${[...params].join(", ")})`
      );
    }
  }
}

if (failures.length) {
  console.error(`invoke-binding check FAILED (${failures.length} mismatch${failures.length > 1 ? "es" : ""}):`);
  for (const f of failures) console.error("  - " + f);
  process.exit(1);
}
console.log(`invoke-binding check OK — ${checked} arg keys across ${new Set([...mainJs.matchAll(invokeRe)].map(m => m[1])).size} invoke commands verified against lib.rs`);
