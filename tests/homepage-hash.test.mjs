// A URL hash that is not a valid CSS selector (#1, #a?b) made querySelector
// throw and stopped the whole homepage script (Intel/phone detection never ran).
import test from "node:test";
import assert from "node:assert/strict";
import vm from "node:vm";
import { readFileSync } from "node:fs";

const src = readFileSync(new URL("../web/src/pages/index.astro", import.meta.url), "utf8");
const fn = src.match(/function openFaqFromHash\(\) \{.*?^      \}/ms);

test("openFaqFromHash survives an invalid selector hash", () => {
  assert.ok(fn, "openFaqFromHash not found");
  const doc = {
    querySelector(sel) {
      if (!/^#[A-Za-z_-][\w-]*$/.test(sel)) throw new SyntaxError("invalid selector");
      return { tagName: "DETAILS", open: false };
    },
  };
  for (const hash of ["#1", "#download?x=1", "#a b", ""]) {
    const ctx = vm.createContext({ document: doc, location: { hash } });
    vm.runInContext(fn[0] + "\nopenFaqFromHash();", ctx);
  }
});

test("openFaqFromHash still opens a valid FAQ item", () => {
  const el = { tagName: "DETAILS", open: false };
  const ctx = vm.createContext({ document: { querySelector: () => el }, location: { hash: "#faq-macos" } });
  vm.runInContext(fn[0] + "\nopenFaqFromHash();", ctx);
  assert.equal(el.open, true);
});
