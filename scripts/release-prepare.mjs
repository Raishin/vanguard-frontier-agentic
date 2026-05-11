#!/usr/bin/env node
/**
 * Semantic-release prepare-step bridge.
 *
 * Runs AFTER `@semantic-release/npm` writes the bumped version to
 * `package.json`, and BEFORE `@semantic-release/npm` packs+publishes the
 * tarball. Synchronizes every published artifact whose content depends on
 * `package.json.version` or on file hashes that include `package.json`:
 *
 *   1. .claude-plugin/plugin.json         — version-parity (validate:plugin-manifest)
 *   2. .cursor-plugin/plugin.json         — version-parity (validate:multi-harness-marketplace)
 *   3. plugins/vanguard-frontier-agentic/.codex-plugin/plugin.json
 *                                         — version-parity (validate:codex-marketplace)
 *   4. catalog/asset-integrity.json       — includes package.json sha256
 *
 * Without this step the released tarball would ship plugin manifests whose
 * version diverges from `package.json` (breaks every harness's version-parity
 * gate) and an asset-integrity manifest whose package.json hash no longer
 * matches the released tree (breaks downstream attestation verification).
 *
 * Idempotent: re-running on an already-synced tree is a no-op.
 */

import { readFileSync, writeFileSync } from "node:fs";
import { spawnSync } from "node:child_process";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const REPO = dirname(dirname(fileURLToPath(import.meta.url)));
const PKG = JSON.parse(readFileSync(join(REPO, "package.json"), "utf8"));
const NEXT_VERSION = process.argv[2] || PKG.version;

const VERSION_PINNED_PLUGINS = [
  ".claude-plugin/plugin.json",
  ".cursor-plugin/plugin.json",
  "plugins/vanguard-frontier-agentic/.codex-plugin/plugin.json",
];

function syncPluginVersion(relPath) {
  const abs = join(REPO, relPath);
  const data = JSON.parse(readFileSync(abs, "utf8"));
  if (data.version === NEXT_VERSION) {
    return false;
  }
  data.version = NEXT_VERSION;
  writeFileSync(abs, JSON.stringify(data, null, 2) + "\n", "utf8");
  return true;
}

function regenerate(cmd, args) {
  const result = spawnSync(cmd, args, { cwd: REPO, stdio: "inherit" });
  if (result.status !== 0) {
    process.exit(result.status ?? 1);
  }
}

console.log(`[release-prepare] syncing artifacts to version ${NEXT_VERSION}`);

let touched = 0;
for (const rel of VERSION_PINNED_PLUGINS) {
  if (syncPluginVersion(rel)) {
    console.log(`[release-prepare] updated ${rel}`);
    touched += 1;
  }
}

// Re-run the Claude Code + Cursor manifest generators so any other
// catalog-derived fields (agents[] list, etc.) stay in sync alongside
// the version bump.
regenerate("node", ["scripts/generate-plugin-manifest.mjs"]);
regenerate("node", ["scripts/generate-cursor-plugin.mjs"]);

// Regenerate the cross-asset integrity manifest LAST so it covers the
// freshly bumped package.json plus the freshly synchronized plugin
// manifests.
regenerate("python3", ["tests/validate-asset-integrity.py", "--write"]);

console.log(`[release-prepare] done (touched ${touched} plugin manifests)`);
