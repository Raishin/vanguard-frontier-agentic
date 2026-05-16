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

// Map: skillName → providerDir (mirrors loadSkills() internal logic)
const skillProviderByName = new Map();
for (const prov of skillProviderDirs) {
  const provDir = path.join(skillsRoot, prov);
  for (const skill of fs.readdirSync(provDir, { withFileTypes: true })) {
    if (skill.isDirectory() && fs.existsSync(path.join(provDir, skill.name, "SKILL.md"))) {
      skillProviderByName.set(skill.name, prov);
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
    return prov !== expectedProvider && prov !== "shared";
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

// F24: --all selects every agent in the catalog.
{
  const r = run(["--platform", "claude-code", "--all", "--dry-run"]);
  const agentCount = (r.stdout.match(/^export agent:/gm) || []).length;
  if (r.exitCode === 0 && agentCount === agents.length) {
    ok(`F24 --all selects all ${agentCount} agents`);
  } else {
    fail(`F24 --all: expected ${agents.length} agents, got ${agentCount}; exit=${r.exitCode}`);
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
