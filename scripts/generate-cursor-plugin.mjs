#!/usr/bin/env node
/**
 * Generate .cursor-plugin/plugin.json from catalog/agents.json.
 *
 * Mirrors scripts/generate-plugin-manifest.mjs (Claude Code) but writes
 * a Cursor plugin manifest (cursor.com/docs/plugins, /docs/reference/plugins).
 * A Cursor plugin is a directory containing .cursor-plugin/plugin.json
 * plus bundled rules/, skills/, agents/, commands/, hooks/, and mcp.json.
 * The repo's existing cursor adapters live at
 *   agents/<provider>/<agent>/harnesses/cursor.agent.md
 * so we enumerate them as explicit paths in the manifest's `agents` array.
 *
 * Mode:
 *   --check  exit 1 if the on-disk manifest does not match
 *   (default) overwrite the on-disk manifest
 */

import { readFileSync, writeFileSync, existsSync } from "node:fs";
import { dirname, join, relative } from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = join(dirname(fileURLToPath(import.meta.url)), "..");
const catalogPath = join(repoRoot, "catalog", "agents.json");
const manifestPath = join(repoRoot, ".cursor-plugin", "plugin.json");
const pkgPath = join(repoRoot, "package.json");

const check = process.argv.includes("--check");

const pkg = JSON.parse(readFileSync(pkgPath, "utf8"));
const catalog = JSON.parse(readFileSync(catalogPath, "utf8"));

const agentEntries = catalog
  .filter((e) => e.type === "agent")
  .filter((e) => Array.isArray(e.harnesses) && e.harnesses.includes("cursor"))
  .map((e) => {
    const adapter =
      e.harness_variants?.cursor ?? `${e.path}/harnesses/cursor.agent.md`;
    return `./${relative(repoRoot, join(repoRoot, adapter))}`;
  })
  .sort();

const missing = agentEntries.filter((p) => !existsSync(join(repoRoot, p)));
if (missing.length > 0) {
  console.error("ERROR: cursor plugin manifest references missing files:");
  missing.forEach((p) => console.error("  " + p));
  process.exit(2);
}

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
    "cursor",
  ],
  agents: agentEntries,
};

const next = JSON.stringify(manifest, null, 2) + "\n";

if (check) {
  if (!existsSync(manifestPath)) {
    console.error(
      `ERROR: ${manifestPath} is missing; run npm run cursor-plugin:write`,
    );
    process.exit(1);
  }
  const current = readFileSync(manifestPath, "utf8");
  if (current !== next) {
    console.error(
      `ERROR: ${relative(repoRoot, manifestPath)} is stale (${agentEntries.length} cursor agents in catalog); run npm run cursor-plugin:write`,
    );
    process.exit(1);
  }
  console.log(`OK: cursor plugin manifest in sync (${agentEntries.length} agents)`);
} else {
  writeFileSync(manifestPath, next);
  console.log(
    `OK: wrote ${relative(repoRoot, manifestPath)} (${agentEntries.length} agents)`,
  );
}
