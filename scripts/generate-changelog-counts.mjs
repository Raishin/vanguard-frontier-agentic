#!/usr/bin/env node
/**
 * Generate dynamic catalog counts for CHANGELOG.md and other versioning docs.
 * Called by release-prepare.mjs to keep counts current across releases.
 */

import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");

// Count agents
const agentDirs = fs.readdirSync(path.join(repoRoot, "agents"), { recursive: true });
const agentCount = agentDirs.filter(f => String(f).endsWith("metadata.json")).length;

// Count skills (on-disk SKILL.md files)
const skillDirs = fs.readdirSync(path.join(repoRoot, "skills"), { recursive: true });
const skillCount = skillDirs.filter(f => String(f).endsWith("SKILL.md")).length;

// Count catalog providers (agents only, not skill-only dirs)
const allProviders = new Set();
for (const f of agentDirs) {
  if (!String(f).endsWith("metadata.json")) continue;
  const m = JSON.parse(fs.readFileSync(path.join(repoRoot, "agents", String(f)), "utf8"));
  if (m.provider) allProviders.add(m.provider);
}
const providerCount = allProviders.size;

// Count roles
const roles = JSON.parse(fs.readFileSync(path.join(repoRoot, "catalog/install-roles.json"), "utf8"));
const roleCount = Object.keys(roles.roles).length;

console.log(`${agentCount} agents · ${skillCount} skills · ${providerCount} providers · ${roleCount} roles`);
