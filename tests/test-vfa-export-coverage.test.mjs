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
 *   D. Provider skill-scope enforcement (regression guard for P0 fix)
 *     12. AWS-scoped role export: zero rival-provider skills in --dry-run output.
 *     13. Azure-scoped role export: zero rival-provider skills in --dry-run output.
 *     14. Standalone --provider aws --all: zero rival-provider skills.
 *
 *   E. Dry-run completeness
 *     15. claude-code --dry-run emits both agent and skill lines.
 *     16. --dry-run --no-skills omits skill lines.
 *     17. cursor --dry-run emits agent lines but no skill lines (unsupported platform).
 *     18. --dry-run stderr summary reports skill count on skill-capable platform.
 *     19. codex --all --dry-run emits both agent and skill lines.
 *     20. codex --dry-run --no-skills omits skill lines.
 *
 *   F. Full CLI flag surface
 *     19. --list exits 0 and prints all agents.
 *     20. --list-roles exits 0 and prints all roles.
 *     21. --list-providers exits 0 and includes 'aws'.
 *     22. --agents <single-id> selects exactly that agent.
 *     23. --agents <id1>,<id2> selects exactly those 2 agents.
 *     24. --all selects every agent in the catalog.
 *     25. --platform claude (alias) resolves to claude-code.
 *     26. --no-skills writes agent file but no skills directory.
 *     27. --force overwrites existing agent files without error.
 *     28. codex export writes agent + companion skill and rewrites skill path.
 *     29. two-stage Codex installer dry-run plans all agents and skills.
 *
 *   G. Error / rejection cases
 *     28. No args → usage text printed, non-zero exit.
 *     29. Unknown --role → exit non-zero with 'role' in output.
 *     30. Unknown --platform → exit non-zero with 'platform' in output.
 *     31. Unknown --agents id → exit non-zero.
 *     32. --platform with no selector → exit non-zero.
 *
 * Run: node tests/test-vfa-export-coverage.test.mjs
 */

import { spawnSync } from "node:child_process";
import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const exporter = path.join(repoRoot, "scripts", "export-marketplace-agents.mjs");
const installer = path.join(repoRoot, "scripts", "install-codex-home.mjs");

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
  const r = spawnSync(process.execPath, [exporter, ...args], { encoding: "utf8", timeout: 30000 });
  if (r.signal === "SIGTERM") fail(`spawnSync timed out (30s) for: ${args.join(" ")}`);
  return { stdout: r.stdout ?? "", stderr: r.stderr ?? "", exitCode: r.status ?? 0 };
}

function runInstaller(args) {
  const r = spawnSync(process.execPath, [installer, ...args], { encoding: "utf8", timeout: 30000 });
  if (r.signal === "SIGTERM") fail(`installer timed out (30s) for: ${args.join(" ")}`);
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

// ── D. Provider skill-scope enforcement ──────────────────────────────────────
//
// Build the rival-provider set from the ACTUAL on-disk skill catalog so new
// providers are automatically covered without updating a hardcoded list.
// This replaces the fragile 5-entry RIVAL_PREFIXES approach that missed 20 of
// 26 providers (kubernetes, terraform, nvidia, ovhcloud, etc.).

const skillsRoot = path.join(repoRoot, "skills");
const skillProviderDirs = fs.readdirSync(skillsRoot, { withFileTypes: true })
  .filter((d) => d.isDirectory())
  .map((d) => d.name);

// Map: skillName → provider (reads from metadata.json, falls back to directory name)
// Mirrors the loadSkills() logic in scripts/export-marketplace-agents.mjs
const skillProviderByName = new Map();
for (const prov of skillProviderDirs) {
  const provDir = path.join(skillsRoot, prov);
  for (const skill of fs.readdirSync(provDir, { withFileTypes: true })) {
    if (!skill.isDirectory()) continue;
    const skillDir = path.join(provDir, skill.name);
    const metaFile = path.join(skillDir, "metadata.json");
    if (fs.existsSync(path.join(skillDir, "SKILL.md"))) {
      let skillProvider = prov; // Default to directory name
      if (fs.existsSync(metaFile)) {
        try {
          const meta = JSON.parse(fs.readFileSync(metaFile, "utf8"));
          if (meta.provider) {
            skillProvider = meta.provider; // Use declared provider if available
          }
        } catch (err) {
          // Fall back to directory name if metadata.json is invalid
        }
      }
      skillProviderByName.set(skill.name, skillProvider);
    }
  }
}

function extractSkillNames(stdout) {
  return (stdout.match(/^export skill: .+$/gm) || [])
    .map((l) => l.replace("export skill: ", "").trim());
}

function extractAgentIds(stdout) {
  return (stdout.match(/^export agent: .+$/gm) || [])
    .map((l) => l.replace(/^export agent: /, "").replace(/ \[provider=[^\]]+\]$/, "").trim());
}

// Returns skills whose catalog provider does not match expectedProvider and is not "shared".
function findLeakedSkills(skillNames, expectedProvider) {
  return skillNames.filter((s) => {
    const prov = skillProviderByName.get(s);
    if (!prov) return false; // unknown/orphan skill — can't classify
    // Allow export of skills with provider='shared' or provider='generic'.
    // generic is used by language/stack boards (dotnet, legal, hr, marketing).
    return prov !== expectedProvider && prov !== "shared" && prov !== "generic";
  });
}

// D12: AWS-scoped role export — no rival-provider skills in dry-run output.
{
  const r = run(["--platform", "claude-code", "--role", "cloud-security-engineer", "--provider", "aws", "--dry-run"]);
  const skills = extractSkillNames(r.stdout);
  const leaked = findLeakedSkills(skills, "aws");
  if (r.exitCode === 0 && skills.length > 0 && leaked.length === 0) {
    ok(`D12 aws-scoped role: ${skills.length} skill(s), 0 non-AWS skills leaked (catalog-verified)`);
  } else {
    fail(`D12 ${leaked.length} non-AWS skill(s) leaked: ${leaked.join(", ")} | total=${skills.length} exit=${r.exitCode}`);
  }
}

// D13: Azure-scoped role export — no rival-provider skills in dry-run output.
{
  const r = run(["--platform", "claude-code", "--role", "cloud-security-engineer", "--provider", "azure", "--dry-run"]);
  const skills = extractSkillNames(r.stdout);
  const leaked = findLeakedSkills(skills, "azure");
  if (r.exitCode === 0 && skills.length > 0 && leaked.length === 0) {
    ok(`D13 azure-scoped role: ${skills.length} skill(s), 0 non-Azure skills leaked (catalog-verified)`);
  } else {
    fail(`D13 ${leaked.length} non-Azure skill(s) leaked: ${leaked.join(", ")} | total=${skills.length} exit=${r.exitCode}`);
  }
}

// D14: Standalone --provider aws --all — skills ARE exported and no rival-provider skills leak.
// Previously this assertion was vacuously true when skills.length === 0 (zero leaked from empty
// set is always true). Now we require at least 1 skill to detect silent regression in skill resolution.
{
  const r = run(["--platform", "claude-code", "--provider", "aws", "--all", "--dry-run"]);
  const skills = extractSkillNames(r.stdout);
  const leaked = findLeakedSkills(skills, "aws");
  if (r.exitCode === 0 && skills.length > 0 && leaked.length === 0) {
    ok(`D14 --provider aws --all: ${skills.length} skill(s), 0 non-AWS skills leaked (catalog-verified)`);
  } else {
    fail(`D14 skills=${skills.length} (need >0), leaked=${leaked.length}: ${leaked.join(", ")} | exit=${r.exitCode}`);
  }
}

// D14c: Full role×provider matrix — every valid (role, provider) combination exports zero
// rival-provider skills. This is the most exhaustive scope guard in the suite: it exercises
// the role.skills filter, the per-agent companion_skills filter, and the name-stripping fallback
// across all combinations where the catalog actually has agents.
//
// Skill-only provider dirs (finops, velero, claude) have no catalog agents so they never
// appear as selectedProvider — their skills being excluded from scoped exports is EXPECTED.
// The test flags a skill as leaked only when its on-disk provider is a catalog provider
// that differs from the selected one.
{
  // Load all roles from the catalog to drive the matrix — no hardcoded role list.
  const installRoles = JSON.parse(
    fs.readFileSync(path.join(repoRoot, "catalog", "install-roles.json"), "utf8")
  ).roles;

  // Load agent catalog to map agent id → provider
  const agentProviderById = new Map();
  const agentDirs = fs.readdirSync(path.join(repoRoot, "agents"), { recursive: true });
  for (const f of agentDirs) {
    if (!String(f).endsWith("metadata.json")) continue;
    const raw = JSON.parse(fs.readFileSync(path.join(repoRoot, "agents", String(f)), "utf8"));
    if (raw.id && raw.provider) agentProviderById.set(raw.id, raw.provider);
  }

  // For each role, derive which catalog providers have agents in it.
  const combos = [];
  for (const [roleId, role] of Object.entries(installRoles)) {
    const roleProviders = [...new Set(
      (role.agents || []).map((id) => agentProviderById.get(id)).filter(Boolean)
    )];
    for (const prov of roleProviders) {
      combos.push({ roleId, prov });
    }
  }

  let combosPassed = 0;
  const combosFailed = [];

  for (const { roleId, prov } of combos) {
    const r = run(["--platform", "claude-code", "--role", roleId, "--provider", prov, "--dry-run"]);
    if (r.exitCode !== 0) {
      // Unexpected failure for a valid combo — treat as failure
      combosFailed.push(`${roleId}+${prov}(exit=${r.exitCode})`);
      continue;
    }
    const skills = extractSkillNames(r.stdout);
    const leaked = findLeakedSkills(skills, prov);
    if (leaked.length === 0) {
      combosPassed++;
    } else {
      combosFailed.push(`${roleId}+${prov}(leaked:${leaked.join(",")})`);
    }
  }

  if (combosFailed.length === 0) {
    ok(`D14c role×provider matrix: ${combosPassed}/${combos.length} combinations clean, 0 skill leaks`);
  } else {
    fail(`D14c skill leakage in ${combosFailed.length}/${combos.length} combo(s):\n  ${combosFailed.join("\n  ")}`);
  }
}

// D14b: ALL 26 providers — standalone --provider <p> --all exports zero rival skills.
// D12/D13/D14 cover AWS and Azure explicitly. This loop covers every provider in the
// catalog so new providers are automatically checked without updating a list.
// Providers with no skills (e.g. multi-cloud) trivially pass since leaked === 0.
{
  const r0 = run(["--list-providers"]);
  const allProviders = r0.stdout.trim().split("\n").map((l) => l.split(/\s/)[0]).filter(Boolean);
  let provPassed = 0;
  const provFailed = [];
  for (const prov of allProviders) {
    const r = run(["--platform", "claude-code", "--provider", prov, "--all", "--dry-run"]);
    const skills = extractSkillNames(r.stdout);
    const leaked = findLeakedSkills(skills, prov);
    if (r.exitCode === 0 && leaked.length === 0) {
      provPassed++;
    } else {
      provFailed.push(`${prov}(leaked:${leaked.join(",")})`);
    }
  }
  if (provFailed.length === 0) {
    ok(`D14b all-provider scope sweep: ${provPassed}/${allProviders.length} providers clean, 0 skill leaks`);
  } else {
    fail(`D14b skill leakage detected in ${provFailed.length} provider(s): ${provFailed.join(" | ")}`);
  }
}

// D14d: All 323 invalid role×provider combinations are rejected with non-zero exit.
// These are (role, provider) pairs where the catalog has NO agents from that provider
// in that role. The CLI must fail fast with "No agents found" rather than silently
// exporting 0 agents or crashing with an unhandled exception.
//
// This completes the matrix: D14c proves valid combos export correctly scoped skills;
// D14d proves invalid combos are explicitly rejected, not silently degraded.
{
  // Build the valid-combo set from the catalog — same derivation as D14c.
  const installRolesAll = JSON.parse(
    fs.readFileSync(path.join(repoRoot, "catalog", "install-roles.json"), "utf8")
  ).roles;
  const agentProvByIdAll = new Map();
  for (const f of fs.readdirSync(path.join(repoRoot, "agents"), { recursive: true })) {
    if (!String(f).endsWith("metadata.json")) continue;
    const raw = JSON.parse(fs.readFileSync(path.join(repoRoot, "agents", String(f)), "utf8"));
    if (raw.id && raw.provider) agentProvByIdAll.set(raw.id, raw.provider);
  }
  const validComboSet = new Set();
  for (const [roleId, role] of Object.entries(installRolesAll)) {
    for (const id of role.agents || []) {
      const prov = agentProvByIdAll.get(id);
      if (prov) validComboSet.add(`${roleId}+${prov}`);
    }
  }

  // Get all catalog providers (same source as --list-providers)
  const allProvidersD14d = run(["--list-providers"]).stdout.trim()
    .split("\n").map((l) => l.split(/\s/)[0]).filter(Boolean);

  // Collect all invalid combos
  const invalidCombos = [];
  for (const roleId of Object.keys(installRolesAll)) {
    for (const prov of allProvidersD14d) {
      if (!validComboSet.has(`${roleId}+${prov}`)) {
        invalidCombos.push({ roleId, prov });
      }
    }
  }

  let invalidPassed = 0;
  const invalidFailed = [];

  for (const { roleId, prov } of invalidCombos) {
    const r = run(["--platform", "claude-code", "--role", roleId, "--provider", prov, "--dry-run"]);
    const combinedOut = r.stdout + r.stderr;
    if (r.exitCode !== 0 && /no agents found/i.test(combinedOut)) {
      invalidPassed++;
    } else if (r.exitCode === 0) {
      // Silently exported 0 agents — the dangerous case; no error surfaced
      invalidFailed.push(`${roleId}+${prov}(silently exited 0)`);
    } else {
      // Exited non-zero but without the expected "No agents found" message
      invalidFailed.push(`${roleId}+${prov}(exit=${r.exitCode} but no 'No agents found' in output)`);
    }
  }

  if (invalidFailed.length === 0) {
    ok(`D14d invalid role×provider combos: ${invalidPassed}/${invalidCombos.length} correctly rejected with "No agents found"`);
  } else {
    fail(`D14d ${invalidFailed.length} invalid combo(s) not properly rejected:\n  ${invalidFailed.join("\n  ")}`);
  }
}

// ── D-matrix completeness: identity + count, not just leak-absence ───────────
//
// D14b/D14c/D14d prove "no rival skill leaks" and "invalid combos rejected",
// but a combo could silently export the WRONG set of agents (e.g. drop one,
// or export 0) and still pass a leak-only check. The blocks below close the
// remaining permutation dimensions by asserting the EXACT exported agent
// identity, driven entirely from the catalog (no hardcoded lists).

const ccCapable = (id) => {
  const a = byId.get(id);
  return Array.isArray(a?.harnesses) && a.harnesses.includes("claude-code");
};
const onDiskSkillNames = new Set(skillProviderByName.keys());

// A5: Role-standalone agent identity — for EVERY role, `--role <r>` (no provider)
// exports exactly the claude-code-capable agents declared in that role.
{
  let passed = 0;
  const failed = [];
  for (const [roleId, role] of Object.entries(rolesDoc.roles)) {
    const expected = new Set((role.agents || []).filter(ccCapable));
    const r = run(["--platform", "claude-code", "--role", roleId, "--dry-run"]);
    const got = new Set(extractAgentIds(r.stdout));
    const missing = [...expected].filter((id) => !got.has(id));
    const extra = [...got].filter((id) => !expected.has(id));
    if (r.exitCode === 0 && missing.length === 0 && extra.length === 0) {
      passed++;
    } else {
      failed.push(`${roleId}(missing:[${missing.join(",")}] extra:[${extra.join(",")}] exit=${r.exitCode})`);
    }
  }
  if (failed.length === 0) {
    ok(`A5 role-standalone agent identity: ${passed}/${Object.keys(rolesDoc.roles).length} roles export exactly their claude-code agents`);
  } else {
    fail(`A5 ${failed.length} role(s) export wrong agent set:\n  ${failed.join("\n  ")}`);
  }
}

// A6: Role-standalone skill completeness — for EVERY role, every declared
// role.skills id that exists on disk is actually exported (no silent drop).
{
  let passed = 0;
  const failed = [];
  for (const [roleId, role] of Object.entries(rolesDoc.roles)) {
    const expectedSkills = (role.skills || []).filter((s) => onDiskSkillNames.has(s));
    const r = run(["--platform", "claude-code", "--role", roleId, "--dry-run"]);
    const got = new Set(extractSkillNames(r.stdout));
    const missing = expectedSkills.filter((s) => !got.has(s));
    if (r.exitCode === 0 && missing.length === 0) {
      passed++;
    } else {
      failed.push(`${roleId}(missing:[${missing.join(",")}] exit=${r.exitCode})`);
    }
  }
  if (failed.length === 0) {
    ok(`A6 role-standalone skill completeness: ${passed}/${Object.keys(rolesDoc.roles).length} roles export all on-disk role skills`);
  } else {
    fail(`A6 ${failed.length} role(s) drop declared skills:\n  ${failed.join("\n  ")}`);
  }
}

// D14f: Valid role×provider identity — for every valid combo, the exported
// agent set equals exactly {role agents whose provider==p AND claude-code-capable}.
// Strengthens D14c (which only checked leak-absence) with count + identity.
{
  const combos = [];
  for (const [roleId, role] of Object.entries(rolesDoc.roles)) {
    const provs = new Set((role.agents || []).map((id) => byId.get(id)?.provider).filter(Boolean));
    for (const prov of provs) combos.push({ roleId, prov });
  }
  let passed = 0;
  const failed = [];
  for (const { roleId, prov } of combos) {
    const role = rolesDoc.roles[roleId];
    const expected = new Set(
      (role.agents || []).filter((id) => byId.get(id)?.provider === prov && ccCapable(id))
    );
    const r = run(["--platform", "claude-code", "--role", roleId, "--provider", prov, "--dry-run"]);
    const got = new Set(extractAgentIds(r.stdout));
    const missing = [...expected].filter((id) => !got.has(id));
    const extra = [...got].filter((id) => !expected.has(id));
    // A combo can be "valid" in D14c terms (has agents from prov) yet have zero
    // claude-code-capable ones; in that case the CLI rejects it (No agents found).
    if (expected.size === 0) {
      if (r.exitCode !== 0) { passed++; continue; }
      failed.push(`${roleId}+${prov}(expected reject, exit=${r.exitCode})`);
      continue;
    }
    if (r.exitCode === 0 && missing.length === 0 && extra.length === 0) {
      passed++;
    } else {
      failed.push(`${roleId}+${prov}(missing:[${missing.join(",")}] extra:[${extra.join(",")}] exit=${r.exitCode})`);
    }
  }
  if (failed.length === 0) {
    ok(`D14f valid role×provider identity: ${passed}/${combos.length} combos export exactly the expected agent set`);
  } else {
    fail(`D14f ${failed.length}/${combos.length} combo(s) export wrong agent set:\n  ${failed.join("\n  ")}`);
  }
}

// D14e: Provider-standalone identity — for EVERY provider, `--provider <p> --all`
// exports exactly the claude-code-capable agents whose provider==p (count +
// identity). Strengthens D14b (leak-only) and generalizes B5/B6 (nvidia-only).
{
  const r0 = run(["--list-providers"]);
  const allProviders = r0.stdout.trim().split("\n").map((l) => l.split(/\s/)[0]).filter(Boolean);
  let passed = 0;
  const failed = [];
  for (const prov of allProviders) {
    const expected = new Set(
      agents.filter((a) => a.provider === prov && Array.isArray(a.harnesses) && a.harnesses.includes("claude-code")).map((a) => a.id)
    );
    const r = run(["--platform", "claude-code", "--provider", prov, "--all", "--dry-run"]);
    const got = new Set(extractAgentIds(r.stdout));
    const missing = [...expected].filter((id) => !got.has(id));
    const extra = [...got].filter((id) => !expected.has(id));
    if (expected.size === 0) {
      // provider exists in catalog but has no claude-code-capable agent → reject
      if (r.exitCode !== 0) { passed++; continue; }
      failed.push(`${prov}(expected reject for 0 cc-agents, exit=${r.exitCode})`);
      continue;
    }
    if (r.exitCode === 0 && missing.length === 0 && extra.length === 0) {
      passed++;
    } else {
      failed.push(`${prov}(missing:[${missing.join(",")}] extra:[${extra.join(",")}] exit=${r.exitCode})`);
    }
  }
  if (failed.length === 0) {
    ok(`D14e provider-standalone identity: ${passed}/${allProviders.length} providers export exactly their claude-code agents`);
  } else {
    fail(`D14e ${failed.length}/${allProviders.length} provider(s) export wrong agent set:\n  ${failed.join("\n  ")}`);
  }
}

// D15: --provider "" (empty string) is rejected with a provider-specific error message.
// Regression guard — empty string was falsy in JS and bypassed all provider validation,
// exporting ALL providers' content. Guard also checks error message so an unrelated
// failure (e.g. missing role) can't produce a false pass.
{
  const r = run(["--platform", "claude-code", "--role", "cloud-security-engineer", "--provider", ""]);
  const combinedOutput = r.stdout + r.stderr;
  if (r.exitCode !== 0 && /provider/i.test(combinedOutput)) {
    ok("D15 --provider \"\" is rejected with non-zero exit and 'provider' in error (falsy bypass guard)");
  } else {
    fail(`D15 --provider \"\" should be rejected with 'provider' in error; exit=${r.exitCode} hasProviderMsg=${/provider/i.test(combinedOutput)}`);
  }
}

// ── E. Dry-run completeness ───────────────────────────────────────────────────

// E15: claude-code dry-run emits both agent and skill lines.
{
  const r = run(["--platform", "claude-code", "--role", "cloud-security-engineer", "--provider", "aws", "--dry-run"]);
  const agentCount = (r.stdout.match(/^export agent:/gm) || []).length;
  const skillCount = (r.stdout.match(/^export skill:/gm) || []).length;
  if (r.exitCode === 0 && agentCount > 0 && skillCount > 0) {
    ok(`E15 --dry-run on claude-code emits both agents (${agentCount}) and skills (${skillCount})`);
  } else {
    fail(`E15 --dry-run missing lines: agents=${agentCount} skills=${skillCount} exit=${r.exitCode}`);
  }
}

// E16: --dry-run --no-skills emits agent lines but no skill lines.
{
  const r = run(["--platform", "claude-code", "--role", "cloud-security-engineer", "--provider", "aws", "--dry-run", "--no-skills"]);
  const agentCount = (r.stdout.match(/^export agent:/gm) || []).length;
  const skillCount = (r.stdout.match(/^export skill:/gm) || []).length;
  if (r.exitCode === 0 && agentCount > 0 && skillCount === 0) {
    ok(`E16 --dry-run --no-skills: ${agentCount} agent line(s), 0 skill lines`);
  } else {
    fail(`E16 expected 0 skill lines with --no-skills, got ${skillCount}; agents=${agentCount}`);
  }
}

// E17: cursor (skill-unsupported) dry-run shows agent lines but no skill lines.
{
  const r = run(["--platform", "cursor", "--role", "cloud-security-engineer", "--provider", "aws", "--dry-run"]);
  const agentCount = (r.stdout.match(/^export agent:/gm) || []).length;
  const skillCount = (r.stdout.match(/^export skill:/gm) || []).length;
  if (r.exitCode === 0 && agentCount > 0 && skillCount === 0) {
    ok(`E17 cursor --dry-run: ${agentCount} agent line(s), 0 skill lines (skill-unsupported platform)`);
  } else {
    fail(`E17 cursor should emit 0 skill lines, got ${skillCount}; agents=${agentCount}`);
  }
}

// E18: Dry-run stderr summary reports skill count on skill-capable platform.
{
  const r = run(["--platform", "claude-code", "--role", "cloud-security-engineer", "--provider", "aws", "--dry-run"]);
  if (r.exitCode === 0 && /\d+ skill\(s\)/.test(r.stderr)) {
    ok("E18 --dry-run stderr summary includes skill count");
  } else {
    fail(`E18 --dry-run stderr missing skill count; stderr: ${r.stderr.slice(0, 200)}`);
  }
}

// E19: codex --all --dry-run emits both agents and companion skills.
{
  const r = run(["--platform", "codex", "--all", "--dry-run"]);
  const agentCount = (r.stdout.match(/^export agent:/gm) || []).length;
  const skillCount = (r.stdout.match(/^export skill:/gm) || []).length;
  const codexAgents = agents.filter((a) => Array.isArray(a.harnesses) && a.harnesses.includes("codex"));
  if (r.exitCode === 0 && agentCount === codexAgents.length && skillCount === skills.length && /\d+ skill\(s\)/.test(r.stderr)) {
    ok(`E19 codex --all --dry-run emits agents (${agentCount}) and skills (${skillCount})`);
  } else {
    fail(`E19 codex dry-run expected ${codexAgents.length} agents and ${skills.length} skills, got agents=${agentCount} skills=${skillCount}; exit=${r.exitCode}; stderr=${r.stderr.slice(0, 300)}`);
  }
}

// E20: codex --dry-run --no-skills emits agent lines but no skill lines.
{
  const r = run(["--platform", "codex", "--agents", "aws-iam-least-privilege-review-agent", "--dry-run", "--no-skills"]);
  const agentCount = (r.stdout.match(/^export agent:/gm) || []).length;
  const skillCount = (r.stdout.match(/^export skill:/gm) || []).length;
  if (r.exitCode === 0 && agentCount === 1 && skillCount === 0) {
    ok("E20 codex --dry-run --no-skills: 1 agent line, 0 skill lines");
  } else {
    fail(`E20 expected 1 agent and 0 skills, got agents=${agentCount} skills=${skillCount}; exit=${r.exitCode}`);
  }
}

// ── F. Full CLI flag surface ──────────────────────────────────────────────────

// F19: --list exits 0 and emits one line per agent.
{
  const r = run(["--list"]);
  const lines = r.stdout.trim().split("\n").filter(Boolean);
  if (r.exitCode === 0 && lines.length === agents.length) {
    ok(`F19 --list exits 0 and prints all ${lines.length} agents`);
  } else {
    fail(`F19 --list: expected ${agents.length} lines, got ${lines.length}; exit=${r.exitCode}`);
  }
}

// F20: --list-roles exits 0 and emits one line per role.
{
  const r = run(["--list-roles"]);
  const roleCount = Object.keys(rolesDoc.roles).length;
  const lines = r.stdout.trim().split("\n").filter(Boolean);
  if (r.exitCode === 0 && lines.length === roleCount) {
    ok(`F20 --list-roles exits 0 and prints all ${lines.length} roles`);
  } else {
    fail(`F20 --list-roles: expected ${roleCount} lines, got ${lines.length}; exit=${r.exitCode}`);
  }
}

// F21: --list-providers exits 0 and includes 'aws'.
{
  const r = run(["--list-providers"]);
  if (r.exitCode === 0 && r.stdout.includes("aws")) {
    ok("F21 --list-providers exits 0 and includes 'aws'");
  } else {
    fail(`F21 --list-providers: exit=${r.exitCode} stdout=${r.stdout.slice(0, 100)}`);
  }
}

// F22: --agents <single-id> selects exactly that agent.
{
  const targetId = "aws-iam-least-privilege-review-agent";
  const r = run(["--platform", "claude-code", "--agents", targetId, "--dry-run"]);
  const ids = extractAgentIds(r.stdout);
  if (r.exitCode === 0 && ids.length === 1 && ids[0] === targetId) {
    ok(`F22 --agents <single-id> selects exactly 1 agent`);
  } else {
    fail(`F22 expected [${targetId}], got [${ids.join(", ")}]; exit=${r.exitCode}`);
  }
}

// F23: --agents <id1>,<id2> selects exactly those two agents.
{
  const id1 = "aws-iam-least-privilege-review-agent";
  const id2 = "azure-rbac-review-agent";
  const r = run(["--platform", "claude-code", "--agents", `${id1},${id2}`, "--dry-run"]);
  const ids = new Set(extractAgentIds(r.stdout));
  if (r.exitCode === 0 && ids.size === 2 && ids.has(id1) && ids.has(id2)) {
    ok(`F23 --agents <id1>,<id2> selects exactly those 2 agents`);
  } else {
    fail(`F23 expected [${id1}, ${id2}], got [${[...ids].join(", ")}]; exit=${r.exitCode}`);
  }
}

// F24: --all selects every catalog agent that supports the requested platform.
// Agents authored without a claude-code harness variant (e.g., Salesforce
// Wave 1 entries declaring harnesses: ["other"]) are silently skipped by --all
// so smoke tests don't fail on partial-portfolio waves.
{
  const r = run(["--platform", "claude-code", "--all", "--dry-run"]);
  const agentCount = (r.stdout.match(/^export agent:/gm) || []).length;
  const claudeCodeAgents = agents.filter((a) => Array.isArray(a.harnesses) && a.harnesses.includes("claude-code"));
  if (r.exitCode === 0 && agentCount === claudeCodeAgents.length) {
    ok(`F24 --all selects all ${agentCount} claude-code-capable agents (of ${agents.length} total)`);
  } else {
    fail(`F24 --all: expected ${claudeCodeAgents.length} claude-code-capable agents, got ${agentCount}; exit=${r.exitCode}`);
  }
}

// F25: Platform alias --platform claude resolves to claude-code.
{
  const r = run(["--platform", "claude", "--role", "cloud-security-engineer", "--provider", "aws", "--dry-run"]);
  const agentCount = (r.stdout.match(/^export agent:/gm) || []).length;
  if (r.exitCode === 0 && agentCount > 0) {
    ok(`F25 --platform claude alias resolves to claude-code (${agentCount} agents)`);
  } else {
    fail(`F25 --platform claude alias failed; exit=${r.exitCode} agents=${agentCount}`);
  }
}

// F26: --no-skills writes agent file but skips skills directory (real write).
{
  const tmpDir = fs.mkdtempSync(path.join(os.tmpdir(), "vfa-test-noskills-"));
  try {
    const r = run(["--platform", "claude-code", "--agents", "aws-iam-least-privilege-review-agent", "--repo", tmpDir, "--no-skills"]);
    const agentsDir = path.join(tmpDir, ".claude", "agents");
    const skillsDir = path.join(tmpDir, ".claude", "skills");
    if (r.exitCode === 0 && fs.existsSync(agentsDir) && !fs.existsSync(skillsDir)) {
      ok("F26 --no-skills writes agent file but skips .claude/skills directory");
    } else {
      fail(`F26 --no-skills: exit=${r.exitCode} agentsDir=${fs.existsSync(agentsDir)} skillsDir=${fs.existsSync(skillsDir)}`);
    }
  } finally {
    fs.rmSync(tmpDir, { recursive: true, force: true });
  }
}

// F27: --force overwrites existing agent files without error.
{
  const tmpDir = fs.mkdtempSync(path.join(os.tmpdir(), "vfa-test-force-"));
  try {
    const args = ["--platform", "claude-code", "--agents", "aws-iam-least-privilege-review-agent", "--repo", tmpDir, "--no-skills"];
    run(args); // first write
    const r2 = run([...args, "--force"]); // overwrite with --force
    if (r2.exitCode === 0) {
      ok("F27 --force overwrites existing files without error");
    } else {
      fail(`F27 --force: exit=${r2.exitCode}\nstderr: ${r2.stderr.slice(0, 300)}`);
    }
  } finally {
    fs.rmSync(tmpDir, { recursive: true, force: true });
  }
}

// F28: codex real write installs agent + companion skill and rewrites skill path.
{
  const tmpDir = fs.mkdtempSync(path.join(os.tmpdir(), "vfa-test-codex-skills-"));
  try {
    const r = run(["--platform", "codex", "--agents", "aws-iam-least-privilege-review-agent", "--repo", tmpDir]);
    const agentFile = path.join(tmpDir, ".codex", "agents", "aws-iam-least-privilege-review-agent.toml");
    const skillDir = path.join(tmpDir, ".codex", "skills", "aws-iam-least-privilege-review");
    const skillFile = path.join(skillDir, "SKILL.md");
    const agentText = fs.existsSync(agentFile) ? fs.readFileSync(agentFile, "utf8") : "";
    const expectedPathLine = `path = ${JSON.stringify(skillDir)}`;
    if (
      r.exitCode === 0 &&
      fs.existsSync(agentFile) &&
      fs.existsSync(skillFile) &&
      agentText.includes(expectedPathLine) &&
      !agentText.includes('path = "skills/aws/aws-iam-least-privilege-review/SKILL.md"')
    ) {
      ok("F28 codex export writes agent + skill and rewrites skill path to installed folder");
    } else {
      fail(`F28 codex export invalid: exit=${r.exitCode} agent=${fs.existsSync(agentFile)} skill=${fs.existsSync(skillFile)} hasExpectedPath=${agentText.includes(expectedPathLine)} stderr=${r.stderr.slice(0, 300)}`);
    }
  } finally {
    fs.rmSync(tmpDir, { recursive: true, force: true });
  }
}

// F28b: skill bundling refuses destination symlinks even with --force.
{
  const tmpDir = fs.mkdtempSync(path.join(os.tmpdir(), "vfa-test-skill-symlink-"));
  const outsideDir = fs.mkdtempSync(path.join(os.tmpdir(), "vfa-test-outside-"));
  const outsideFile = path.join(outsideDir, "OUTSIDE_SENTINEL");
  try {
    const skillDir = path.join(tmpDir, ".codex", "skills", "aws-iam-least-privilege-review");
    fs.mkdirSync(skillDir, { recursive: true });
    fs.writeFileSync(outsideFile, "ORIGINAL_OUTSIDE_SENTINEL\n");
    fs.symlinkSync(outsideFile, path.join(skillDir, "SKILL.md"));
    const r = run([
      "--platform", "codex",
      "--agents", "aws-iam-least-privilege-review-agent",
      "--repo", tmpDir,
      "--force",
    ]);
    const outsideText = fs.readFileSync(outsideFile, "utf8");
    if (r.exitCode !== 0 && /symbolic link destination in skill tree/i.test(r.stderr) && outsideText === "ORIGINAL_OUTSIDE_SENTINEL\n") {
      ok("F28b codex skill export rejects destination symlink and preserves outside file");
    } else {
      fail(`F28b expected symlink rejection without outside overwrite; exit=${r.exitCode} preserved=${outsideText === "ORIGINAL_OUTSIDE_SENTINEL\n"} stderr=${r.stderr.slice(0, 300)}`);
    }
  } finally {
    fs.rmSync(tmpDir, { recursive: true, force: true });
    fs.rmSync(outsideDir, { recursive: true, force: true });
  }
}

// F28c: codex export rejects symlinked platform directory ancestors.
{
  const tmpDir = fs.mkdtempSync(path.join(os.tmpdir(), "vfa-test-codex-parent-symlink-"));
  const outsideDir = fs.mkdtempSync(path.join(os.tmpdir(), "vfa-test-outside-"));
  try {
    fs.symlinkSync(outsideDir, path.join(tmpDir, ".codex"));
    const r = run([
      "--platform", "codex",
      "--agents", "aws-iam-least-privilege-review-agent",
      "--repo", tmpDir,
      "--force",
    ]);
    const outsideAgent = path.join(outsideDir, "agents", "aws-iam-least-privilege-review-agent.toml");
    const outsideSkill = path.join(outsideDir, "skills", "aws-iam-least-privilege-review", "SKILL.md");
    if (
      r.exitCode !== 0 &&
      /symbolic link ancestor/i.test(r.stderr) &&
      !fs.existsSync(outsideAgent) &&
      !fs.existsSync(outsideSkill)
    ) {
      ok("F28c codex export rejects symlinked .codex ancestor without outside writes");
    } else {
      fail(`F28c expected symlink ancestor rejection without outside writes; exit=${r.exitCode} agentOutside=${fs.existsSync(outsideAgent)} skillOutside=${fs.existsSync(outsideSkill)} stderr=${r.stderr.slice(0, 300)}`);
    }
  } finally {
    fs.rmSync(tmpDir, { recursive: true, force: true });
    fs.rmSync(outsideDir, { recursive: true, force: true });
  }
}

// F29: two-stage installer dry-run composes marketplace-safe export path.
{
  const tmpDir = fs.mkdtempSync(path.join(os.tmpdir(), "vfa-test-two-stage-"));
  try {
    const r = runInstaller(["--dry-run", "--skip-marketplace", "--repo", tmpDir]);
    const combined = `${r.stdout}
${r.stderr}`;
    const planMatch = /(\d+) agent\(s\), (\d+) skill\(s\) planned/.exec(combined);
    if (r.exitCode === 0 && planMatch && parseInt(planMatch[1], 10) > 0 && parseInt(planMatch[2], 10) > 0) {
      ok(`F29 two-stage Codex installer dry-run plans all agents and skills (${planMatch[1]} agents, ${planMatch[2]} skills)`);
    } else {
      fail(`F29 installer dry-run did not plan expected agents/skills; exit=${r.exitCode} output=${combined.slice(-500)}`);
    }
  } finally {
    fs.rmSync(tmpDir, { recursive: true, force: true });
  }
}

// ── G. Error / rejection cases ────────────────────────────────────────────────

// G28: No args → usage text printed to stderr, non-zero exit.
{
  const r = run([]);
  if (r.exitCode !== 0 && /usage/i.test(r.stderr)) {
    ok("G28 no args → usage text printed to stderr, non-zero exit");
  } else {
    fail(`G28 expected non-zero exit with usage in stderr; exit=${r.exitCode} hasUsage=${/usage/i.test(r.stderr)}`);
  }
}

// G29: Unknown --role → exit non-zero with 'role' in output.
{
  const r = run(["--platform", "claude-code", "--role", "not-a-real-role-xyz"]);
  if (r.exitCode !== 0 && /role/i.test(r.stderr + r.stdout)) {
    ok("G29 unknown --role exits non-zero with 'role' in output");
  } else {
    fail(`G29 expected non-zero for unknown role; exit=${r.exitCode} stderr=${r.stderr.slice(0, 100)}`);
  }
}

// G30: Unknown --platform → exit non-zero with 'platform' in output.
{
  const r = run(["--platform", "definitely-not-a-platform", "--role", "cloud-security-engineer"]);
  if (r.exitCode !== 0 && /platform/i.test(r.stderr + r.stdout)) {
    ok("G30 unknown --platform exits non-zero with 'platform' in output");
  } else {
    fail(`G30 expected non-zero for unknown platform; exit=${r.exitCode}`);
  }
}

// G31: Unknown --agents id → exit non-zero.
{
  const r = run(["--platform", "claude-code", "--agents", "totally-not-a-real-agent-id-999"]);
  if (r.exitCode !== 0) {
    ok("G31 unknown --agents id exits non-zero");
  } else {
    fail(`G31 expected non-zero for unknown agent id; exit=${r.exitCode}`);
  }
}

// G32: --platform with no selector → exit non-zero.
{
  const r = run(["--platform", "claude-code"]);
  if (r.exitCode !== 0) {
    ok("G32 --platform with no selector exits non-zero");
  } else {
    fail(`G32 expected non-zero when no selector given; exit=${r.exitCode}`);
  }
}

// G33: --provider=aws (equals-sign inline form) is accepted and works correctly.
// Regression guard for util.parseArgs migration — hand-rolled parsers often
// treat --key=value as an unknown flag and silently call usage(1).
{
  const r = run(["--platform", "claude-code", "--role", "cloud-security-engineer", "--provider=aws", "--dry-run"]);
  const agentCount = (r.stdout.match(/^export agent:/gm) || []).length;
  if (r.exitCode === 0 && agentCount > 0) {
    ok(`G33 --provider=aws (equals-sign form) is accepted (${agentCount} agents)`);
  } else {
    fail(`G33 --provider=aws inline form rejected; exit=${r.exitCode} agents=${agentCount} stderr=${r.stderr.slice(0, 200)}`);
  }
}

// G34: Unicode zero-width space in --provider is rejected by the format regex.
// U+200B passes String.prototype.trim() in some V8 versions but is not in
// [a-z0-9-], so the /^[a-z0-9][a-z0-9-]*$/ gate must catch it.
// (Defense-in-depth: the empty-string guard runs first if trim() strips it.)
{
  const zwsp = "​";
  const r = run(["--platform", "claude-code", "--role", "cloud-security-engineer", "--provider", zwsp]);
  if (r.exitCode !== 0) {
    ok("G34 --provider with Unicode zero-width space is rejected (format gate)");
  } else {
    fail(`G34 Unicode zero-width in --provider should be rejected; exit=${r.exitCode} agents exported=${(r.stdout.match(/^export agent:/gm)||[]).length}`);
  }
}

// G35: Completely unknown flag exits non-zero with an error, not silent usage().
// util.parseArgs strict mode must surface a real error message, not swallow unknowns.
{
  const r = run(["--platform", "claude-code", "--role", "cloud-security-engineer", "--not-a-real-flag"]);
  const combinedOutput = r.stdout + r.stderr;
  if (r.exitCode !== 0 && /unknown|not-a-real-flag/i.test(combinedOutput)) {
    ok("G35 unknown flag exits non-zero with descriptive error");
  } else {
    fail(`G35 unknown flag should produce descriptive error; exit=${r.exitCode} output=${combinedOutput.slice(0, 200)}`);
  }
}

// ── Summary ─────────────────────────────────────────────────────────────────

if (failures > 0) {
  console.error(`\n${failures} check(s) failed`);
  process.exit(1);
}
console.log(`\nOK: all coverage and CLI checks passed`);
