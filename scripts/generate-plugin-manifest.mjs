#!/usr/bin/env node
/**
 * Generate .claude-plugin/plugin.json and .claude-plugin/marketplace.json
 * from catalog/agents.json and package.json.
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
 * marketplace.json version and agent count are also auto-computed so both
 * files stay in sync with package.json as the single source of truth.
 *
 * Output is sorted and deterministic so the manifest is reproducible across
 * runs and reviewable in PR diffs.
 *
 * Mode:
 *   --check  exit 1 if the on-disk manifest does not match the generated one
 *   (default) overwrite the on-disk manifest
 */

import { readFileSync, writeFileSync, existsSync } from "node:fs";
import { dirname, isAbsolute, join, relative, sep } from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = join(dirname(fileURLToPath(import.meta.url)), "..");
const catalogPath = join(repoRoot, "catalog", "agents.json");
const skillManifestPath = join(repoRoot, "catalog", "skill-manifest.json");
const manifestPath = join(repoRoot, ".claude-plugin", "plugin.json");
const marketplacePath = join(repoRoot, ".claude-plugin", "marketplace.json");
const pkgPath = join(repoRoot, "package.json");

const check = process.argv.includes("--check");

const pkg = JSON.parse(readFileSync(pkgPath, "utf8"));
const catalog = JSON.parse(readFileSync(catalogPath, "utf8"));

function manifestPathForAdapter(entry, adapter) {
  // Note 1: Treat catalog paths as untrusted input. Even though this repo
  // owns catalog/agents.json, release tooling should fail closed because a
  // future PR can change metadata without touching the generator itself.
  if (
    typeof adapter !== "string"
    || adapter.trim() === ""
    || isAbsolute(adapter)
  ) {
    throw new Error(
      `Agent ${entry.id} has an invalid claude-code harness path: ${adapter}`,
    );
  }
  // Note 2: Joining against repoRoot normalizes ordinary relative paths, but
  // it does not by itself prove containment. For example, "../x" resolves
  // successfully; the relative() check below is what detects the escape.
  const resolved = join(repoRoot, adapter);
  const rel = relative(repoRoot, resolved);
  // Note 3: A path is inside repoRoot only when the normalized relative path
  // is not "..", does not start with "../", and is not absolute. This mirrors
  // the defensive pattern used by the export CLI for install destinations.
  if (rel === "" || rel.startsWith(`..${sep}`) || rel === ".." || isAbsolute(rel)) {
    throw new Error(
      `Agent ${entry.id} claude-code harness path escapes the repository: ${adapter}`,
    );
  }
  // Note 4: Plugin manifests use POSIX-style paths even when generated on
  // Windows, so split/join converts platform separators into stable JSON.
  return `./${rel.split(sep).join("/")}`;
}

const allAgents = catalog.filter((entry) => entry.type === "agent");
const agentEntries = allAgents
  .filter((entry) => Array.isArray(entry.harnesses) && entry.harnesses.includes("claude-code"))
  .map((entry) => {
    const adapter = entry.harness_variants?.["claude-code"]
      ?? `${entry.path}/harnesses/claude-code.agent.md`;
    return manifestPathForAdapter(entry, adapter);
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

// Derive a concise agent count and provider list for the marketplace description.
const totalAgents = agentEntries.length;
const providers = [...new Set(allAgents.map((a) => a.provider).filter(Boolean))].sort();

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

// marketplace.json — version and agent count auto-computed from package.json
// and catalog/agents.json so they never diverge from the installed artifact.
const marketplace = {
  name: "vanguard-frontier-agentic",
  owner: {
    name: "Raishin",
    url: "https://github.com/Raishin",
  },
  metadata: {
    description: pkg.description,
    version: pkg.version,
  },
  plugins: [
    {
      name: "vanguard-frontier-agentic",
      source: "./",
      description: `All ${totalAgents} cloud, security, compliance, platform, accounting, and finance agents in one install. Includes maestros, advisory reviewers, and live-mutation guards across ${providers.length} providers.`,
      category: "cloud",
      tags: [
        "agents",
        "cloud",
        "kubernetes",
        "terraform",
        "zero-trust",
        "compliance",
        "live-guards",
      ],
      strict: true,
    },
  ],
};

const nextManifest = JSON.stringify(manifest, null, 2) + "\n";
const nextMarketplace = JSON.stringify(marketplace, null, 2) + "\n";

if (check) {
  let ok = true;
  if (!existsSync(manifestPath)) {
    console.error(`ERROR: ${manifestPath} is missing; run npm run plugin-manifest:write`);
    ok = false;
  } else {
    const current = readFileSync(manifestPath, "utf8");
    if (current !== nextManifest) {
      console.error(
        `ERROR: ${relative(repoRoot, manifestPath)} is stale (${agentEntries.length} agents in catalog, manifest is out of sync); run npm run plugin-manifest:write`,
      );
      ok = false;
    }
  }
  if (!existsSync(marketplacePath)) {
    console.error(`ERROR: ${marketplacePath} is missing; run npm run plugin-manifest:write`);
    ok = false;
  } else {
    const currentMarketplace = readFileSync(marketplacePath, "utf8");
    if (currentMarketplace !== nextMarketplace) {
      console.error(
        `ERROR: ${relative(repoRoot, marketplacePath)} is stale (version or agent count changed); run npm run plugin-manifest:write`,
      );
      ok = false;
    }
  }
  if (!ok) process.exit(1);
  console.log(
    `OK: plugin manifest is in sync (${agentEntries.length} agents, ${skillCount} skills tracked in catalog/skill-manifest.json)`,
  );
} else {
  writeFileSync(manifestPath, nextManifest);
  writeFileSync(marketplacePath, nextMarketplace);
  console.log(
    `OK: wrote ${relative(repoRoot, manifestPath)} (${agentEntries.length} agents)`,
  );
  console.log(
    `OK: wrote ${relative(repoRoot, marketplacePath)} (version=${pkg.version}, agents=${totalAgents}, providers=${providers.length})`,
  );
}

