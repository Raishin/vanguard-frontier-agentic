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
import { dirname, isAbsolute, join, relative, sep } from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = join(dirname(fileURLToPath(import.meta.url)), "..");
const catalogPath = join(repoRoot, "catalog", "agents.json");
const manifestPath = join(repoRoot, ".cursor-plugin", "plugin.json");
const pkgPath = join(repoRoot, "package.json");

const check = process.argv.includes("--check");

const pkg = JSON.parse(readFileSync(pkgPath, "utf8"));
const catalog = JSON.parse(readFileSync(catalogPath, "utf8"));

function manifestPathForAdapter(entry, adapter) {
  // Note 1: Cursor consumes the generated manifest as an install surface, so
  // metadata paths need the same trust boundary treatment as user input.
  if (
    typeof adapter !== "string"
    || adapter.trim() === ""
    || isAbsolute(adapter)
  ) {
    throw new Error(
      `Agent ${entry.id} has an invalid cursor harness path: ${adapter}`,
    );
  }
  // Note 2: path.join() normalizes syntax but can still produce a resolved
  // path outside repoRoot when adapter contains ".." segments.
  const resolved = join(repoRoot, adapter);
  const rel = relative(repoRoot, resolved);
  // Note 3: The relative path is the security decision point. Values that
  // start with ".." would make the generated Cursor manifest reference files
  // outside the plugin package.
  if (rel === "" || rel.startsWith(`..${sep}`) || rel === ".." || isAbsolute(rel)) {
    throw new Error(
      `Agent ${entry.id} cursor harness path escapes the repository: ${adapter}`,
    );
  }
  // Note 4: JSON plugin manifests should be deterministic across platforms;
  // converting separators avoids OS-specific diffs.
  return `./${rel.split(sep).join("/")}`;
}

const agentEntries = catalog
  .filter((e) => e.type === "agent")
  .filter((e) => Array.isArray(e.harnesses) && e.harnesses.includes("cursor"))
  .map((e) => {
    const adapter =
      e.harness_variants?.cursor ?? `${e.path}/harnesses/cursor.agent.md`;
    return manifestPathForAdapter(e, adapter);
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
