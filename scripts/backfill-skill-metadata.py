#!/usr/bin/env python3
"""Backfill `metadata.updated` and `metadata.category` on every SKILL.md.

`updated` is derived from the last git commit date that touched the SKILL.md.
`category` is classified deterministically from the skill name using a keyword
rules table with a fixed precedence order.

Usage:
  python3 scripts/backfill-skill-metadata.py --dry-run
  python3 scripts/backfill-skill-metadata.py
"""

from __future__ import annotations

import argparse
import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SKILLS_DIR = ROOT / "skills"

# Precedence order: earlier categories win when multiple keyword groups match.
# security > networking > resilience > observability > delivery > compliance
#         > finops > ai > data > platform
CATEGORY_RULES: list[tuple[str, list[str]]] = [
    (
        "security",
        [
            "iam", "rbac", "secret", "kms", "vault", "perimeter", "policy",
            "psa", "pod-security", "guard", "supply-chain", "falco", "cosign",
            "sigstore", "kyverno", "security", "hardening", "cert-manager",
            "certificate", "private-ca", "issuer", "trust", "workload-identity",
            "entra", "pim", "external-secrets", "keyvault", "key-vault",
            "ambient-mesh", "network-policy", "rotation", "purge", "destruction",
            "cloud-guard", "threat",
        ],
    ),
    (
        "networking",
        [
            "network", "mesh", "cilium", "istio", "vpc", "endpoint", "topology",
            "load-balancer", "traffic", "private-endpoint", "api-edge", "edge",
        ],
    ),
    (
        "resilience",
        [
            "backup", "recovery", "bcdr", "resilience", "velero",
            "data-protection", "restore",
        ],
    ),
    (
        "observability",
        [
            "observability", "monitor", "incident", "responder", "investigator",
            "prometheus", "opentelemetry", "alerting", "resource-health",
            "triage", "health",
        ],
    ),
    (
        "delivery",
        [
            "ci-cd", "release", "pipeline", "rollout", "deployment", "gitops",
            "argocd", "argo-rollouts", "flux", "scaffolder", "registry",
            "rollout-corrector", "hotfix", "slot-swap", "approval", "devops",
            "platform-automation", "agent-skill-designer", "stack-guard",
            "iac", "arm-deployment", "resource-manager-stack", "migration",
            "cutover", "fix-operator", "patch-executor", "change-impact",
            "change-safety", "deployment-stack",
        ],
    ),
    (
        "compliance",
        [
            "compliance", "evidence", "audit", "governance", "landing-zone",
            "guardrail", "subscription-resource", "identity-governance",
            "role-selector", "entra-id-specialist", "access-governor",
            "limits-capacity", "resource-search", "ticket-triage",
        ],
    ),
    (
        "finops",
        [
            "cost", "finops", "budget", "kubecost", "anomaly", "price",
            "chargeback", "estimation",
        ],
    ),
    (
        "ai",
        [
            "bedrock", "agentcore", "generative", "ai-foundry", "heatwave-ai",
            "iot-digital-twin", "maestro", "grounded-advisor",
        ],
    ),
    (
        "data",
        [
            "rds", "dynamodb", "cosmos", "aurora", "database", "dba",
            "autonomous-db", "autonomous-database", "exadata", "goldengate",
            "mysql", "dbtools", "sql-analyst", "fusion-apps",
        ],
    ),
]

# Keywords whose presence forces a re-route override. Some names contain
# substrings that would mismatch precedence; codify hard overrides here.
HARD_OVERRIDES: list[tuple[re.Pattern, str]] = [
    # Solution / network / multi-cloud architects are platform/networking design
    (re.compile(r"network-architect$"), "networking"),
    (re.compile(r"network-topology"), "networking"),
    (re.compile(r"multi-cloud-architect$"), "platform"),
    (re.compile(r"solution-architect$"), "platform"),
    (re.compile(r"landing-zone"), "compliance"),
    (re.compile(r"governance-policy-guardrails$"), "compliance"),
    # Live-guard skills with policy/iam/rbac stay security
    (re.compile(r"iam-policy-compartment-guard$"), "security"),
    (re.compile(r"rbac-mutation-guard$"), "security"),
    (re.compile(r"role-assignment-guard$"), "security"),
    (re.compile(r"pim-jit-activation-guard$"), "security"),
    (re.compile(r"vault-key-destruction-guard$"), "security"),
    (re.compile(r"keyvault-rotation-purge-guard$"), "security"),
    (re.compile(r"network-security-rule-guard$"), "security"),
    # Cost-budget guards remain finops despite "guard"
    (re.compile(r"cost-budget-runaway-guard$"), "finops"),
    (re.compile(r"cost-budget-action-guard$"), "finops"),
    # Live IaC change guards are delivery
    (re.compile(r"iac-change-guard$"), "delivery"),
    (re.compile(r"resource-manager-stack-guard$"), "delivery"),
    (re.compile(r"arm-deployment-stack-guard$"), "delivery"),
    # Rollout / deployment guards are delivery
    (re.compile(r"rollout-guard$"), "delivery"),
    (re.compile(r"deployment-guarded-operator$"), "delivery"),
    (re.compile(r"pipeline-approval-operator$"), "delivery"),
    (re.compile(r"serverless-release-guard$"), "delivery"),
    (re.compile(r"slot-swap-guard$"), "delivery"),
    (re.compile(r"app-service-production-readiness$"), "platform"),
    (re.compile(r"app-service.*$"), "platform"),
    (re.compile(r"serverless-production-readiness$"), "platform"),
    (re.compile(r"event-driven-architecture-review$"), "platform"),
    # AKS / EKS / OKE / ECS / Fargate platform operators
    (re.compile(r"aks-platform-operator$"), "platform"),
    (re.compile(r"eks-platform-operator$"), "platform"),
    (re.compile(r"oke.*$"), "platform"),
    (re.compile(r"ecs-fargate-platform-operator$"), "platform"),
    (re.compile(r"ecs-service-remediation-operator$"), "platform"),
    (re.compile(r"compute-platform-operator$"), "platform"),
    (re.compile(r"compute-instance-agent-operator$"), "platform"),
    (re.compile(r"ec2-compute-operations-steward$"), "platform"),
    (re.compile(r"cosmosdb-platform-operator$"), "platform"),
    (re.compile(r"container-platform-engineer$"), "platform"),
    (re.compile(r"environment-operator$"), "platform"),
    # Cosmos / DB performance/dev/audit are data
    (re.compile(r"cosmosdb-application-developer$"), "data"),
    (re.compile(r"cosmosdb-performance-investigator$"), "data"),
    (re.compile(r"keyvault-secret-lifecycle-auditor$"), "security"),
    (re.compile(r"key-vault-secret-lifecycle-auditor$"), "security"),
    (re.compile(r"keyvault-certificate-issuer-review$"), "security"),
    # Pod spec review = platform
    (re.compile(r"pod-spec-review$"), "platform"),
    # Backstage scaffolder = delivery
    (re.compile(r"scaffolder-template-review$"), "delivery"),
    # OCI registry = delivery
    (re.compile(r"registry-artifact-governor$"), "delivery"),
    # Storage backup steward = resilience
    (re.compile(r"storage-backup-steward$"), "resilience"),
    (re.compile(r"data-protection-backup-steward$"), "resilience"),
    (re.compile(r"recovery-service-operator$"), "resilience"),
    # Observability investigators
    (re.compile(r"observability-investigator$"), "observability"),
    (re.compile(r"observability-incident-responder$"), "observability"),
    (re.compile(r"resource-health-incident-triage$"), "observability"),
    (re.compile(r"support-incident-coordinator$"), "observability"),
    (re.compile(r"daily-operations-briefing-coordinator$"), "observability"),
    (re.compile(r"performance-investigator$"), "observability"),
    (re.compile(r"rds-aurora-performance-investigator$"), "data"),
    # Maestros are routing skills — bucket as ai (router/judgment)
    (re.compile(r"-maestro$"), "ai"),
    (re.compile(r"^terraform-maestro$"), "delivery"),
    # Migration cutover architects = delivery
    (re.compile(r"migration-cutover-architect$"), "delivery"),
    (re.compile(r"migrate-landing-zone-cutover$"), "delivery"),
    # Skill designer
    (re.compile(r"agent-skill-designer$"), "delivery"),
    # Identity / RBAC reviews
    (re.compile(r"rbac-review$"), "security"),
    (re.compile(r"identity-governance-review$"), "compliance"),
    (re.compile(r"identity-access-governor$"), "compliance"),
    (re.compile(r"entra-id-specialist$"), "security"),
    # Generative AI dev
    (re.compile(r"generative-ai-developer$"), "ai"),
    (re.compile(r"ai-foundry-ops-governor$"), "ai"),
    (re.compile(r"heatwave-ai-specialist$"), "ai"),
    (re.compile(r"iot-digital-twin-engineer$"), "ai"),
    (re.compile(r"agentcore$"), "ai"),
    (re.compile(r"oracle-oci-mcp-grounded-advisor$"), "ai"),
    (re.compile(r"bedrock-agent-security-governor$"), "security"),
    # Network architect / load balancer
    (re.compile(r"load-balancer-traffic-engineer$"), "networking"),
    (re.compile(r"private-endpoint-adoption-planner$"), "networking"),
    (re.compile(r"api-edge-delivery-review$"), "networking"),
    # Cost
    (re.compile(r"cost-anomaly-watch-coordinator$"), "finops"),
    (re.compile(r"cost-optimization-governor$"), "finops"),
    (re.compile(r"cost-finops-analyst$"), "finops"),
    (re.compile(r"cost-estimation-review$"), "finops"),
    (re.compile(r"cloud-price-advisor$"), "finops"),
    (re.compile(r"chargeback-allocation-review$"), "finops"),
    # Compliance / security posture
    (re.compile(r"security-posture-hardening$"), "security"),
    (re.compile(r"compliance-evidence-mapper$"), "compliance"),
    (re.compile(r"security-compliance-reviewer$"), "compliance"),
    (re.compile(r"cloud-guard-responder$"), "security"),
    # Resilience
    (re.compile(r"resilience-bcdr-review$"), "resilience"),
    # Subscription / governance
    (re.compile(r"subscription-resource-organization$"), "compliance"),
    (re.compile(r"governance-policy-guardrails$"), "compliance"),
    (re.compile(r"limits-capacity-planner$"), "platform"),
    (re.compile(r"resource-search-inventory-analyst$"), "platform"),
    # Platform automation / DevOps
    (re.compile(r"platform-automation-devops$"), "delivery"),
    (re.compile(r"ci-cd-release-engineer$"), "delivery"),
    (re.compile(r"non-destructive-task-automation-advisor$"), "delivery"),
    (re.compile(r"ticket-triage-escalation-coordinator$"), "observability"),
    # Pipeline / hotfix / serverless rollout corrector
    (re.compile(r"pipeline-fix-operator$"), "delivery"),
    (re.compile(r"deployment-hotfix-operator$"), "delivery"),
    (re.compile(r"serverless-rollout-corrector$"), "delivery"),
    (re.compile(r"iac-patch-executor$"), "delivery"),
    (re.compile(r"iac-change-safety-review$"), "delivery"),
    (re.compile(r"change-impact-advisor$"), "delivery"),
    # DynamoDB / RDS modeling = data
    (re.compile(r"dynamodb-data-modeling-performance-review$"), "data"),
    (re.compile(r"dbtools-sql-analyst$"), "data"),
    (re.compile(r"goldengate-replication-operator$"), "data"),
    (re.compile(r"database-platform-dba$"), "data"),
    (re.compile(r"autonomous-database-architect$"), "data"),
    (re.compile(r"autonomous-db-lifecycle-guard$"), "data"),
    (re.compile(r"exadata-platform-architect$"), "platform"),
    (re.compile(r"exadata-database-architect$"), "data"),
    (re.compile(r"fusion-apps-environment-operator$"), "platform"),
    # Architects (broad)
    (re.compile(r"^aws-solution-architect$"), "platform"),
    (re.compile(r"^oci-solution-architect$"), "platform"),
    (re.compile(r"^oci-multi-cloud-architect$"), "platform"),
    (re.compile(r"^azure-landing-zone-architect$"), "compliance"),
    (re.compile(r"^aws-landing-zone-governor$"), "compliance"),
    (re.compile(r"^aws-network-architect$"), "networking"),
    (re.compile(r"^oci-network-architect$"), "networking"),
]


def classify(skill_name: str) -> str:
    # Apply hard overrides first.
    for pat, cat in HARD_OVERRIDES:
        if pat.search(skill_name):
            return cat

    name_l = skill_name.lower()
    for cat, keywords in CATEGORY_RULES:
        for kw in keywords:
            # match whole word-ish on hyphen boundaries
            if kw in name_l:
                return cat
    return "platform"


def git_last_date(path: Path) -> str | None:
    try:
        out = subprocess.check_output(
            ["git", "log", "-1", "--format=%cs", "--", str(path)],
            cwd=ROOT,
            text=True,
        ).strip()
        if out and re.match(r"^\d{4}-\d{2}-\d{2}$", out):
            return out
    except subprocess.CalledProcessError:
        return None
    return None


def find_frontmatter_bounds(text: str) -> tuple[int, int] | None:
    """Return (start_after_open_fence, end_before_close_fence) line indices."""
    lines = text.splitlines(keepends=True)
    if not lines or lines[0].rstrip("\n") != "---":
        return None
    for i in range(1, len(lines)):
        if lines[i].rstrip("\n") == "---":
            return (1, i)
    return None


def update_skill_md(path: Path, dry_run: bool) -> tuple[bool, str, str | None, str | None]:
    """Returns (changed, skill_name, applied_updated, applied_category)."""
    text = path.read_text(encoding="utf-8")
    bounds = find_frontmatter_bounds(text)
    if bounds is None:
        return (False, path.parent.name, None, None)

    lines = text.splitlines(keepends=True)
    fm_start, fm_end = bounds  # fm_end is the closing '---' index

    # Locate metadata block
    meta_idx = None
    for i in range(fm_start, fm_end):
        if lines[i].startswith("metadata:"):
            meta_idx = i
            break
    if meta_idx is None:
        return (False, path.parent.name, None, None)

    # Find end of metadata block (next non-indented line within frontmatter)
    meta_block_end = fm_end
    for i in range(meta_idx + 1, fm_end):
        line = lines[i]
        if line.strip() == "":
            continue
        if not (line.startswith(" ") or line.startswith("\t")):
            meta_block_end = i
            break

    meta_lines = lines[meta_idx + 1 : meta_block_end]

    has_updated = any(
        re.match(r"^\s+updated\s*:", ln) for ln in meta_lines
    )
    has_category = any(
        re.match(r"^\s+category\s*:", ln) for ln in meta_lines
    )

    skill_name = path.parent.name
    new_updated = None
    new_category = None
    insertions: list[str] = []

    if not has_updated:
        date = git_last_date(path) or "2026-05-05"
        new_updated = date
        insertions.append(f'  updated: "{date}"\n')

    if not has_category:
        # Read declared name from frontmatter if available; fall back to dir
        name_in_fm = None
        for i in range(fm_start, fm_end):
            m = re.match(r"^name:\s*(.+)$", lines[i].rstrip("\n"))
            if m:
                name_in_fm = m.group(1).strip().strip('"').strip("'")
                break
        cat = classify(name_in_fm or skill_name)
        new_category = cat
        insertions.append(f"  category: {cat}\n")

    if not insertions:
        return (False, skill_name, None, None)

    # Insert after the last existing metadata sub-line (keep ordering stable).
    # Find the last non-blank line within meta_lines.
    insert_at = meta_block_end
    # walk back over trailing blank lines
    while insert_at - 1 > meta_idx and lines[insert_at - 1].strip() == "":
        insert_at -= 1

    new_lines = lines[:insert_at] + insertions + lines[insert_at:]

    if not dry_run:
        path.write_text("".join(new_lines), encoding="utf-8")

    return (True, skill_name, new_updated, new_category)


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--dry-run", action="store_true")
    args = ap.parse_args()

    skill_files = sorted(SKILLS_DIR.glob("*/*/SKILL.md"))
    if not skill_files:
        print("ERROR: no SKILL.md files found", file=sys.stderr)
        return 2

    changed = 0
    cat_counts: dict[str, int] = {}
    rows: list[tuple[str, str | None, str | None]] = []

    for sf in skill_files:
        ch, name, upd, cat = update_skill_md(sf, args.dry_run)
        if ch:
            changed += 1
        if cat:
            cat_counts[cat] = cat_counts.get(cat, 0) + 1
        rows.append((name, upd, cat))

    mode = "DRY-RUN" if args.dry_run else "APPLIED"
    print(f"{mode}: {changed} of {len(skill_files)} SKILL.md files updated")
    print("Category distribution:")
    for c in sorted(cat_counts):
        print(f"  {c}: {cat_counts[c]}")

    if args.dry_run:
        print("\nPer-skill assignments:")
        for name, upd, cat in rows:
            print(f"  {name}: updated={upd} category={cat}")

    return 0


if __name__ == "__main__":
    sys.exit(main())
