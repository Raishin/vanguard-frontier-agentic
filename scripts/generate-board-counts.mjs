#!/usr/bin/env node
/**
 * Generate per-board catalog counts and inject them into docs/language-stack-boards.md.
 *
 * That document describes each language/stack board with a property table whose
 * agent and skill totals were previously typed by hand, so they went stale
 * silently whenever a board gained, lost, or reclassified an asset. This
 * generator derives every number from the catalog on disk and rewrites the
 * marker spans, exactly as `generate-readme-counts.mjs` does for README.md.
 *
 * Literal numbers are written into the markdown rather than Liquid variables:
 * this file has no Jekyll front matter and is read as raw markdown on GitHub,
 * where `{{ site.data... }}` would render as literal braces.
 *
 * Marker syntax (the number between the markers is what gets replaced):
 *
 *   <!-- count:board:snowflake:agents -->28<!-- /count -->
 *
 * Keys, all derived per provider. Deprecated assets are separated out FIRST and
 * the tier keys then describe only the live board, so the parts sum to the whole
 * without double counting — a deprecated live guard is `deprecated`, not `guards`:
 *
 *   agents      every agent for the provider, deprecated included
 *   deprecated  agents whose metadata sets lifecycle: deprecated
 *   skills      every skill for the provider
 *   router      non-deprecated `*-maestro-agent` agents
 *   static      non-deprecated static-review agents (the router is one of these)
 *   review      static minus router — the review specialists
 *   readonly    non-deprecated read-only-runtime agents
 *   guards      non-deprecated mutating-runtime agents
 *   live        readonly + guards — the live control plane
 *
 * Unknown providers and unknown keys are hard errors: a typo in a marker must
 * fail the gate rather than silently keep a stale number.
 *
 * Mode:
 *   (default)  rewrite docs/language-stack-boards.md in place
 *   --check    compare expected vs actual, exit 1 if stale, 0 if current
 *
 * Run: npm run board-counts:write
 */

import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const docPath = path.join(repoRoot, "docs", "language-stack-boards.md");
const check = process.argv.includes("--check");

// ---------------------------------------------------------------------------
// Collect per-provider stats from the catalog on disk
// ---------------------------------------------------------------------------

/** Read every metadata.json two levels under `root` (e.g. agents/<provider>/<asset>/). */
function readMetadata(root) {
  const out = [];
  const base = path.join(repoRoot, root);
  if (!fs.existsSync(base)) return out;
  for (const provider of fs.readdirSync(base)) {
    const providerDir = path.join(base, provider);
    if (!fs.statSync(providerDir).isDirectory()) continue;
    for (const asset of fs.readdirSync(providerDir)) {
      const metaPath = path.join(providerDir, asset, "metadata.json");
      if (!fs.existsSync(metaPath)) continue;
      out.push(JSON.parse(fs.readFileSync(metaPath, "utf8")));
    }
  }
  return out;
}

const agentMeta = readMetadata("agents");
const skillMeta = readMetadata("skills");

/** @type {Map<string, Record<string, number>>} */
const stats = new Map();

function bucket(provider) {
  if (!stats.has(provider)) {
    stats.set(provider, {
      agents: 0, deprecated: 0, skills: 0,
      router: 0, static: 0, review: 0, readonly: 0, guards: 0, live: 0,
    });
  }
  return stats.get(provider);
}

for (const m of agentMeta) {
  if (!m.provider) continue;
  const s = bucket(m.provider);
  s.agents += 1;

  // Deprecated first, and exclusively: a deprecated live guard is not part of
  // the live board, so counting it under `guards` would both overstate the
  // mutation surface and break the sum against `agents`.
  if (m.lifecycle === "deprecated") {
    s.deprecated += 1;
    continue;
  }

  if (m.execution_tier === "static-review") s.static += 1;
  else if (m.execution_tier === "read-only-runtime") s.readonly += 1;
  else if (m.execution_tier === "mutating-runtime") s.guards += 1;

  // A maestro is itself static-review; `review` is the specialists beside it.
  if (typeof m.id === "string" && m.id.endsWith("-maestro-agent")) s.router += 1;
}

for (const m of skillMeta) {
  if (!m.provider) continue;
  bucket(m.provider).skills += 1;
}

for (const s of stats.values()) {
  s.review = s.static - s.router;
  s.live = s.readonly + s.guards;
}

const VALID_KEYS = new Set([
  "agents", "deprecated", "skills",
  "router", "static", "review", "readonly", "guards", "live",
]);

// ---------------------------------------------------------------------------
// Rewrite the marker spans
// ---------------------------------------------------------------------------

const original = fs.readFileSync(docPath, "utf8");
const markerRe = /<!-- count:board:([a-z0-9-]+):([a-z]+) -->(\d+)<!-- \/count -->/g;

const errors = [];
let seen = 0;

const updated = original.replace(markerRe, (match, provider, key, current) => {
  seen += 1;
  if (!VALID_KEYS.has(key)) {
    errors.push(`unknown key "${key}" in marker count:board:${provider}:${key} — valid keys: ${[...VALID_KEYS].sort().join(", ")}`);
    return match;
  }
  const s = stats.get(provider);
  if (!s) {
    errors.push(`unknown provider "${provider}" in marker count:board:${provider}:${key} — no agents or skills found on disk for it`);
    return match;
  }
  const expected = String(s[key]);
  if (check && expected !== current) {
    errors.push(`stale count:board:${provider}:${key} — file says ${current}, catalog says ${expected}`);
  }
  return `<!-- count:board:${provider}:${key} -->${expected}<!-- /count -->`;
});

if (errors.length) {
  console.error("FAIL: docs/language-stack-boards.md board counts");
  for (const e of errors) console.error(`  - ${e}`);
  if (check) console.error("\nRun `npm run board-counts:write` to refresh.");
  process.exit(1);
}

if (seen === 0) {
  console.error("FAIL: no count:board:* markers found in docs/language-stack-boards.md.");
  console.error("      The generator is wired in but the document no longer carries markers,");
  console.error("      which means its counts are unguarded. Restore the markers.");
  process.exit(1);
}

if (check) {
  console.log(`OK: board counts current (${seen} marker(s) across ${stats.size} provider(s))`);
  process.exit(0);
}

if (updated !== original) {
  fs.writeFileSync(docPath, updated);
  console.log(`OK: docs/language-stack-boards.md updated (${seen} marker(s))`);
} else {
  console.log(`OK: docs/language-stack-boards.md already up to date (${seen} marker(s))`);
}
