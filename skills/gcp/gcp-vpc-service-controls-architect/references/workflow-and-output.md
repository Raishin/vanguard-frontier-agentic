# Workflow and output contract

Use this reference only when performing the full VPC-SC review, implementation guidance, violation triage, or production-readiness pass.

## Review domains

Check these areas before giving a verdict:

- Access policy: org ID, policy name, policy resource name (`accessPolicies/POLICY_NUMBER`)
- Service perimeters: name, perimeter type (regular/bridge), enforcement mode (enforced/dry-run), resources (projects)
- Restricted services: which GCP service APIs are restricted in each perimeter
- Ingress/egress rules: principal types, sources/targets, services, ACM access level conditions
- Bridge perimeters: which regular perimeters are bridged, resource overlap, traffic patterns
- ACM access levels: level name, conditions (IP ranges, device policy, identity), combining function
- Serverless workloads: Cloud Functions, Cloud Run, Dataflow — confirmed API egress patterns, VPC Accessible Services configuration
- Violation logs: GOOGLE_API_VIOLATION, POLICY_VIOLATION, DRY_RUN_VIOLATION — principal, service, direction

## Safe workflow

1. **Frame scope**
   - Org and access policy confirmed:
   - Perimeters in scope:
   - Environment (prod/staging/dev):
   - Enforcement mode status:
   - Required outcome:
   - Explicit non-goals:
2. **Collect evidence**
   - Prefer sanitized `gcloud access-context-manager perimeters list`, `gcloud access-context-manager perimeters describe`, or Terraform state exports.
   - For violations: Cloud Logging exports filtered for `protoPayload.status.code=PERMISSION_DENIED` with VPC-SC context.
   - Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Stress-test risk**
   - Which perimeters are in enforcement mode without dry-run validation history?
   - Which serverless workloads inside a perimeter have uncovered API egress paths?
   - Which dry-run violations represent legitimate workflows vs. unexpected access?
   - Which bridge perimeters increase blast radius unnecessarily?
   - What evidence is missing?
4. **Recommend the smallest safe action**
   - Prefer dry-run first, minimal ingress/egress rules with ACM conditions, and staged enforcement rollout.
   - If the safest action is to stop and gather dry-run violation evidence, say that plainly.

## Output contract

Return this structure:

```markdown
# GCP VPC Service Controls Review: <scope>
## Executive verdict
- Status: READY / READY WITH RISKS / NOT READY / NEEDS EVIDENCE
- Biggest risk:
- Evidence level:
## Access policy and perimeter inventory
| Perimeter | Type | Enforcement mode | Resources | Restricted services |
|---|---|---|---|---|
## Dry-run violation summary
| Principal | Target service | Direction | Violation reason | Legitimate? |
|---|---|---|---|---|
## Bridge perimeter assessment
| Bridge perimeter | Connects | Risk | Alternative |
|---|---|---|---|
## ACM access level review
| Level name | Conditions | Applied to rules | Gap |
|---|---|---|---|
## Serverless workload blind spots
| Workload | Type | API egress covered | Action needed |
|---|---|---|---|
## Recommended actions
1. <action> — owner: <owner>, validation: <check>, rollback: <rollback>
## Residual risk
- <risk or explicit none>
```
