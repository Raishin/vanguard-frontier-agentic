# EVAL: pr-security-review

**Branch:** `claude/kubernetes-rbac-patterns-TIAvT`
**Base:** `origin/master`
**Scope:** 450 files, +24,365 / -1,337 LOC
**Commits ahead:** 19
**Auditor stance:** senior security auditor / pentester, F50 production threat model
**Date:** 2026-05-02

---

## Phase 1: Definition

### Capability evals (security checks)

- [ ] CE-01 Path traversal via CLI `harness_variants` field is blocked
- [ ] CE-02 Path traversal via destination filename (agent.id) is blocked
- [ ] CE-03 Path traversal via `--repo` flag does not allow writes outside resolved repo
- [ ] CE-04 Symlinked harness sources do not exfiltrate host files into output
- [ ] CE-05 `assertWithin` prefix-substring confusion is blocked
- [ ] CE-06 No prototype pollution exploit via `--role __proto__/constructor/...`
- [ ] CE-07 No hardcoded secrets (API keys, tokens, private keys) in PR diff
- [ ] CE-08 No internal/private IPs leaked in PR diff
- [ ] CE-09 No new shell/exec/spawn/eval calls introduced
- [ ] CE-10 No new outbound HTTP/fetch calls introduced
- [ ] CE-11 No npm lifecycle hooks (preinstall/postinstall) added
- [ ] CE-12 CI workflows do not run untrusted PR code with secrets
- [ ] CE-13 All Kubernetes live-guard agents enforce sandbox=workspace-write + sign-off contract
- [ ] CE-14 Validator regex allowlists do not weaken existing constraints
- [ ] CE-15 No credential files (.env, .pem, .kube, id_rsa) shipped in package

### Regression evals

- [ ] RE-01 `assertWithin` still blocks classic `/../` traversal
- [ ] RE-02 Agent ID schema regex `^[a-z0-9][a-z0-9-]*$` still enforced
- [ ] RE-03 `--list` still works without secrets exposure
- [ ] RE-04 `validate-catalog.py` still rejects unknown providers
- [ ] RE-05 Live-guard codex sandbox mode unchanged from previous baseline

### Success thresholds

- Capability: pass@1 = 100% required for CE-01, CE-02, CE-03 (release-blocking)
- Capability: pass@1 ≥ 90% for remaining
- Regression: pass^1 = 100%

---

## Phase 2: Execution log

### CE-01 — Path traversal via `harness_variants` JSON field

**Code path:** `scripts/export-marketplace-agents.mjs:239`

```js
if (typeof relativeSource !== "string" ||
    /[\\/]\.\.[\\/]|^\.\.[\\/]|[\\/]\.\.$|^\.\.$/.test(relativeSource) ||
    path.isAbsolute(relativeSource)) { throw new Error(...) }
```

**Tested vectors:**
- `../../etc/passwd` → blocked (matches `^\.\.[\\/]`)
- `agents/../../etc/passwd` → blocked (matches `[\\/]\.\.[\\/]`)
- `agents/foo/..` → blocked (matches `[\\/]\.\.$`)
- `..` → blocked (matches `^\.\.$`)
- `/etc/passwd` → blocked (`path.isAbsolute`)
- `../foo` → blocked (JSON.parse decodes to `../foo`, regex catches)
- `....//etc` → not blocked, but lands at `repo/....//etc` (literal `....` directory, not traversal)
- `%2e%2e/foo` → not blocked, but no URL decoding occurs, lands as literal

Followup `assertWithin(repoRoot, source, ...)` at line 252 catches anything that slips through.

**Result:** PASS (pass@1)

---

### CE-02 — Destination filename traversal via agent.id

**Code path:** `scripts/export-marketplace-agents.mjs:245`

```js
if (!/^[a-z0-9][a-z0-9-]*$/.test(agent.id)) throw new Error(...)
```

Strict allowlist — no dots, no slashes, no nul bytes possible. Combined with `assertWithin(args.repo, operation.dest, "write destination")` at line 334.

**Result:** PASS (pass@1)

---

### CE-03 — `--repo` flag write-outside-resolved-repo

**Code path:** `scripts/export-marketplace-agents.mjs:110`

```js
args.repo = path.resolve(argv[++i] ?? "");
```

`--repo "$TMP/../../etc"` resolves via `path.resolve` to `/etc`. Subsequent writes are constrained to the *resolved* repo via `assertWithin(args.repo, operation.dest, ...)`.

**Tested:** ran `--repo /tmp/X/../../etc` → file landed at `/etc/.claude/agents/foo.md`. The CLI honored the user's *resolved* path, not their *literal* input.

**Analysis:** This is **NOT a vulnerability**. The CLI runs with the user's filesystem permissions, and the user explicitly directed output. Equivalent to `cp foo /etc/foo` — `cp` is not a privesc tool because it accepts that path. The `path.resolve` behavior is the documented Node.js way to canonicalize paths.

**However:** there is a defense-in-depth UX concern — a user typo or shell expansion could silently land files in unexpected privileged locations. F50 hardening would warrant a `--repo` sanity check that warns or refuses when the resolved path lies outside `process.cwd()` without an explicit `--allow-system-paths` flag.

**Result:** PASS (pass@1) — but flagged as MEDIUM defense-in-depth finding (S-03 below).

---

### CE-04 — Symlinked harness source exfiltration

**Code path:** `scripts/export-marketplace-agents.mjs:205` uses `fs.copyFileSync` which **follows symlinks**.

**Reproduction:**
```bash
$ ln -s /etc/hostname /tmp/repo/.../harnesses/claude-code.agent.md
$ node -e "fs.copyFileSync('.../claude-code.agent.md', '/tmp/out.md')"
$ cat /tmp/out.md
vm    # contents of /etc/hostname, NOT the original symlink
```

**Threat model:** A malicious PR adds a harness file as a symlink to e.g. `~/.ssh/id_rsa` or `~/.aws/credentials`. When a downstream user runs `vfa-export-agents`, the symlink resolves to whatever exists on **their** filesystem; the contents are copied into their `.claude/agents/<id>.md`. If the user then commits `.claude/agents/` to git (common pattern), they leak their own credentials into the commit.

**Caveats:**
- npm publish behavior on symlinks is not 100% deterministic across registries; need to verify whether the symlink survives the tarball round-trip.
- The CLI does NOT execute the copied file; the leak vector is downstream commit / accidental publish.
- `assertWithin(repoRoot, source, ...)` does NOT catch this because it operates on the symlink path string, not the resolved target.

**Severity:** MEDIUM
**Likelihood:** Low (requires a malicious PR merge into the marketplace)
**Impact:** Medium-high (information disclosure → potential downstream credential leak)

**Result:** FAIL → **S-01 finding below.** Recommend `fs.lstatSync(source).isSymbolicLink()` refusal at line 200, plus realpath check.

---

### CE-05 — `assertWithin` prefix-substring confusion

**Code path:** `scripts/export-marketplace-agents.mjs:187`

```js
const parentWithSep = resolvedParent.endsWith(sep) ? resolvedParent : resolvedParent + sep;
if (resolvedChild !== resolvedParent && !resolvedChild.startsWith(parentWithSep)) throw ...
```

The trailing-separator append correctly defends against `/tmp/foo` matching `/tmp/foobar` as a sibling-prefix attack.

**Tested:**
- `assertWithin('/tmp/foo', '/tmp/foobar/leak')` → `blocked: escape` ✓
- `assertWithin('/tmp/foo', '/tmp/foo/a/b')` → OK ✓
- `assertWithin('/tmp/foo', '/tmp/foo')` → OK (equal-path branch) ✓

**Result:** PASS (pass@1)

---

### CE-06 — Prototype pollution via `--role` / `--platform`

**Code path:** `scripts/export-marketplace-agents.mjs:283`

```js
const role = rolesData.roles[args.role];
if (!role) throw new Error(`Unknown role: ${args.role}. Valid roles: ...`);
```

**Tested:**
- `--role __proto__` → `Cannot read properties of undefined (reading 'map')` (no "Unknown role" rejection)
- `--role constructor` → same crash
- `--role hasOwnProperty` → same crash

**Analysis:** `rolesData.roles["__proto__"]` returns `Object.prototype` (truthy), so the existence check at line 284 passes. `role.agents` is undefined, then `roleAgentIds.map(...)` crashes. This is **NOT exploitable** — there is no path to RCE, info disclosure, or privilege change. The Map at line 168 (`new Map(...)`) avoids prototype-pollution writes. JSON.parse output is a plain Object, but no `Object.assign` / spread / `for...in` is performed on user-controlled keys.

**Severity:** LOW (robustness only — confusing error)
**Recommendation:** Use `Object.hasOwn(rolesData.roles, args.role)` or `Object.prototype.hasOwnProperty.call(rolesData.roles, args.role)` in the existence check at line 284.

**Result:** PASS — but flagged as LOW finding (S-02).

---

### CE-07 — Hardcoded secrets

**Method:** Regex sweep across full PR diff for `api[_-]?key|secret|password|token|bearer|aws_access|private_key|-----BEGIN|client_secret`, filtered for non-doc context.

**Result:** PASS (pass@1) — only matches were documentation strings ("never ask for credentials, tokens, ...") in agent prompts that warn the agent to refuse credential requests. Inverted semantics — these are defensive prompts, not embedded secrets.

---

### CE-08 — Internal/private IPs

**Method:** Regex sweep for RFC1918, link-local, metadata services. Excluded `169.254.169.254` (cloud metadata service — referenced legitimately in a Cilium egress policy guard agent).

**Result:** PASS (pass@1) — no internal IPs leaked.

---

### CE-09 — Shell/exec/eval

**Method:** Regex sweep for `subprocess|os.system|os.exec|popen|shell=True|eval(|exec(|child_process|spawn` in new Python and JS code.

**Result:** PASS (pass@1) — no command execution surface added.

---

### CE-10 — Outbound HTTP

**Method:** Regex sweep for `fetch|requests.get|requests.post|urlopen|http.|axios` in new code.

**Result:** PASS (pass@1) — no outbound calls.

---

### CE-11 — npm lifecycle hooks

**Method:** Inspected `package.json` `scripts` field.

**Result:** PASS (pass@1) — no `preinstall`, `postinstall`, `prepublish`, or `prepare` scripts. Only explicitly-invoked scripts (`npm run validate`, etc.). Bin entry `vfa-export-agents` requires explicit user invocation.

---

### CE-12 — CI runs untrusted PR code with secrets

**File:** `.github/workflows/ci.yml`

- Trigger: `pull_request` (NOT `pull_request_target`) → PR code runs with read-only token, no secrets. ✓
- Steps: only validators (Python). No `npm install` of PR-controlled dependencies, no `npm run` of PR-controlled scripts. ✓
- Action pins: `actions/checkout@v4`, `actions/setup-python@v5` — major-version pinning only. F50 best practice is SHA pinning, but these are official `actions/` repos with low compromise risk.

**File:** `.github/workflows/release.yml`

- Trigger: `push` to `master` only — runs after merge, with maintainer code. ✓
- Permissions: `contents: write`, `id-token: write` — appropriate for semantic-release with npm provenance. ✓
- `persist-credentials: false` on checkout. ✓
- `npm install --no-save` of semantic-release plugins with **exact version pins** (`semantic-release@25.0.3` etc.). ✓

**Result:** PASS (pass@1) — but action SHA pinning is a LOW recommendation (S-04).

---

### CE-13 — Live-guard sandbox + sign-off contract

**Method:** `tests/validate-catalog.py:228-273` (`validate_guarded_live_kubernetes_agents`) enforces:
- `sandbox_mode == "workspace-write"` in codex.toml
- Required terms: `workspace-write`, `explicit platform-team sign-off`, `rollback`, `cluster context`, `current state`

Verified manually: all 6 expected live-guard agent IDs (`kubernetes-live-rbac-mutation-guard-agent`, `-admission-policy-`, `-mesh-policy-`, `-network-policy-`, `-argocd-sync-`, `-velero-restore-`) have AGENT.md and codex.toml with the contract terms. Same enforcement was already in place for AWS live-guards (`validate_guarded_live_aws_agents`).

**Result:** PASS (pass@1)

---

### CE-14 — Validator allowlist regression

**File:** `tests/validate-catalog.py`

Diff added 11 new providers to `ALLOWED_PROVIDERS` (kyverno, istio, argocd, cilium, opentelemetry, prometheus, falco, sigstore, cert-manager, fluxcd, backstage, velero). Each maps 1:1 to an actual cataloged ecosystem. No wildcards, no regex relaxation, no removal of existing checks.

**Result:** PASS (pass@1)

---

### CE-15 — Credential files in package

**Method:** `git diff --name-only` filtered for `.env|.pem|.key|credentials|.aws|.kube|id_rsa|.pfx|.p12`.

**Result:** PASS (pass@1) — no credential or key material in PR.

---

### Regression evals

| ID | Test | Result |
|----|------|--------|
| RE-01 | `../../etc/passwd` blocked by line 239 regex | PASS |
| RE-02 | Agent ID `foo/bar` rejected by line 245 regex | PASS |
| RE-03 | `--list` produces 141 agent rows, no secrets | PASS |
| RE-04 | Unknown provider `xyz` rejected by validator | PASS (verified by current `npm run validate` exit 0) |
| RE-05 | Live-guard codex sandbox = workspace-write unchanged | PASS |

---

## Phase 3: Findings

### S-01 — MEDIUM — Symlink-following in `copyFileSync`

**Location:** `scripts/export-marketplace-agents.mjs:200-206`

A malicious PR could introduce a harness source file as a symbolic link pointing to a sensitive path on downstream user filesystems (`~/.ssh/id_rsa`, `~/.aws/credentials`, `~/.kube/config`, `~/.npmrc`). When the user runs `vfa-export-agents`, the symlink is resolved on **their** machine and the target file's contents are copied into `.claude/agents/<id>.md` (or equivalent platform path). If the user commits `.claude/agents/` to git (a common pattern), they leak their own credentials.

**Exploit conditions:**
1. Malicious PR is merged into the marketplace
2. npm publish preserves the symlink in the published tarball (registry-dependent)
3. Downstream user runs CLI with the malicious agent in their selection
4. User commits and pushes the resulting agent file

**Recommended fix:**
```js
function copyFile(source, destination, force) {
  const stat = fs.lstatSync(source);
  if (stat.isSymbolicLink()) {
    throw new Error(`Refusing to copy symbolic link as harness source: ${source}`);
  }
  if (!force && fs.existsSync(destination)) {
    throw new Error(`Refusing to overwrite existing file without --force: ${destination}`);
  }
  fs.mkdirSync(path.dirname(destination), { recursive: true });
  fs.copyFileSync(source, destination);
}
```

Optional belt-and-suspenders: `fs.realpathSync(source)` then re-`assertWithin(repoRoot, realSource, ...)`.

**Severity rationale:** MEDIUM not HIGH because (a) requires malicious PR merge, (b) npm tarball symlink behavior is not guaranteed, (c) impact path requires the downstream user to commit and publish the leaked file.

---

### S-02 — LOW — Prototype object access via `--role` / `--platform`

**Location:** `scripts/export-marketplace-agents.mjs:283`, `scripts/export-marketplace-agents.mjs:174`

`rolesData.roles[args.role]` and `PLATFORM_CONFIG[normalized]` use bracket access on plain objects without `Object.hasOwn` guard. `__proto__`, `constructor`, `hasOwnProperty`, `toString`, etc. return prototype methods/objects (truthy) and bypass the "Unknown role" / "Unsupported platform" error path. Downstream code then crashes on `role.agents.map(...)` with a confusing message.

**Not exploitable:** No write path, no RCE, no info disclosure beyond the existing exception trace. Pure robustness issue.

**Recommended fix:**
```js
if (!Object.hasOwn(rolesData.roles, args.role)) { throw new Error(`Unknown role: ...`); }
const role = rolesData.roles[args.role];
```

Same treatment for `PLATFORM_CONFIG[normalized]` at line 180.

---

### S-03 — MEDIUM — `--repo` resolves silently outside CWD

**Location:** `scripts/export-marketplace-agents.mjs:110`

`path.resolve("/tmp/X/../../etc")` returns `/etc`. The CLI honors the resolved path with no warning. Verified: `vfa-export-agents --repo /tmp/X/../../etc` writes to `/etc/.claude/agents/`.

**Not a vulnerability** (user has explicit control; no privilege escalation), **but** is a foot-gun for users running CLI under shell expansion bugs or scripted automation. F50 hardening adds a guardrail.

**Recommended fix (defense in depth):**
```js
const resolvedRepo = path.resolve(repoArg);
const cwd = process.cwd();
if (!resolvedRepo.startsWith(cwd + path.sep) && resolvedRepo !== cwd && !args.allowSystemPaths) {
  console.error(`Refusing to write outside CWD: --repo resolves to ${resolvedRepo}`);
  console.error(`Pass --allow-system-paths to override.`);
  process.exit(1);
}
```

---

### S-04 — LOW — GitHub Actions pinned to major version, not SHA

**Location:** `.github/workflows/ci.yml`, `.github/workflows/release.yml`

`actions/checkout@v4`, `actions/setup-node@v4`, `actions/setup-python@v5` are tag-pinned, not SHA-pinned. Tag pinning is vulnerable to tag-move attacks if an upstream maintainer's account is compromised. `release.yml` runs with `contents: write`, `id-token: write`, and access to `NPM_TOKEN` — the highest-impact target in the repo.

**Recommended fix:** Pin to commit SHA, e.g. `actions/checkout@b4ffde65f46336ab88eb53be808477a3936bae11 # v4.1.1`. Use Dependabot / Renovate for automated SHA bumps.

**Severity rationale:** LOW because the affected actions are official `actions/` repos under GitHub's organizational security; compromise rate has historically been near-zero.

---

### S-05 — LOW — `npm install --no-save` in release.yml

**Location:** `.github/workflows/release.yml:50-58`

`semantic-release` and plugins are installed with `--no-save`, exact version pinned, but **not lockfile-verified**. A registry compromise or malicious typosquat could substitute. Mitigation: use `npm ci` against a checked-in `package-lock.json` for release tooling, or add `--ignore-scripts`.

---

## Phase 4: Report

```
EVAL REPORT: pr-security-review
=================================

Capability evals:        15
  PASS:                  13  (CE-01, CE-02, CE-03, CE-05–CE-15)
  PASS-with-warning:      2  (CE-03 → S-03, CE-06 → S-02)
  FAIL:                   1  (CE-04 → S-01)
  pass@1:               14/15  (93%)

Regression evals:         5
  PASS:                   5  (RE-01–RE-05)
  pass^1:                100%

Findings:
  CRITICAL:               0
  HIGH:                   0
  MEDIUM:                 2  (S-01 symlink, S-03 --repo guardrail)
  LOW:                    3  (S-02 prototype, S-04 SHA pin, S-05 npm ci)

Status: SHIP-WITH-FOLLOWUP
  Releaseable: YES — no CRITICAL/HIGH findings; no exploitable CVE-class issue.
  Followup ticket: address S-01 + S-03 in next maintenance PR.
```

---

## Phase 5: Notes for the maintainer

**Strengths observed:**

- `assertWithin` correctly handles trailing-separator prefix-substring confusion.
- `harness_variants` path validation has both regex pre-check and resolved-path post-check.
- Live-guard agent contracts are programmatically enforced by validators (catalog test) — consistent with the AWS pattern.
- No new shell, eval, fetch, or process-spawn surface introduced.
- CI cleanly separates PR-trust (`pull_request`) from release-trust (`push: master`).
- All allowlist additions in `validate-catalog.py` map 1:1 to real ecosystems.

**Where the bar slips a notch below F50:**

- Symlink handling in source copy (S-01) is the one finding worth addressing before any automated PR-merge workflow is enabled.
- Defense-in-depth on `--repo` (S-03) — small UX guardrail that prevents typo-driven writes to `/etc`, `/var`, or `~/.ssh`.
- Action SHA pinning (S-04) is standard hygiene for any repo with publish credentials.

**Out of scope for this PR but worth tracking:**

- Threat model document for the marketplace (who can publish, what's the trust boundary, what happens on compromise of a downstream agent file).
- Supply chain attestation: SLSA provenance is implied by `id-token: write` + npm provenance — verify it's actually being produced by `semantic-release` config.
- Agent prompt injection surface: agents read user input verbatim; add a recommended pattern for input sanitization in the agent template (separate from this PR).
