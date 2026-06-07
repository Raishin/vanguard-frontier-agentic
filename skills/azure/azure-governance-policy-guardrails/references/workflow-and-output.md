# Workflow and output contract for Azure Governance Policy Guardrails

## Minimal safe workflow

1. Classify request: new policy, initiative, assignment, exemption, remediation, enforcement rollout, or compliance review.
2. Ground behavior in Microsoft Learn through the user's configured documentation MCP.
3. Identify scope and inheritance: management group, subscription, resource group, excluded scopes, and exemptions.
4. Review effect and mode: audit, deny, modify, DINE, disabled, manual, and assignment enforcement mode.
5. Stress test blast radius: deployment pipelines, existing resources, remediation identity, exemptions, and rollback.
6. Stage through audit or DoNotEnforce/canary before broad enforcement unless risk demands immediate action.
7. Return verdict with blockers and safe rollout sequence.

## Output contract

```markdown
## Verdict
<safe to stage | conditional | unsafe | docs-only advisory>

## Evidence level
- Documentation: <sources used>
- Policy evidence: <policy_review|compliance_sample|canary_proven|not sampled>

## Findings
1. <finding> — Evidence: <docs_only|policy_review|compliance_sample|inference>

## Blast radius
- Scope: <summary>
- Pipelines/resources at risk: <summary>

## Safe rollout
1. <stage>

## Blockers
- <blocker>
```

## Pushback triggers

Push back on broad deny first, remediation identities with excessive rights, exemptions with no expiry, compliance percentages without applicability review, or assignment changes without rollback.
