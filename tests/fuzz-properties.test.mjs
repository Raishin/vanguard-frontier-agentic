#!/usr/bin/env node
/**
 * Property-based fuzz tests for security-critical logic in the exporter.
 *
 * Uses fast-check to generate adversarial inputs and verify the invariants
 * that guard against path traversal, ID injection, and platform instability.
 *
 * Run: node tests/fuzz-properties.test.mjs
 *
 * Satisfies OpenSSF Scorecard FuzzingID requirement for JS/TS projects.
 * Reference: https://github.com/ossf/scorecard/blob/main/docs/checks.md#fuzzing
 */

import assert from "node:assert/strict";
import path from "node:path";
import fc from "fast-check";

// ── Security-critical implementations under test ─────────────────────────────
// Reproduced verbatim from scripts/export-marketplace-agents.mjs.

function assertWithin(parent, child, label) {
  const resolvedParent = path.resolve(parent);
  const resolvedChild = path.resolve(child);
  const sep = path.sep;
  const parentWithSep = resolvedParent.endsWith(sep)
    ? resolvedParent
    : resolvedParent + sep;
  if (
    resolvedChild !== resolvedParent &&
    !resolvedChild.startsWith(parentWithSep)
  ) {
    throw new Error(
      `Refusing to ${label}: path '${resolvedChild}' escapes '${resolvedParent}'.`
    );
  }
}

const AGENT_ID_PATTERN = /^[a-z0-9][a-z0-9-]*$/;

const HARNESS_PATH_TRAVERSAL =
  /[\\/]\.\.[\\/]|^\.\.[\\/]|[\\/]\.\.$|^\.\.$/;

function normalizePlatform(platform, aliases) {
  const lowered = platform.toLowerCase();
  return aliases[lowered] ?? lowered;
}

const ALIASES = {
  claude: "claude-code",
  kiroide: "kiro-ide",
  kirocli: "kiro-cli",
};

// ── Arbitraries ───────────────────────────────────────────────────────────────

const safeSegment = fc.stringMatching(/^[a-z0-9][a-z0-9-]{0,15}$/);
const safeRelPath = fc
  .array(safeSegment, { minLength: 1, maxLength: 4 })
  .map((segs) => segs.join("/"));

// ── 1. assertWithin ──────────────────────────────────────────────────────────

fc.assert(
  fc.property(
    fc.string({ minLength: 1, maxLength: 50 }),
    (dir) => {
      assert.doesNotThrow(() => assertWithin(dir, dir, "test"));
    }
  ),
  { numRuns: 200 }
);
console.log("PASS  assertWithin: parent is always within itself");

fc.assert(
  fc.property(
    fc.string({ minLength: 1, maxLength: 30 }),
    safeSegment,
    (dir, name) => {
      const child = path.join(dir, name);
      assert.doesNotThrow(() => assertWithin(dir, child, "test"));
    }
  ),
  { numRuns: 200 }
);
console.log("PASS  assertWithin: direct child is always within parent");

fc.assert(
  fc.property(safeSegment, safeSegment, (a, b) => {
    if (a === b) return;
    const parent = `/sandbox/${a}`;
    const outside = `/sandbox/${b}`;
    const resolvedParent = path.resolve(parent);
    const resolvedOutside = path.resolve(outside);
    const sep = path.sep;
    if (
      resolvedOutside !== resolvedParent &&
      !resolvedOutside.startsWith(resolvedParent + sep)
    ) {
      assert.throws(
        () => assertWithin(parent, outside, "read source"),
        /escapes/
      );
    }
  }),
  { numRuns: 200 }
);
console.log("PASS  assertWithin: sibling paths outside parent always throw");

fc.assert(
  fc.property(safeSegment, safeSegment, (parentName, childName) => {
    const parent = path.join("/sandbox", parentName);
    const traversed = path.join(parent, "..", childName);
    if (path.resolve(traversed) === path.resolve(parent)) return;
    assert.throws(
      () => assertWithin(parent, traversed, "read source"),
      /escapes/
    );
  }),
  { numRuns: 200 }
);
console.log("PASS  assertWithin: '..' traversal always escapes and always throws");

// ── 2. Agent ID allowlist ────────────────────────────────────────────────────

fc.assert(
  fc.property(
    fc.string({ minLength: 1, maxLength: 40 }).filter((s) => /[A-Z]/.test(s)),
    (id) => {
      assert.equal(AGENT_ID_PATTERN.test(id), false);
    }
  ),
  { numRuns: 200 }
);
console.log("PASS  agent ID: uppercase letters always rejected");

for (const bad of [
  "../etc/passwd",
  "../../secret",
  "a/b",
  "a\\b",
  "a\x00b",
  "a b",
  ".hidden",
  "a!b",
  "A-good-id",
]) {
  assert.equal(AGENT_ID_PATTERN.test(bad), false, `Expected ${JSON.stringify(bad)} to fail`);
}
console.log("PASS  agent ID: path separators, spaces, dots, control chars always rejected");

fc.assert(
  fc.property(
    fc.stringMatching(/^[a-z0-9][a-z0-9-]{0,40}$/),
    (id) => {
      assert.equal(AGENT_ID_PATTERN.test(id), true);
    }
  ),
  { numRuns: 200 }
);
console.log("PASS  agent ID: valid kebab-case IDs always accepted");

// ── 3. Harness path traversal guard ──────────────────────────────────────────

for (const bad of ["..", "../", "../foo", "foo/../bar", "foo/.."])  {
  assert.equal(HARNESS_PATH_TRAVERSAL.test(bad), true, `Expected traversal in ${JSON.stringify(bad)}`);
}
console.log("PASS  harness path: known traversal patterns detected");

fc.assert(
  fc.property(fc.string({ maxLength: 40 }), (suffix) => {
    assert.equal(HARNESS_PATH_TRAVERSAL.test(`../${suffix}`), true);
  }),
  { numRuns: 200 }
);
console.log("PASS  harness path: paths starting with '../' always flagged");

fc.assert(
  fc.property(safeSegment, safeSegment, (before, after) => {
    assert.equal(HARNESS_PATH_TRAVERSAL.test(`${before}/../${after}`), true);
  }),
  { numRuns: 200 }
);
console.log("PASS  harness path: '/../' in the middle always flagged");

fc.assert(
  fc.property(safeRelPath, (relPath) => {
    assert.equal(HARNESS_PATH_TRAVERSAL.test(relPath), false);
  }),
  { numRuns: 200 }
);
console.log("PASS  harness path: clean relative paths never flagged");

// ── 4. normalizePlatform stability ──────────────────────────────────────────

fc.assert(
  fc.property(fc.string({ maxLength: 50 }), (platform) => {
    const result = normalizePlatform(platform, ALIASES);
    assert.equal(typeof result, "string");
    assert.equal(result, result.toLowerCase());
  }),
  { numRuns: 500 }
);
console.log("PASS  normalizePlatform: never throws, always returns lowercase string");

for (const [alias, canonical] of Object.entries(ALIASES)) {
  assert.equal(normalizePlatform(alias, ALIASES), canonical);
  assert.equal(normalizePlatform(alias.toUpperCase(), ALIASES), canonical);
}
console.log("PASS  normalizePlatform: known aliases resolve to canonical form");

console.log("\nAll fuzz properties passed.");
