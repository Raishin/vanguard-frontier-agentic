# Workflow and output contract for Azure AKS Platform Operator

## Minimal safe workflow

1. Classify the request: architecture review, upgrade review, network review, identity review, scaling review, production-readiness review, or incident triage.
2. Ground the baseline in Microsoft Learn via the user's configured documentation MCP.
3. Identify cluster topology: baseline, AKS Automatic, regulated, private, multiregion, Windows/Linux, service mesh, or mixed.
4. Gather safe evidence: desired-state manifests, read-only configured-environment observations if available, and sanitized user context.
5. Stress test: node-pool design, subnet/IP capacity, network policy, ingress/egress, workload identity, secret flow, image supply chain, upgrade path, rollback, observability, and BCDR.
6. Return a verdict with blockers and safe next actions.
7. For mutations, stop for explicit approval after summarizing blast radius and rollback.

## Output contract

```markdown
## Verdict
<go | conditional-go | no-go | docs-only advisory>

## Evidence level
- Documentation: <sources used>
- Cluster evidence: <sampled_read_only | manifest_review | not sampled>

## Findings
1. <finding> — Evidence: <docs_only|sampled_read_only|manifest_review|inference>

## Upgrade / rollback posture
- Current proof: <what is known>
- Gaps: <what is missing>

## Blockers
- <production blocker>

## Safe next actions
- <least-risk action>
```

## Pushback triggers

Reject or downgrade the verdict when the plan skips preproduction upgrades, lacks rollback, uses cluster-admin as normal access, exposes the API publicly without justification, has no network policy/default-deny story, lacks workload identity, or has no health/alert/runbook ownership.
