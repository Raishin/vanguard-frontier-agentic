#!/usr/bin/env node
/**
 * Per-harness model + reasoning-effort policy engine for agent harness variants.
 *
 * Canonical intent lives in catalog/model-policy.json. This script projects
 * that policy into the executable harness files (codex.toml `model` /
 * `model_reasoning_effort`, claude-code/cursor `.agent.md` frontmatter
 * `model:`) and regenerates the resolved index catalog/model-assignments.json
 * that read-side tooling (vfa-tui) displays.
 *
 * Policy semantics:
 *   - Scopes: "all", "provider:<id>", "role:<id>", "agent:<id>".
 *   - Precedence: agent > role > provider > all, per field (model and
 *     reasoning_effort resolve independently).
 *   - Two rules in the same tier that disagree on the same agent+harness+field
 *     are a hard error (roles overlap by design, so equal values are allowed;
 *     conflicting values must be settled by an agent-level rule).
 *   - "auto" clears the field: the managed line is removed and the harness
 *     runtime default applies. Absence of any rule means "auto".
 *   - Fields are only projected into harnesses that support them (see
 *     HARNESS_CAPABILITIES). A rule targeting an unsupported field fails
 *     `check` — intent that cannot be enforced is treated as an error, not
 *     silently recorded.
 *
 * Commands:
 *   report [--json]     print resolved assignments per agent x harness
 *   check               validate policy + detect drift (exit 1 on violation)
 *   apply [--dry-run]   project policy into harness files + assignments index
 *   set --scope <all|provider=ID|role=ID|agent=ID|agents=a,b> --harness <id>
 *       [--model <name|auto>] [--reasoning <effort|auto>] [--dry-run]
 *                       upsert rule(s), then apply
 *   import-current [--force]
 *                       seed the policy from the values currently present in
 *                       harness files (bootstrap; refuses to overwrite an
 *                       existing policy without --force)
 *
 * After a non-dry-run apply/set, refresh the integrity manifest:
 *   npm run asset-integrity:write
 */

import { readFileSync, writeFileSync } from "node:fs";
import { createHash } from "node:crypto";
import { dirname, join, relative } from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = join(dirname(fileURLToPath(import.meta.url)), "..");
const policyPath = join(repoRoot, "catalog", "model-policy.json");
const assignmentsPath = join(repoRoot, "catalog", "model-assignments.json");
const agentsCatalogPath = join(repoRoot, "catalog", "agents.json");
const rolesCatalogPath = join(repoRoot, "catalog", "install-roles.json");

/** Which policy fields each harness's executable format can express.
 * Only verified, officially supported keys are projected; inventing metadata
 * fields in executable agent files is forbidden (see CLAUDE.md cross-platform
 * asset rule). Extend this table only with documented harness support. */
const HARNESS_CAPABILITIES = {
  codex: { model: true, reasoning_effort: true, variant: "codex", file: "codex.toml" },
  "claude-code": { model: true, reasoning_effort: false, variant: "claude-code", file: "claude-code.agent.md" },
  cursor: { model: true, reasoning_effort: false, variant: "cursor", file: "cursor.agent.md" },
  copilot: { model: false, reasoning_effort: false, variant: "copilot", file: "copilot.agent.md" },
  gemini: { model: false, reasoning_effort: false, variant: "gemini", file: "gemini.agent.md" },
  kiro: { model: false, reasoning_effort: false, variant: "kiro-ide", file: "kiro-ide.agent.md" },
};

/** Harness-specific model-name shape. All values must also satisfy
 * SAFE_VALUE (no quotes, spaces, or shell metacharacters). */
const MODEL_PATTERNS = {
  codex: /^gpt-[a-z0-9.-]+$/,
  "claude-code": /^(opus|sonnet|haiku|inherit|claude-[a-z0-9.-]+)$/,
  cursor: /^(auto|inherit|[a-z0-9][a-z0-9.-]*)$/,
};

const REASONING_EFFORTS = ["minimal", "low", "medium", "high"];
const SAFE_VALUE = /^[A-Za-z0-9][A-Za-z0-9._-]*$/;
const SCOPE_TIERS = ["all", "provider", "role", "agent"];

// ── shared loading ───────────────────────────────────────────────────────────

/** Read a file as UTF-8, or return null if it does not exist. Avoids the
 * TOCTOU race of existsSync-then-read: a single syscall either returns the
 * contents or reports ENOENT, leaving no window between check and use. */
function readFileOrNull(path) {
  try {
    return readFileSync(path, "utf8");
  } catch (e) {
    if (e && e.code === "ENOENT") return null;
    throw e;
  }
}

function loadJson(path, label) {
  const raw = readFileOrNull(path);
  if (raw === null) {
    fail(2, `ERROR: ${label} not found at ${relative(repoRoot, path)}`);
  }
  try {
    return JSON.parse(raw);
  } catch (e) {
    fail(2, `ERROR: ${label} is not valid JSON: ${e.message}`);
  }
}

/** Load JSON, returning `fallback` when the file is absent. */
function loadJsonOrDefault(path, label, fallback) {
  const raw = readFileOrNull(path);
  if (raw === null) return fallback;
  try {
    return JSON.parse(raw);
  } catch (e) {
    fail(2, `ERROR: ${label} is not valid JSON: ${e.message}`);
  }
}

function fail(code, ...lines) {
  for (const l of lines) console.error(l);
  process.exit(code);
}

function loadAgents() {
  const catalog = loadJson(agentsCatalogPath, "agent catalog");
  return catalog.filter((e) => e.type === "agent");
}

function loadRoles() {
  return loadJson(rolesCatalogPath, "role catalog").roles;
}

function sha256Hex(text) {
  return createHash("sha256").update(text).digest("hex");
}

// ── policy parsing + validation ──────────────────────────────────────────────

function parseScope(scope) {
  if (scope === "all") return { tier: "all", id: null };
  const m = /^(provider|role|agent):([a-z0-9][a-z0-9-]*)$/.exec(scope);
  if (!m) return null;
  return { tier: m[1], id: m[2] };
}

/** Structural + referential validation. Returns a list of error strings. */
function validatePolicy(policy, agents, roles) {
  const errors = [];
  if (!policy || typeof policy !== "object" || Array.isArray(policy)) {
    return ["policy root must be an object"];
  }
  if (policy.manifest_version !== 1) {
    errors.push("manifest_version must be 1");
  }
  if (policy.defaults) {
    if (policy.defaults.model !== "auto" || policy.defaults.reasoning_effort !== "auto") {
      errors.push('defaults must be {"model": "auto", "reasoning_effort": "auto"}');
    }
  }
  if (!Array.isArray(policy.rules)) {
    errors.push("rules must be an array");
    return errors;
  }
  const agentIds = new Set(agents.map((a) => a.id));
  const providers = new Set(agents.map((a) => a.provider));
  const seen = new Set();
  policy.rules.forEach((rule, i) => {
    const where = `rules[${i}]`;
    const known = new Set(["scope", "harness", "model", "reasoning_effort"]);
    for (const k of Object.keys(rule)) {
      if (!known.has(k)) errors.push(`${where}: unknown field "${k}"`);
    }
    const scope = typeof rule.scope === "string" ? parseScope(rule.scope) : null;
    if (!scope) {
      errors.push(`${where}: invalid scope "${rule.scope}"`);
      return;
    }
    const caps = HARNESS_CAPABILITIES[rule.harness];
    if (!caps) {
      errors.push(`${where}: unknown harness "${rule.harness}"`);
      return;
    }
    if (rule.model === undefined && rule.reasoning_effort === undefined) {
      errors.push(`${where}: must set model and/or reasoning_effort`);
    }
    if (rule.model !== undefined) {
      if (!caps.model) {
        errors.push(`${where}: harness "${rule.harness}" does not support model pinning`);
      } else if (rule.model !== "auto") {
        if (typeof rule.model !== "string" || !SAFE_VALUE.test(rule.model)) {
          errors.push(`${where}: unsafe model value "${rule.model}"`);
        } else if (MODEL_PATTERNS[rule.harness] && !MODEL_PATTERNS[rule.harness].test(rule.model)) {
          errors.push(
            `${where}: model "${rule.model}" does not match the allowed pattern for ${rule.harness} (${MODEL_PATTERNS[rule.harness]})`,
          );
        }
      }
    }
    if (rule.reasoning_effort !== undefined) {
      if (!caps.reasoning_effort) {
        errors.push(`${where}: harness "${rule.harness}" does not support reasoning_effort`);
      } else if (rule.reasoning_effort !== "auto" && !REASONING_EFFORTS.includes(rule.reasoning_effort)) {
        errors.push(
          `${where}: reasoning_effort "${rule.reasoning_effort}" must be auto|${REASONING_EFFORTS.join("|")}`,
        );
      }
    }
    if (scope.tier === "provider" && !providers.has(scope.id)) {
      errors.push(`${where}: unknown provider "${scope.id}"`);
    }
    if (scope.tier === "role" && !roles[scope.id]) {
      errors.push(`${where}: unknown role "${scope.id}"`);
    }
    if (scope.tier === "agent" && !agentIds.has(scope.id)) {
      errors.push(`${where}: unknown agent "${scope.id}"`);
    }
    const dupKey = `${rule.scope}::${rule.harness}`;
    if (seen.has(dupKey)) {
      errors.push(`${where}: duplicate rule for scope "${rule.scope}" harness "${rule.harness}"`);
    }
    seen.add(dupKey);
  });
  return errors;
}

// ── resolution ───────────────────────────────────────────────────────────────

/** Resolve effective {model, reasoning_effort, sources} for one agent+harness.
 *
 * Only the highest-priority tier (agent > role > provider > all) that actually
 * matches a field decides that field. A conflict is reported *only* when the
 * winning tier itself carries two different values — lower-tier disagreements
 * are irrelevant because a higher tier overrides them. This preserves the
 * documented escape hatch: adding a single `agent:<id>` rule resolves a
 * role-overlap conflict instead of being rejected alongside it. */
function resolveAgentHarness(agent, harness, policy, roleIndex, errors) {
  const result = {
    model: "auto",
    reasoning_effort: "auto",
    model_source: "default",
    reasoning_source: "default",
  };
  // Collect matching rules per field, grouped by tier.
  const hitsByTier = { model: new Map(), reasoning_effort: new Map() };
  for (const rule of policy.rules) {
    if (rule.harness !== harness) continue;
    const scope = parseScope(rule.scope);
    if (!scope) continue;
    const matches =
      scope.tier === "all" ||
      (scope.tier === "provider" && scope.id === agent.provider) ||
      (scope.tier === "role" && (roleIndex.get(scope.id) || new Set()).has(agent.id)) ||
      (scope.tier === "agent" && scope.id === agent.id);
    if (!matches) continue;
    for (const field of ["model", "reasoning_effort"]) {
      if (rule[field] === undefined) continue;
      if (!hitsByTier[field].has(scope.tier)) hitsByTier[field].set(scope.tier, []);
      hitsByTier[field].get(scope.tier).push({ rule, value: rule[field] });
    }
  }
  for (const field of ["model", "reasoning_effort"]) {
    // Highest-priority tier with any matching rule wins outright.
    let winningTier = null;
    for (const tier of SCOPE_TIERS) {
      if (hitsByTier[field].has(tier)) winningTier = tier;
    }
    if (winningTier === null) continue;
    const hits = hitsByTier[field].get(winningTier);
    const values = new Set(hits.map((h) => h.value));
    if (values.size > 1) {
      errors.push(
        `conflict: agent "${agent.id}" harness "${harness}" gets ${field} values ` +
          `[${[...values].join(", ")}] from ${winningTier}-tier rules ` +
          `(${hits.map((h) => h.rule.scope).join(", ")}); add an agent-level override`,
      );
      continue;
    }
    result[field] = hits[0].value;
    const src = field === "model" ? "model_source" : "reasoning_source";
    result[src] = hits[0].rule.scope;
  }
  return result;
}

function buildRoleIndex(roles) {
  const index = new Map();
  for (const [roleId, role] of Object.entries(roles)) {
    index.set(roleId, new Set(role.agents || []));
  }
  return index;
}

/** Resolve the full assignment table. Returns { assignments, errors }. */
function resolveAll(policy, agents, roles) {
  const errors = [];
  const roleIndex = buildRoleIndex(roles);
  const assignments = [];
  const sorted = [...agents].sort((a, b) => a.id.localeCompare(b.id));
  for (const agent of sorted) {
    for (const [harness, caps] of Object.entries(HARNESS_CAPABILITIES)) {
      if (!caps.model && !caps.reasoning_effort) continue;
      const variantRel =
        agent.harness_variants?.[caps.variant] ?? `${agent.path}/harnesses/${caps.file}`;
      if (readFileOrNull(join(repoRoot, variantRel)) === null) continue;
      const r = resolveAgentHarness(agent, harness, policy, roleIndex, errors);
      assignments.push({
        agent_id: agent.id,
        harness,
        model: r.model === "auto" ? null : r.model,
        reasoning_effort: caps.reasoning_effort
          ? r.reasoning_effort === "auto"
            ? null
            : r.reasoning_effort
          : null,
        model_source: r.model_source,
        reasoning_source: caps.reasoning_effort ? r.reasoning_source : "default",
        file: variantRel,
      });
    }
  }
  return { assignments, errors };
}

// ── surgical file editing ────────────────────────────────────────────────────

/** Set/replace/remove a managed top-level key in a codex.toml file.
 * Only lines before the first table header ([...]) and outside triple-quoted
 * strings are considered, so prose inside developer_instructions can never be
 * mistaken for a config key. `value === null` removes the line. */
function editTomlKey(content, key, value) {
  const lines = content.split("\n");
  let inTriple = false;
  let topLevelEnd = lines.length;
  const keyRe = new RegExp(`^${key}\\s*=`);
  let keyLine = -1;
  let nameLine = -1;
  let descLine = -1;
  for (let i = 0; i < lines.length; i++) {
    const tripleCount = (lines[i].match(/"""/g) || []).length;
    if (!inTriple) {
      if (/^\[/.test(lines[i])) {
        topLevelEnd = i;
        break;
      }
      if (keyRe.test(lines[i]) && keyLine === -1) keyLine = i;
      if (/^name\s*=/.test(lines[i])) nameLine = i;
      if (/^description\s*=/.test(lines[i]) && descLine === -1) descLine = i;
    }
    if (tripleCount % 2 === 1) inTriple = !inTriple;
  }
  const rendered = value === null ? null : `${key} = "${value}"`;
  if (keyLine >= 0) {
    if (rendered === null) {
      lines.splice(keyLine, 1);
    } else if (lines[keyLine] !== rendered) {
      lines[keyLine] = rendered;
    } else {
      return content;
    }
    return lines.join("\n");
  }
  if (rendered === null) return content;
  // Insert after `model =` for the reasoning key, else after description/name.
  let anchor = descLine >= 0 ? descLine : nameLine;
  if (key === "model_reasoning_effort") {
    for (let i = 0; i < topLevelEnd; i++) {
      if (/^model\s*=/.test(lines[i])) {
        anchor = i;
        break;
      }
    }
  }
  lines.splice(anchor + 1, 0, rendered);
  return lines.join("\n");
}

/** Set/replace/remove the managed `model:` key in a YAML frontmatter block.
 * Only the block between the leading `---` fence pair is touched.
 * `value === null` removes the line. */
function editFrontmatterModel(content, value) {
  const lines = content.split("\n");
  if (lines[0] !== "---") return content;
  let close = -1;
  for (let i = 1; i < lines.length; i++) {
    if (lines[i] === "---") {
      close = i;
      break;
    }
  }
  if (close === -1) return content;
  const rendered = value === null ? null : `model: "${value}"`;
  for (let i = 1; i < close; i++) {
    if (/^model:/.test(lines[i])) {
      if (rendered === null) {
        lines.splice(i, 1);
      } else if (lines[i] !== rendered) {
        lines[i] = rendered;
      } else {
        return content;
      }
      return lines.join("\n");
    }
  }
  if (rendered === null) return content;
  lines.splice(close, 0, rendered);
  return lines.join("\n");
}

/** Compute the projected content for one assignment. */
function projectFile(content, assignment) {
  if (assignment.harness === "codex") {
    let next = editTomlKey(content, "model", assignment.model);
    next = editTomlKey(next, "model_reasoning_effort", assignment.reasoning_effort);
    return next;
  }
  return editFrontmatterModel(content, assignment.model);
}

// ── assignments index ────────────────────────────────────────────────────────

function renderAssignments(policyText, assignments) {
  const capabilities = {};
  for (const [harness, caps] of Object.entries(HARNESS_CAPABILITIES)) {
    capabilities[harness] = { model: caps.model, reasoning_effort: caps.reasoning_effort };
  }
  const doc = {
    manifest_version: 1,
    generated_by: "scripts/model-policy.mjs",
    policy_sha256: sha256Hex(policyText),
    capabilities,
    assignments: assignments.map(({ file, ...rest }) => rest),
  };
  return JSON.stringify(doc, null, 2) + "\n";
}

function canonicalPolicyText(policy) {
  const tierRank = Object.fromEntries(SCOPE_TIERS.map((t, i) => [t, i]));
  const rules = [...policy.rules].sort((a, b) => {
    if (a.harness !== b.harness) return a.harness.localeCompare(b.harness);
    const ta = parseScope(a.scope);
    const tb = parseScope(b.scope);
    if (tierRank[ta.tier] !== tierRank[tb.tier]) return tierRank[ta.tier] - tierRank[tb.tier];
    return a.scope.localeCompare(b.scope);
  });
  const doc = {
    manifest_version: 1,
    description:
      "Per-harness model and reasoning-effort policy for agent harness variants. " +
      "Managed by scripts/model-policy.mjs (vfa-tui Model Policy view or CLI). " +
      "'auto' omits the field so the harness runtime default applies. " +
      "Precedence: agent > role > provider > all.",
    defaults: { model: "auto", reasoning_effort: "auto" },
    rules: rules.map((r) => {
      const out = { scope: r.scope, harness: r.harness };
      if (r.model !== undefined) out.model = r.model;
      if (r.reasoning_effort !== undefined) out.reasoning_effort = r.reasoning_effort;
      return out;
    }),
  };
  return JSON.stringify(doc, null, 2) + "\n";
}

// ── plan / apply ─────────────────────────────────────────────────────────────

/** Compute the full projection plan: file changes + new assignments text. */
function computePlan(policy, agents, roles) {
  const { assignments, errors } = resolveAll(policy, agents, roles);
  const changes = [];
  for (const a of assignments) {
    const abs = join(repoRoot, a.file);
    const current = readFileSync(abs, "utf8");
    const next = projectFile(current, a);
    if (next !== current) {
      changes.push({ file: a.file, assignment: a, next });
    }
  }
  return { assignments, errors, changes };
}

function describeChange(c) {
  const model = c.assignment.model ?? "auto";
  const reasoning = c.assignment.reasoning_effort ?? "auto";
  const detail =
    c.assignment.harness === "codex" ? `model=${model} reasoning=${reasoning}` : `model=${model}`;
  return `file update: ${c.file} (${detail})`;
}

function runApply({ dryRun }) {
  const agents = loadAgents();
  const roles = loadRoles();
  const policy = loadJson(policyPath, "model policy");
  const structuralErrors = validatePolicy(policy, agents, roles);
  if (structuralErrors.length > 0) {
    fail(1, "ERROR: model policy is invalid:", ...structuralErrors.map((e) => "  " + e));
  }
  const { assignments, errors, changes } = computePlan(policy, agents, roles);
  if (errors.length > 0) {
    fail(1, "ERROR: model policy conflicts:", ...[...new Set(errors)].map((e) => "  " + e));
  }
  const policyText = canonicalPolicyText(policy);
  const assignmentsText = renderAssignments(policyText, assignments);
  const assignmentsStale = readFileOrNull(assignmentsPath) !== assignmentsText;

  for (const c of changes) console.log(describeChange(c));
  if (dryRun) {
    console.log(
      `dry-run: ${changes.length} file(s) would change; assignments index ${assignmentsStale ? "would be rewritten" : "already in sync"}`,
    );
    return;
  }
  for (const c of changes) writeFileSync(join(repoRoot, c.file), c.next);
  const currentPolicyText = readFileSync(policyPath, "utf8");
  if (currentPolicyText !== policyText) writeFileSync(policyPath, policyText);
  if (assignmentsStale) writeFileSync(assignmentsPath, assignmentsText);
  console.log(
    `OK: applied model policy (${changes.length} file(s) changed, ${assignments.length} assignments)`,
  );
  if (changes.length > 0 || assignmentsStale) {
    console.log("reminder: run `npm run asset-integrity:write` before committing");
  }
}

function runCheck() {
  const agents = loadAgents();
  const roles = loadRoles();
  const policy = loadJson(policyPath, "model policy");
  const structuralErrors = validatePolicy(policy, agents, roles);
  if (structuralErrors.length > 0) {
    fail(1, "ERROR: model policy is invalid:", ...structuralErrors.map((e) => "  " + e));
  }
  const { assignments, errors, changes } = computePlan(policy, agents, roles);
  const problems = [...new Set(errors)];
  for (const c of changes) {
    problems.push(`drift: ${c.file} does not match the model policy`);
  }
  const policyText = canonicalPolicyText(policy);
  const assignmentsText = renderAssignments(policyText, assignments);
  const currentAssignments = readFileOrNull(assignmentsPath);
  if (currentAssignments === null) {
    problems.push("catalog/model-assignments.json is missing; run npm run model-policy:apply");
  } else if (currentAssignments !== assignmentsText) {
    problems.push("catalog/model-assignments.json is stale; run npm run model-policy:apply");
  }
  if (problems.length > 0) {
    fail(
      1,
      "ERROR: model policy check failed:",
      ...problems.map((p) => "  " + p),
      "fix: adjust catalog/model-policy.json or run npm run model-policy:apply",
    );
  }
  console.log(
    `OK: model policy in sync (${policy.rules.length} rules, ${assignments.length} assignments)`,
  );
}

function runReport({ json }) {
  const agents = loadAgents();
  const roles = loadRoles();
  const policy = loadJson(policyPath, "model policy");
  const structuralErrors = validatePolicy(policy, agents, roles);
  if (structuralErrors.length > 0) {
    fail(1, "ERROR: model policy is invalid:", ...structuralErrors.map((e) => "  " + e));
  }
  const { assignments, errors } = resolveAll(policy, agents, roles);
  if (errors.length > 0) {
    fail(1, "ERROR: model policy conflicts:", ...[...new Set(errors)].map((e) => "  " + e));
  }
  if (json) {
    console.log(renderAssignments(canonicalPolicyText(policy), assignments).trimEnd());
    return;
  }
  for (const a of assignments) {
    const model = a.model ?? "auto";
    const reasoning = a.reasoning_effort ?? "auto";
    console.log(`${a.agent_id} ${a.harness} model=${model} reasoning=${reasoning}`);
  }
}

// ── set ──────────────────────────────────────────────────────────────────────

function parseSetArgs(argv) {
  const args = { scopes: [], harness: null, model: undefined, reasoning: undefined, dryRun: false };
  for (let i = 0; i < argv.length; i++) {
    const a = argv[i];
    if (a === "--dry-run") {
      args.dryRun = true;
    } else if (a === "--scope") {
      const v = argv[++i];
      if (v === undefined) fail(2, "ERROR: --scope requires a value");
      if (v === "all") {
        args.scopes.push("all");
      } else {
        const m = /^(provider|role|agent|agents)=(.+)$/.exec(v);
        if (!m) fail(2, `ERROR: invalid --scope "${v}"`);
        if (m[1] === "agents") {
          for (const id of m[2].split(",").filter(Boolean)) args.scopes.push(`agent:${id}`);
        } else {
          args.scopes.push(`${m[1]}:${m[2]}`);
        }
      }
    } else if (a === "--harness") {
      args.harness = argv[++i];
    } else if (a === "--model") {
      args.model = argv[++i];
    } else if (a === "--reasoning") {
      args.reasoning = argv[++i];
    } else {
      fail(2, `ERROR: unknown argument "${a}"`);
    }
  }
  if (args.scopes.length === 0) fail(2, "ERROR: --scope is required");
  if (!args.harness) fail(2, "ERROR: --harness is required");
  if (args.model === undefined && args.reasoning === undefined) {
    fail(2, "ERROR: provide --model and/or --reasoning");
  }
  return args;
}

function runSet(argv) {
  const args = parseSetArgs(argv);
  const agents = loadAgents();
  const roles = loadRoles();
  const policy = loadJsonOrDefault(policyPath, "model policy", {
    manifest_version: 1,
    rules: [],
  });

  for (const scope of args.scopes) {
    const existing = policy.rules.find((r) => r.scope === scope && r.harness === args.harness);
    const rule = existing ?? { scope, harness: args.harness };
    if (args.model !== undefined) rule.model = args.model;
    if (args.reasoning !== undefined) rule.reasoning_effort = args.reasoning;
    if (!existing) policy.rules.push(rule);
    console.log(
      `policy rule: ${scope} ${args.harness}` +
        (args.model !== undefined ? ` model=${args.model}` : "") +
        (args.reasoning !== undefined ? ` reasoning=${args.reasoning}` : ""),
    );
  }

  const structuralErrors = validatePolicy(policy, agents, roles);
  if (structuralErrors.length > 0) {
    fail(1, "ERROR: resulting policy would be invalid:", ...structuralErrors.map((e) => "  " + e));
  }
  const { assignments, errors, changes } = computePlan(policy, agents, roles);
  if (errors.length > 0) {
    fail(1, "ERROR: resulting policy has conflicts:", ...[...new Set(errors)].map((e) => "  " + e));
  }
  for (const c of changes) console.log(describeChange(c));
  if (args.dryRun) {
    console.log(`dry-run: ${changes.length} file(s) would change; policy not written`);
    return;
  }
  const policyText = canonicalPolicyText(policy);
  writeFileSync(policyPath, policyText);
  for (const c of changes) writeFileSync(join(repoRoot, c.file), c.next);
  writeFileSync(assignmentsPath, renderAssignments(policyText, assignments));
  console.log(
    `OK: policy updated (${policy.rules.length} rules), ${changes.length} file(s) changed`,
  );
  console.log("reminder: run `npm run asset-integrity:write` before committing");
}

// ── import-current ───────────────────────────────────────────────────────────

function readCurrentValue(content, harness, key) {
  if (harness === "codex") {
    const lines = content.split("\n");
    let inTriple = false;
    const keyRe = new RegExp(`^${key}\\s*=\\s*"(.*)"\\s*$`);
    for (const line of lines) {
      const tripleCount = (line.match(/"""/g) || []).length;
      if (!inTriple) {
        if (/^\[/.test(line)) break;
        const m = keyRe.exec(line);
        if (m) return m[1];
      }
      if (tripleCount % 2 === 1) inTriple = !inTriple;
    }
    return "auto";
  }
  const lines = content.split("\n");
  if (lines[0] !== "---") return "auto";
  for (let i = 1; i < lines.length; i++) {
    if (lines[i] === "---") break;
    const m = /^model:\s*"?([^"]*)"?\s*$/.exec(lines[i]);
    if (m) return m[1];
  }
  return "auto";
}

/** Pick the most common value; ties prefer "auto", then lexicographic. */
function majority(counter) {
  let best = null;
  for (const [value, count] of [...counter.entries()].sort((a, b) => {
    if (b[1] !== a[1]) return b[1] - a[1];
    if (a[0] === "auto") return -1;
    if (b[0] === "auto") return 1;
    return a[0].localeCompare(b[0]);
  })) {
    best = { value, count };
    break;
  }
  return best;
}

/** Derive a minimal rule set (all -> provider -> agent) reproducing the
 * observed per-agent values for one harness+field. */
function minimizeRules(observed, harness, field) {
  const rules = [];
  const global = new Map();
  for (const { value } of observed) global.set(value, (global.get(value) || 0) + 1);
  const globalBest = majority(global);
  const allValue = globalBest ? globalBest.value : "auto";
  if (allValue !== "auto") {
    rules.push({ scope: "all", harness, [field]: allValue });
  }
  const byProvider = new Map();
  for (const o of observed) {
    if (!byProvider.has(o.provider)) byProvider.set(o.provider, []);
    byProvider.get(o.provider).push(o);
  }
  for (const [provider, entries] of [...byProvider.entries()].sort()) {
    const counter = new Map();
    for (const { value } of entries) counter.set(value, (counter.get(value) || 0) + 1);
    const best = majority(counter);
    let effective = allValue;
    if (best.value !== allValue && best.count > entries.length / 2) {
      rules.push({ scope: `provider:${provider}`, harness, [field]: best.value });
      effective = best.value;
    }
    for (const o of entries.sort((a, b) => a.agent_id.localeCompare(b.agent_id))) {
      if (o.value !== effective) {
        rules.push({ scope: `agent:${o.agent_id}`, harness, [field]: o.value });
      }
    }
  }
  return rules;
}

function runImportCurrent({ force, dryRun }) {
  if (readFileOrNull(policyPath) !== null && !force) {
    fail(2, "ERROR: catalog/model-policy.json already exists; pass --force to regenerate");
  }
  const agents = loadAgents();
  const roles = loadRoles();
  const rawRules = [];
  for (const [harness, caps] of Object.entries(HARNESS_CAPABILITIES)) {
    if (!caps.model && !caps.reasoning_effort) continue;
    const observedModel = [];
    const observedReasoning = [];
    for (const agent of agents) {
      const variantRel =
        agent.harness_variants?.[caps.variant] ?? `${agent.path}/harnesses/${caps.file}`;
      const abs = join(repoRoot, variantRel);
      const content = readFileOrNull(abs);
      if (content === null) continue;
      if (caps.model) {
        observedModel.push({
          agent_id: agent.id,
          provider: agent.provider,
          value: readCurrentValue(content, harness, "model"),
        });
      }
      if (caps.reasoning_effort) {
        observedReasoning.push({
          agent_id: agent.id,
          provider: agent.provider,
          value: readCurrentValue(content, harness, "model_reasoning_effort"),
        });
      }
    }
    if (caps.model) rawRules.push(...minimizeRules(observedModel, harness, "model"));
    if (caps.reasoning_effort) {
      rawRules.push(...minimizeRules(observedReasoning, harness, "reasoning_effort"));
    }
  }
  // Merge model + reasoning rules that share scope+harness.
  const merged = new Map();
  for (const r of rawRules) {
    const key = `${r.scope}::${r.harness}`;
    merged.set(key, { ...(merged.get(key) || {}), ...r });
  }
  const policy = { manifest_version: 1, rules: [...merged.values()] };
  const structuralErrors = validatePolicy(policy, agents, roles);
  if (structuralErrors.length > 0) {
    fail(1, "ERROR: imported policy is invalid:", ...structuralErrors.map((e) => "  " + e));
  }
  const { assignments, errors, changes } = computePlan(policy, agents, roles);
  if (errors.length > 0) {
    fail(1, "ERROR: imported policy has conflicts:", ...[...new Set(errors)].map((e) => "  " + e));
  }
  if (changes.length > 0) {
    fail(
      1,
      "ERROR: imported policy does not reproduce the current tree (bug):",
      ...changes.map((c) => "  " + describeChange(c)),
    );
  }
  if (dryRun) {
    console.log(`dry-run: would write policy with ${policy.rules.length} rules (0 file changes)`);
    return;
  }
  const policyText = canonicalPolicyText(policy);
  writeFileSync(policyPath, policyText);
  writeFileSync(assignmentsPath, renderAssignments(policyText, assignments));
  console.log(
    `OK: imported model policy (${policy.rules.length} rules, ${assignments.length} assignments, 0 file changes)`,
  );
}

// ── main ─────────────────────────────────────────────────────────────────────

const [command, ...rest] = process.argv.slice(2);
switch (command) {
  case "report":
    runReport({ json: rest.includes("--json") });
    break;
  case "check":
    runCheck();
    break;
  case "apply":
    runApply({ dryRun: rest.includes("--dry-run") });
    break;
  case "set":
    runSet(rest);
    break;
  case "import-current":
    runImportCurrent({ force: rest.includes("--force"), dryRun: rest.includes("--dry-run") });
    break;
  default:
    fail(
      2,
      "usage: node scripts/model-policy.mjs <report|check|apply|set|import-current> [options]",
    );
}
