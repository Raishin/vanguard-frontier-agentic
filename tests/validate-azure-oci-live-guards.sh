#!/usr/bin/env bash
# Eval grader for azure-oci-live-guards
# Pass^3 deterministic structural + anti-pattern + regression checks.
set -u
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

PASS=0
FAIL=0
WARN=0
fail() { echo "  FAIL: $*"; FAIL=$((FAIL+1)); }
pass() { echo "  PASS: $*"; PASS=$((PASS+1)); }
warn() { echo "  WARN: $*"; WARN=$((WARN+1)); }

AZURE_AGENTS=(
  azure-live-arm-deployment-stack-guard
  azure-live-pim-jit-activation-guard
  azure-live-aks-rollout-guard
  azure-live-app-service-slot-swap-guard
  azure-live-keyvault-rotation-purge-guard
  azure-live-cost-budget-action-guard
)
OCI_AGENTS=(
  oci-live-resource-manager-stack-guard
  oci-live-iam-policy-compartment-guard
  oci-live-oke-rollout-guard
  oci-live-autonomous-db-lifecycle-guard
  oci-live-vault-key-destruction-guard
  oci-live-cost-budget-runaway-guard
)
HARNESSES=(claude-code.agent.md codex.toml copilot.agent.md cursor.agent.md gemini.agent.md kiro-cli.agent.json kiro-ide.agent.md)

echo "=== STRUCTURAL EVALS ==="
for id in "${AZURE_AGENTS[@]}" "${OCI_AGENTS[@]}"; do
  cloud="${id%%-*}"
  dir="agents/$cloud/${id}-agent"
  for f in AGENT.md PERMISSIONS.md PREFLIGHT.md ROLLBACK.md metadata.json; do
    [ -f "$dir/$f" ] && pass "$dir/$f exists" || fail "$dir/$f missing"
  done
  for h in "${HARNESSES[@]}"; do
    [ -f "$dir/harnesses/$h" ] && pass "$dir/harnesses/$h exists" || fail "$dir/harnesses/$h missing"
  done
  skill="skills/$cloud/${id}/SKILL.md"
  [ -f "$skill" ] && pass "$skill exists" || fail "$skill missing"
  smeta="skills/$cloud/${id}/metadata.json"
  [ -f "$smeta" ] && pass "$smeta exists" || fail "$smeta missing"
done

echo
echo "=== SCHEMA EVALS ==="
python3 - <<'PY'
import json, sys, pathlib, re
root = pathlib.Path(".")
agent_schema = json.load(open("schemas/agent.schema.json"))
skill_schema = json.load(open("schemas/skill.schema.json"))
ids = ["azure-live-arm-deployment-stack-guard","azure-live-pim-jit-activation-guard","azure-live-aks-rollout-guard","azure-live-app-service-slot-swap-guard","azure-live-keyvault-rotation-purge-guard","azure-live-cost-budget-action-guard","oci-live-resource-manager-stack-guard","oci-live-iam-policy-compartment-guard","oci-live-oke-rollout-guard","oci-live-autonomous-db-lifecycle-guard","oci-live-vault-key-destruction-guard","oci-live-cost-budget-runaway-guard"]
errs = 0
for id_ in ids:
    cloud = id_.split("-")[0]
    a = root / f"agents/{cloud}/{id_}-agent/metadata.json"
    s = root / f"skills/{cloud}/{id_}/metadata.json"
    for path, schema, required_id in [(a, agent_schema, f"{id_}-agent"), (s, skill_schema, id_)]:
        if not path.exists(): continue
        m = json.load(open(path))
        for r in schema["required"]:
            if r not in m:
                print(f"  FAIL: {path} missing required field {r}"); errs += 1
        if m.get("id") != required_id:
            print(f"  FAIL: {path} id={m.get('id')} expected {required_id}"); errs += 1
        if not re.match(r"^\d{4}-\d{2}-\d{2}$", m.get("last_verified","")):
            print(f"  FAIL: {path} bad last_verified"); errs += 1
print(f"  schema errors: {errs}")
PY

echo
echo "=== ANTI-PATTERN EVALS ==="
for id in "${AZURE_AGENTS[@]}" "${OCI_AGENTS[@]}"; do
  cloud="${id%%-*}"
  perm="agents/$cloud/${id}-agent/PERMISSIONS.md"
  [ -f "$perm" ] || continue
  if grep -qi 'AdministratorAccess' "$perm"; then
    fail "$perm contains AdministratorAccess (AWS anti-pattern)"
  else pass "$perm no AdministratorAccess"; fi
  if [ "$cloud" = "oci" ]; then
    if grep -Pi 'manage\s+all-resources\s+in\s+tenancy' "$perm" | grep -vqi 'do not\|never\|forbidden\|anti-pattern'; then
      fail "$perm grants 'manage all-resources in tenancy' (OCI anti-pattern)"
    else pass "$perm no unrestricted tenancy-wide manage"; fi
    if ! grep -qi 'Allow' "$perm"; then
      fail "$perm has no OCI 'Allow' policy statements"
    else pass "$perm contains OCI Allow statements"; fi
    if ! grep -Pqi '\b(inspect|read|use|manage)\b' "$perm"; then
      fail "$perm has no OCI verbs (inspect/read/use/manage)"
    else pass "$perm uses OCI verbs"; fi
  fi
  if [ "$cloud" = "azure" ]; then
    if ! grep -qi 'Microsoft\.' "$perm"; then
      fail "$perm has no Microsoft.* RBAC actions"
    else pass "$perm uses Microsoft.* actions"; fi
  fi
done

echo
echo "=== PREFLIGHT/ROLLBACK COMMAND PRESENCE ==="
for id in "${AZURE_AGENTS[@]}" "${OCI_AGENTS[@]}"; do
  cloud="${id%%-*}"
  pf="agents/$cloud/${id}-agent/PREFLIGHT.md"
  rb="agents/$cloud/${id}-agent/ROLLBACK.md"
  cli=$([ "$cloud" = "azure" ] && echo "az " || echo "oci ")
  [ -f "$pf" ] && grep -q "^\`\`\`" "$pf" && pass "$pf has fenced code blocks" || fail "$pf no fenced code blocks"
  [ -f "$rb" ] && grep -q "^\`\`\`" "$rb" && pass "$rb has fenced code blocks" || fail "$rb no fenced code blocks"
done

echo
echo "=== UNIQUENESS EVAL ==="
python3 - <<'PY'
import pathlib, hashlib, re
root = pathlib.Path(".")
for cloud, ids in [("azure",["azure-live-arm-deployment-stack-guard","azure-live-pim-jit-activation-guard","azure-live-aks-rollout-guard","azure-live-app-service-slot-swap-guard","azure-live-keyvault-rotation-purge-guard","azure-live-cost-budget-action-guard"]),
                   ("oci",["oci-live-resource-manager-stack-guard","oci-live-iam-policy-compartment-guard","oci-live-oke-rollout-guard","oci-live-autonomous-db-lifecycle-guard","oci-live-vault-key-destruction-guard","oci-live-cost-budget-runaway-guard"])]:
    bodies = {}
    for id_ in ids:
        p = root / f"agents/{cloud}/{id_}-agent/AGENT.md"
        if not p.exists(): continue
        text = re.sub(r'\s+', ' ', p.read_text())
        bodies[id_] = text
    items = list(bodies.items())
    for i in range(len(items)):
        for j in range(i+1, len(items)):
            a, b = items[i][1], items[j][1]
            shared = sum(1 for w in set(a.split()) if w in set(b.split()))
            ratio = shared / max(len(set(a.split())), 1)
            if ratio > 0.92:
                print(f"  FAIL: {items[i][0]} vs {items[j][0]} word-set overlap {ratio:.2f}")
            else:
                print(f"  PASS: {items[i][0]} vs {items[j][0]} overlap {ratio:.2f}")
PY

echo
echo "=== REGRESSION EVALS ==="
aws_count=$(ls -d agents/aws/aws-live-* 2>/dev/null | wc -l)
[ "$aws_count" = "5" ] && pass "AWS live agents intact (5)" || fail "AWS live count drift: $aws_count"
azure_total=$(ls -d agents/azure/*-agent 2>/dev/null | wc -l)
oci_total=$(ls -d agents/oci/*-agent 2>/dev/null | wc -l)
echo "  azure agent total: $azure_total"
echo "  oci   agent total: $oci_total"

echo
echo "=== CATALOG EVALS ==="
for id in "${AZURE_AGENTS[@]}" "${OCI_AGENTS[@]}"; do
  grep -q "\"${id}-agent\"" catalog/agents.json && pass "catalog/agents.json has ${id}-agent" || fail "catalog/agents.json missing ${id}-agent"
  grep -q "\"${id}\"" catalog/skills.json && pass "catalog/skills.json has ${id}" || fail "catalog/skills.json missing ${id}"
done

echo
echo "=== SUMMARY ==="
echo "PASS: $PASS  FAIL: $FAIL  WARN: $WARN"
[ "$FAIL" = "0" ] && exit 0 || exit 1
