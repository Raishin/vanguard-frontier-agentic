#!/usr/bin/env node
/**
 * Generate .claude-plugin/plugin.json from catalog/agents.json.
 *
 * The Claude Code plugin spec lets a manifest declare its agents via an
 * explicit array of file paths (see code.claude.com/docs/en/plugins-reference,
 * field `agents`). The repo's agents live at
 *   agents/<provider>/<agent>/harnesses/claude-code.agent.md
 * which is one nesting level deeper than the conventional flat
 *   agents/<agent>.md
 * Rather than restructuring the catalog (which would break the multi-harness
 * design and the npm package layout), this script enumerates every
 * claude-code adapter file and writes it into plugin.json's `agents` array.
 *
 * Output is sorted and deterministic so the manifest is reproducible across
 * runs and reviewable in PR diffs.
 *
 * Mode:
 *   --check  exit 1 if the on-disk manifest does not match the generated one
 *   (default) overwrite the on-disk manifest
 */

import { readFileSync, writeFileSync, existsSync } from "node:fs";
import { dirname, join, relative } from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = join(dirname(fileURLToPath(import.meta.url)), "..");
const catalogPath = join(repoRoot, "catalog", "agents.json");
const skillManifestPath = join(repoRoot, "catalog", "skill-manifest.json");
const manifestPath = join(repoRoot, ".claude-plugin", "plugin.json");
const pkgPath = join(repoRoot, "package.json");

const check = process.argv.includes("--check");

const pkg = JSON.parse(readFileSync(pkgPath, "utf8"));
const catalog = JSON.parse(readFileSync(catalogPath, "utf8"));

const agentEntries = catalog
  .filter((entry) => entry.type === "agent")
  .filter((entry) => Array.isArray(entry.harnesses) && entry.harnesses.includes("claude-code"))
  .map((entry) => {
    const adapter = entry.harness_variants?.["claude-code"]
      ?? `${entry.path}/harnesses/claude-code.agent.md`;
    return `./${relative(repoRoot, join(repoRoot, adapter))}`;
  })
  .sort();

// Validate every adapter file actually exists. A missing adapter would let
// Claude Code register a phantom agent that errors on first invocation.
const missing = agentEntries.filter((p) => !existsSync(join(repoRoot, p)));
if (missing.length > 0) {
  console.error("ERROR: plugin manifest references files that do not exist:");
  missing.forEach((p) => console.error("  " + p));
  process.exit(2);
}

// Skills are nested under skills/<provider>/<skill>/SKILL.md. Claude Code's
// plugin loader expects skills/<skill>/SKILL.md (one level less). Until
// the skill tree is flattened or Claude Code adds nested discovery, we
// omit `skills` from the manifest to avoid silently shipping zero skills
// under a misleading declaration. The catalog/skill-manifest.json remains
// authoritative for the npm install path.
const skillCount = existsSync(skillManifestPath)
  ? JSON.parse(readFileSync(skillManifestPath, "utf8")).length ?? 0
  : 0;

const manifest = {
  name: "vanguard-frontier-agentic",
  version: pkg.version,
  description: pkg.description,
  author: {
    name: "Raishin",
    url: "https://github.com/Raishin",
  },
  homepage: "https://github.com/Raishin/vanguard-frontier-agentic",
  repository: "https://github.com/Raishin/vanguard-frontier-agentic",
  license: pkg.license,
  keywords: [
    "agentic",
    "agents",
    "marketplace",
    "cloud",
    "aws",
    "azure",
    "gcp",
    "oci",
    "alibaba",
    "huawei",
    "kubernetes",
    "terraform",
    "zero-trust",
    "compliance",
    "claude-code",
  ],
  agents: agentEntries,
};

const next = JSON.stringify(manifest, null, 2) + "\n";

if (check) {
  if (!existsSync(manifestPath)) {
    console.error(`ERROR: ${manifestPath} is missing; run npm run plugin-manifest:write`);
    process.exit(1);
  }
  const current = readFileSync(manifestPath, "utf8");
  if (current !== next) {
    console.error(
      `ERROR: ${relative(repoRoot, manifestPath)} is stale (${agentEntries.length} agents in catalog, manifest is out of sync); run npm run plugin-manifest:write`,
    );
    process.exit(1);
  }
  console.log(
    `OK: plugin manifest is in sync (${agentEntries.length} agents, ${skillCount} skills tracked in catalog/skill-manifest.json)`,
  );
} else {
  writeFileSync(manifestPath, next);
  console.log(
    `OK: wrote ${relative(repoRoot, manifestPath)} (${agentEntries.length} agents)`,
  );
}
