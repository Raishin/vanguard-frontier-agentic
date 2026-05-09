# Workflow and output contract

Use this reference only when performing the full CI/CD pipeline review, security audit, implementation guidance, or production-readiness pass.

## Review domains

Check these areas before giving a verdict:
- Cloud Build: cloudbuild.yaml structure, private pool vs. public pool, build trigger inventory
- Cloud Deploy: delivery pipeline stages, target configurations, promotion criteria
- Artifact Registry: repository type, region, retention policy, public access settings
- Service accounts: Cloud Build SA permissions (minimum: Cloud Run Admin + Artifact Registry Writer + GKE Developer)
- SLSA provenance: provenance generation enabled, Binary Authorization policy configured
- Approval gates: required approvers per stage, timeout configuration
- Skaffold: version pinned and compatible with Cloud Deploy release version

## Safe workflow

1. **Frame scope**
   - Project/region/environment:
   - Business criticality and owner:
   - Data classification and compliance driver:
   - Required outcome:
   - Explicit non-goals:
2. **Collect evidence**
   - Prefer live GCP CLI/API read-only evidence if available.
   - Otherwise inspect repository IaC/config (cloudbuild.yaml, skaffold.yaml), sanitized user evidence, or official Google Cloud docs.
   - Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Stress-test risk**
   - What service account permissions exceed minimum required?
   - What artifacts can be deployed without provenance verification?
   - What stages lack approval gates?
   - What evidence is missing?
4. **Recommend the smallest safe action**
   - Prefer narrow scope, staged rollout, validation, and rollback.
   - If the safest action is to stop and gather evidence, say that plainly.

## Output contract

Return this structure:
```markdown
# GCP Cloud Build Deploy CI/CD Operator: <scope>
## Executive verdict
- Status: READY / READY WITH RISKS / NOT READY / NEEDS EVIDENCE
- Biggest risk:
- Evidence level:
## Scope and assumptions
- Confirmed:
- Unknown:
- Out of scope:
## Findings
| Severity | Finding | Evidence | Why it matters | Minimum safe action |
|---|---|---|---|---|
## Recommended actions
1. <action> — owner: <owner>, validation: <check>, rollback: <rollback>
## Validation
- Commands or checks:
- Expected result:
## Residual risk
- <risk or explicit none>
```
