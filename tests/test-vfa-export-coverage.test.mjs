#!/usr/bin/env node
/**
 * Coverage + CLI tests for vfa-export-agents role/provider install paths.
 *
 * TDD contract:
 *
 *   A. Catalog coverage
 *      1. Every agent in catalog/agents.json appears in at least one
 *         role in catalog/install-roles.json (no orphans).
 *      2. Every provider that has agents has at least one role-covered
 *         agent (no orphan providers).
 *      3. Every agent id referenced by a role exists in catalog/agents.json.
 *      4. Every skill id referenced by a role exists in catalog/skills.json.
 *
 *   B. CLI — per-provider install
 *      5. --provider <p> --all selects exactly the agents whose provider==p.
 *      6. --provider <p> alone (no --role, no --agents) is equivalent to
 *         --provider <p> --all.
 *      7. --provider <p> --role <r> filters role agents to provider p (existing
 *         behavior — must not regress).
 *      8. --provider <unknown> emits a descriptive error and exits non-zero.
 *      9. --list-providers prints every distinct provider in the catalog.
 *
 *   C. NVIDIA presence (regression guard for PR #22)
 *     10. nvidia-model-promotion-gatekeeper-agent is in at least one role.
 *     11. Every NVIDIA agent is in at least one role.
 *
 * Run: node tests/test-vfa-export-coverage.test.mjs
 */

import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const exporter = path.join(repoRoot, "scripts", "export-marketplace-agents.mjs");

const agents = JSON.parse(fs.readFileSync(path.join(repoRoot, "catalog/agents.json"), "utf8"));
const skills = JSON.parse(fs.readFileSync(path.join(repoRoot, "catalog/skills.json"), "utf8"));
const rolesDoc = JSON.parse(fs.readFileSync(path.join(repoRoot, "catalog/install-roles.json"), "utf8"));

const agentIds = new Set(agents.map((a) => a.id));
const skillIds = new Set(skills.map((s) => s.id));
const byId = new Map(agents.map((a) => [a.id, a]));
const providersInCatalog = new Set(agents.map((a) => a.provider));

const allRoleAgentIds = new Set();
const allRoleSkillIds = new Set();
for (const role of Object.values(rolesDoc.roles)) {
  for (const id of role.agents) allRoleAgentIds.add(id);
  for (const id of role.skills ?? []) allRoleSkillIds.add(id);
}

let failures = 0;
const ok = (msg) => console.log(`OK   ${msg}`);
const fail = (msg) => {
  console.log(`FAIL ${msg}`);
  failures += 1;
};

// ── A. Catalog coverage ──────────────────────────────────────────────────────

const orphans = agents.filter((a) => !allRoleAgentIds.has(a.id));
if (orphans.length === 0) {
  ok("A1 every agent appears in at least one role");
} else {
  fail(`A1 ${orphans.length} agent(s) appear in no role:\n  ` +
    orphans.slice(0, 20).map((a) => `[${a.provider}] ${a.id}`).join("\n  ") +
    (orphans.length > 20 ? `\n  ... and ${orphans.length - 20} more` : ""));
}

const orphanProviders = [];
for (const p of providersInCatalog) {
  const covered = agents.some((a) => a.provider === p && allRoleAgentIds.has(a.id));
  if (!covered) orphanProviders.push(p);
}
if (orphanProviders.length === 0) {
  ok("A2 every provider has at least one role-covered agent");
} else {
  fail(`A2 orphan providers: ${orphanProviders.join(", ")}`);
}

const danglingRoleAgents = [...allRoleAgentIds].filter((id) => !agentIds.has(id));
if (danglingRoleAgents.length === 0) {
  ok("A3 every role-referenced agent id exists in catalog");
} else {
  fail(`A3 role references unknown agent ids: ${danglingRoleAgents.join(", ")}`);
}

const danglingRoleSkills = [...allRoleSkillIds].filter((id) => !skillIds.has(id));
if (danglingRoleSkills.length === 0) {
  ok("A4 every role-referenced skill id exists in catalog");
} else {
  fail(`A4 role references unknown skill ids: ${danglingRoleSkills.join(", ")}`);
}

// ── B. CLI behaviour ─────────────────────────────────────────────────────────

function run(args) {
  const r = spawnSync(process.execPath, [exporter, ...args], { encoding: "utf8" });
  return { stdout: r.stdout ?? "", stderr: r.stderr ?? "", exitCode: r.status ?? 0 };
}

// 5. --provider <p> --all should list the same count as the catalog says.
{
  const r = run(["--platform", "claude-code", "--provider", "nvidia", "--all", "--dry-run"]);
  const nvidiaCount = agents.filter((a) => a.provider === "nvidia").length;
  const matches = (r.stdout.match(/^export agent:/gm) || []).length;
  if (r.exitCode === 0 && matches === nvidiaCount) {
    ok(`B5 --provider nvidia --all exports ${matches}/${nvidiaCount} agents`);
  } else {
    fail(`B5 expected ${nvidiaCount} agents, got ${matches}; exit=${r.exitCode}\nstderr: ${r.stderr.slice(0, 500)}`);
  }
}

// 6. --provider alone == --provider --all
{
  const r = run(["--platform", "claude-code", "--provider", "nvidia", "--dry-run"]);
  const nvidiaCount = agents.filter((a) => a.provider === "nvidia").length;
  const matches = (r.stdout.match(/^export agent:/gm) || []).length;
  if (r.exitCode === 0 && matches === nvidiaCount) {
    ok(`B6 --provider nvidia (no --all) exports ${matches}/${nvidiaCount} agents`);
  } else {
    fail(`B6 expected ${nvidiaCount}, got ${matches}; exit=${r.exitCode}\nstderr: ${r.stderr.slice(0, 500)}`);
  }
}

// 7. --provider + --role filters role to provider (regression guard).
{
  const role = rolesDoc.roles["cloud-security-engineer"];
  const expectedAzure = role.agents.filter((id) => byId.get(id)?.provider === "azure");
  const r = run(["--platform", "claude-code", "--role", "cloud-security-engineer", "--provider", "azure", "--dry-run"]);
  const matches = (r.stdout.match(/^export agent:/gm) || []).length;
  if (r.exitCode === 0 && matches === expectedAzure.length && expectedAzure.length > 0) {
    ok(`B7 --role cloud-security-engineer --provider azure exports ${matches} (=${expectedAzure.length}) agents`);
  } else {
    fail(`B7 expected ${expectedAzure.length} azure security agents, got ${matches}; exit=${r.exitCode}`);
  }
}

// 8. Unknown provider rejected.
{
  const r = run(["--platform", "claude-code", "--provider", "not-a-real-provider", "--dry-run"]);
  if (r.exitCode !== 0 && /provider/i.test(r.stderr)) {
    ok("B8 unknown --provider rejected with descriptive error");
  } else {
    fail(`B8 expected non-zero exit and 'provider' in stderr; exit=${r.exitCode}\nstderr: ${r.stderr.slice(0, 300)}`);
  }
}

// 9. --list-providers prints every distinct provider.
{
  const r = run(["--list-providers"]);
  const missing = [...providersInCatalog].filter((p) => !r.stdout.includes(p));
  if (r.exitCode === 0 && missing.length === 0) {
    ok(`B9 --list-providers prints all ${providersInCatalog.size} providers`);
  } else {
    fail(`B9 missing from --list-providers: ${missing.join(", ")}; exit=${r.exitCode}`);
  }
}

// ── C. NVIDIA regression guard ───────────────────────────────────────────────

if (allRoleAgentIds.has("nvidia-model-promotion-gatekeeper-agent")) {
  ok("C10 nvidia-model-promotion-gatekeeper-agent present in at least one role");
} else {
  fail("C10 nvidia-model-promotion-gatekeeper-agent missing from every role");
}

const nvidiaOrphans = agents
  .filter((a) => a.provider === "nvidia" && !allRoleAgentIds.has(a.id))
  .map((a) => a.id);
if (nvidiaOrphans.length === 0) {
  ok("C11 every NVIDIA agent present in at least one role");
} else {
  fail(`C11 NVIDIA agents missing from every role: ${nvidiaOrphans.join(", ")}`);
}

// ── Summary ─────────────────────────────────────────────────────────────────

if (failures > 0) {
  console.error(`\n${failures} check(s) failed`);
  process.exit(1);
}
console.log(`\nOK: all coverage and CLI checks passed`);
