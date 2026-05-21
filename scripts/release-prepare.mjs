#!/usr/bin/env node
/**
 * Semantic-release prepare-step bridge.
 *
 * Runs AFTER `@semantic-release/npm` writes the bumped version to
 * `package.json`, and BEFORE `@semantic-release/npm` packs+publishes the
 * tarball. Synchronizes every published artifact whose content depends on
 * `package.json.version` or on file hashes that include `package.json`:
 *
 *   1. .claude-plugin/plugin.json              — version-parity (validate:plugin-manifest)
 *   2. .cursor-plugin/plugin.json              — version-parity (validate:multi-harness-marketplace)
 *   3. plugins/vanguard-frontier-agentic/.codex-plugin/plugin.json
 *                                              — version-parity (validate:codex-marketplace)
 *   4. .github/plugin/marketplace.json         — Copilot marketplace version field
 *   5. SECURITY.md                             — "current published version" + supported-version table
 *   6. catalog/asset-integrity.json            — includes package.json sha256
 *
 * Without this step the released tarball would ship plugin manifests whose
 * version diverges from `package.json` (breaks every harness's version-parity
 * gate) and an asset-integrity manifest whose package.json hash no longer
 * matches the released tree (breaks downstream attestation verification).
 *
 * `package.json` is the single source of truth for the release version.
 * No other file should hard-code a version string that this script can derive.
 *
 * Idempotent: re-running on an already-synced tree is a no-op.
 */

import { readFileSync, writeFileSync, readdirSync } from "node:fs";
import { spawnSync } from "node:child_process";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const REPO = dirname(dirname(fileURLToPath(import.meta.url)));
const PKG = JSON.parse(readFileSync(join(REPO, "package.json"), "utf8"));
const NEXT_VERSION = process.argv[2] || PKG.version;

// Defense-in-depth: reject non-semver version strings before they are
// written into plugin JSON files. The primary caller (@semantic-release/exec)
// always supplies a strict semver string, but manual invocations have no
// other guard. JSON.stringify escapes the value safely, so there is no
// injection risk, but a bogus string would silently corrupt plugin manifests.
if (!/^\d+\.\d+\.\d+(-[\w.]+)?(\+[\w.]+)?$/.test(NEXT_VERSION)) {
  console.error(`[release-prepare] Refusing unsafe version string: ${NEXT_VERSION}`);
  process.exit(1);
}

const VERSION_PINNED_PLUGINS = [
  ".claude-plugin/plugin.json",
  ".cursor-plugin/plugin.json",
  "plugins/vanguard-frontier-agentic/.codex-plugin/plugin.json",
  // Copilot marketplace manifest — version field only; content is manually curated.
  ".github/plugin/marketplace.json",
];

// Plugin manifests whose version lives under a nested key (not top-level).
// Each entry declares the relative path and the dot-path to the version field.
const VERSION_PINNED_NESTED = [
  // Claude Code marketplace catalog — version sits under metadata.version.
  { path: ".claude-plugin/marketplace.json", key: "metadata.version" },
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

function syncNestedPluginVersion({ path: relPath, key }) {
  const abs = join(REPO, relPath);
  const data = JSON.parse(readFileSync(abs, "utf8"));
  const segments = key.split(".");
  let cursor = data;
  for (let i = 0; i < segments.length - 1; i += 1) {
    if (cursor[segments[i]] === undefined) {
      console.error(`[release-prepare] missing path ${key} in ${relPath}`);
      return false;
    }
    cursor = cursor[segments[i]];
  }
  const last = segments[segments.length - 1];
  if (cursor[last] === NEXT_VERSION) {
    return false;
  }
  cursor[last] = NEXT_VERSION;
  writeFileSync(abs, JSON.stringify(data, null, 2) + "\n", "utf8");
  return true;
}

// Sync the "current published version" banner and supported-version table in
// SECURITY.md from package.json. Keeps the security policy credible without
// requiring a manual edit on every release.
function syncSecurityMd(nextVersion) {
  const abs = join(REPO, "SECURITY.md");
  let content = readFileSync(abs, "utf8");
  const [major, minor] = nextVersion.split(".").map(Number);
  const curr = `${major}.${minor}.x`;

  // At a major-version boundary (X.0.0), the "previous minor" is the last
  // minor of the previous major — which this script cannot know precisely.
  // Use `(major-1).x` as a conservative label and `< X.0.0` as the floor.
  // A human must verify the exact prior-major support window on X.0.0 releases.
  const isMajorBump = minor === 0;
  const prev = isMajorBump ? `${major - 1}.x` : `${major}.${minor - 1}.x`;
  const floor = isMajorBump ? `${major}.0.0` : `${major}.${minor - 1}.0`;

  const updated = content
    .replace(
      /current published version: \*\*[\d.]+\*\*/,
      `current published version: **${nextVersion}**`
    )
    .replace(
      /\| \d+\.\d+\.x\s+\| Yes — current minor \|[^\n]*/,
      `| ${curr.padEnd(13)} | Yes — current minor |`
    )
    .replace(
      /\| [\d.x]+\s+\| Yes — previous minor \|[^\n]*/,
      `| ${prev.padEnd(13)} | Yes — previous minor |`
    )
    .replace(
      /\| < [\d.]+\s+\| No[^\n]*/,
      `| < ${floor.padEnd(10)} | No                 |`
    );

  if (updated === content) return false;
  writeFileSync(abs, updated, "utf8");
  return true;
}

function syncChangelogCounts(nextVersion) {
  // Update counts in CHANGELOG.md from the live catalog to prevent stale hardcoded values.
  // Pattern: > _Provider-scoped exports... 334 agents · 335 skills · 26 providers · 16 roles_
  const abs = join(REPO, "CHANGELOG.md");
  let content = readFileSync(abs, "utf8");

  // Count agents, skills, providers, roles from live catalog
  const agentDirs = readdirSync(join(REPO, "agents"), { recursive: true });
  const agentCount = agentDirs.filter(f => String(f).endsWith("metadata.json")).length;

  const skillDirs = readdirSync(join(REPO, "skills"), { recursive: true });
  const skillCount = skillDirs.filter(f => String(f).endsWith("SKILL.md")).length;

  const allProviders = new Set();
  for (const f of agentDirs) {
    if (!String(f).endsWith("metadata.json")) continue;
    const m = JSON.parse(readFileSync(join(REPO, "agents", String(f)), "utf8"));
    if (m.provider) allProviders.add(m.provider);
  }
  const providerCount = allProviders.size;

  const roles = JSON.parse(readFileSync(join(REPO, "catalog/install-roles.json"), "utf8"));
  const roleCount = Object.keys(roles.roles).length;

  // Replace counts in all version blurb lines
  const versionRegex = /(\> _[^.]+\. )\d+ agents · \d+ skills · \d+ providers · \d+ roles/g;
  const updated = content.replace(
    versionRegex,
    `$1${agentCount} agents · ${skillCount} skills · ${providerCount} providers · ${roleCount} roles`
  );

  if (updated === content) return false;
  writeFileSync(abs, updated, "utf8");
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
for (const entry of VERSION_PINNED_NESTED) {
  if (syncNestedPluginVersion(entry)) {
    console.log(`[release-prepare] updated ${entry.path} (${entry.key})`);
    touched += 1;
  }
}

if (syncSecurityMd(NEXT_VERSION)) {
  console.log("[release-prepare] updated SECURITY.md");
  touched += 1;
}

if (syncChangelogCounts(NEXT_VERSION)) {
  console.log("[release-prepare] updated CHANGELOG.md counts");
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
