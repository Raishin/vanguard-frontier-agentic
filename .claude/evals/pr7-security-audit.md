# EVAL: pr7-security-audit

## Scope

PR #7: 244 files / +14,606 / -316 across 6 commits on `claude/setup-azure-oci-agents-qH7H6`

**Material attack surface:**
- 13× `PERMISSIONS.md` files (Azure RBAC roles, OCI IAM policies, service principal grants)
- 12× `preflight-commands.md` files (live `az` / `kubectl` / `oci` command examples)
- 12× `rollback-playbook.md` files (mutation/recovery commands)
- `skills/finops/finops-cloud-price-advisor/references/pricing-apis.md` (3 public HTTP APIs invoked via WebFetch — SSRF / URL trust surface)
- `skills/finops/finops-cloud-price-advisor/references/currency-handling.md` (2 exchange-rate APIs)
- 7× harness adapter files per agent × 12 new agents = 84 prompt surfaces
- `catalog/agents.json` / `catalog/skill-manifest.json` (schema-validated metadata)
- `tests/validate-azure-oci-live-guards.sh` (new test harness — eval to be run)
- 2 generator scripts: `scripts/gen_azure_live_guards.py`, `scripts/gen_oci_live_guards.py`

## Capability Evals (security)

### Secret / credential scanning
- [ ] No hardcoded API keys, OAuth tokens, JWTs, AWS access keys, Azure connection strings, OCI API keys, SSH keys, database passwords, or webhook secrets in diff
- [ ] No real subscription IDs, tenant IDs, account numbers, OCIDs (Oracle Cloud IDs), email addresses, or customer-identifying values
- [ ] No `.env`, `*.pem`, `*.key`, `id_rsa*`, `credentials`, or `config` private files committed
- [ ] No tokens in commit messages or branch names

### RBAC / IAM least-privilege
- [ ] Azure custom roles use `NotActions` for irreversible operations (delete, purge)
- [ ] No wildcard `Microsoft.*/*` actions
- [ ] `AssignableScopes` is narrow (subscription or resource-group scoped, not `/`)
- [ ] OCI policies do not grant `manage all-resources in tenancy` without conditions
- [ ] Service-principal grants (`Allow service ...`) are tag-condition or resource-scoped
- [ ] Break-glass paths (PIM, MFA-TOTP, dual-approval) are documented as exceptions, not defaults
- [ ] `NotActions` cannot be bypassed via Microsoft Authorization role assignment write
- [ ] DataActions/NotDataActions correctly partition Key Vault data-plane operations

### Privilege escalation paths
- [ ] No agent role can grant itself broader role via `roleAssignments/write`
- [ ] No OCI dynamic-group grant allows pivot to tenancy admin
- [ ] No combination of two roles in this PR composes to broader-than-documented surface
- [ ] PIM activation flows enforce time-bounded duration (max 8 hours)

### URL / SSRF / external-fetch trust
- [ ] All WebFetch target URLs in pricing-apis.md belong to documented public cloud-provider domains
- [ ] No user-controllable URL substitution that could pivot to internal IPs (169.254.169.254, link-local, 127.0.0.1, RFC1918)
- [ ] Exchange-rate APIs (open.er-api.com, ecb.europa.eu) are explicitly named, not parameterized
- [ ] OCI pricing endpoint (`apexapps.oracle.com`) is verified as Oracle-owned
- [ ] No HTTP (cleartext) URLs — only HTTPS

### Command injection / unsafe CLI patterns
- [ ] No shell command examples use unquoted variable interpolation
- [ ] No `eval`, `exec`, `bash -c "$VAR"`, or backtick command substitution into user input
- [ ] kubectl/az/oci commands use `--` separator and quoted placeholders
- [ ] Rollback playbooks do not contain `rm -rf` without explicit scope confirmation
- [ ] Force flags (`--force`, `--no-prompt`, `--auto-approve`) are gated behind operator confirmation in skill text

### Prompt injection / instruction smuggling
- [ ] No skill reference contains `<system>`, `<assistant>`, or role-claim markers
- [ ] No reference content includes "ignore previous instructions" patterns
- [ ] No URL in references could be replaced by attacker to inject content into agent context
- [ ] Live-guard refusal-by-default posture is consistent across all 7 harness adapters per agent
- [ ] Harness adapters (codex.toml, kiro-cli.agent.json) cannot be modified to escape sandbox via prompt content

### Catalog / supply-chain integrity
- [ ] All catalog `path` fields are relative, no `../`, no absolute paths
- [ ] `catalog/skill-manifest.json` SHA-256 hashes are valid hex
- [ ] No new harness flag values that bypass schema validation
- [ ] Generator scripts (`gen_*_live_guards.py`) do not write to paths outside `agents/` or `skills/`
- [ ] `package.json` no new postinstall/prepare scripts that execute arbitrary code
- [ ] CLI `vfa-export-agents` does not allow `--repo` to traverse via `../`

### OWASP Top 10 coverage (2021/2025)
- [ ] A01 — Broken Access Control: RBAC roles enforce intended scope
- [ ] A02 — Cryptographic Failures: no hardcoded keys, no MD5/SHA-1 used for integrity
- [ ] A03 — Injection: no SQL/NoSQL/command/prompt injection in skill commands
- [ ] A04 — Insecure Design: live-guard refusal-by-default is enforced
- [ ] A05 — Security Misconfiguration: no overly permissive defaults
- [ ] A06 — Vulnerable Components: no dependency changes (offline check)
- [ ] A07 — Identification/Auth Failures: PIM/MFA gates enforced for sensitive ops
- [ ] A08 — Software/Data Integrity: skill-manifest hashes valid; no unverified external content fetch on install
- [ ] A09 — Logging/Monitoring: skill response shape requires evidence trails
- [ ] A10 — SSRF: pricing API URLs are allowlisted to known cloud-provider hostnames

## Regression Evals

- [ ] `tests/validate-azure-oci-live-guards.sh` exits 0 with 253 PASS
- [ ] `npm run validate` — all 4 sub-checks green
- [ ] `npm run manifest:check` — manifest hashes match skill content
- [ ] No previously-passing security control was weakened by this PR

## Graders

- **Code grader**: regex secret scan, JSON schema validation, manifest hash check, exit-code tests
- **Model grader**: security-reviewer agent for cross-cutting OWASP analysis
- **Human grader**: required for any HIGH/CRITICAL finding before close

## Success criteria

- pass^1 = 1.00 on secret/credential scanning (zero tolerance)
- pass@1 ≥ 0.95 on RBAC least-privilege evals
- pass^1 = 1.00 on SSRF / URL trust (zero tolerance)
- pass^3 = 1.00 on regression evals
- Zero unresolved CRITICAL findings before merge
