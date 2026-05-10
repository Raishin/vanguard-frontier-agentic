# Workflow and output contract

Use this reference only when performing a full IONOS DCD topology review or blast-radius assessment.

## Review domains

Check these areas before giving a verdict:

- Datacenter composition: server placement, NIC configuration, volume attachments, and LAN assignments
- Multi-AZ strategy: are servers distributed across availability zones within the datacenter or across datacenters?
- Private LAN segmentation: are application tiers isolated via separate LANs with appropriate NIC assignments?
- Firewall posture: are inbound and outbound rules scoped correctly per NIC; are public-facing NICs minimized?
- Volume layout: are boot volumes and data volumes separated; is encryption enabled; are detach risks understood?
- Blast radius: what is the full impact scope if the proposed change fails mid-execution?
- GDPR data residency: does the datacenter region match the declared data processing location?
- IaC alignment: does the live DCD topology match the declared Terraform or IaC state?

## Safe workflow

1. **Frame scope**
   - Datacenter name and region:
   - Declared GDPR processing location:
   - Change type: new resource, topology restructure, NIC modification, volume change, LAN change
   - Business criticality and owner:
   - Required outcome:
   - Explicit non-goals:
2. **Collect evidence**
   - Require a current DCD topology export or snapshot before assessing any structural change.
   - Inspect IaC (Terraform) if live DCD export is not available, and note the gap.
   - Label each finding as `live evidence`, `user-provided sanitized evidence`, `documentation-based`, or `inference`.
3. **Stress-test blast radius**
   - What resources share this datacenter and are affected by the proposed change?
   - What can disrupt server connectivity if a LAN or NIC change fails?
   - What can make volume data inaccessible if a detach or move fails mid-execution?
   - What rollback path exists, and does it require downtime?
   - What GDPR or compliance flag does this change trigger?
4. **Recommend the smallest safe action**
   - Prefer staged changes (one resource at a time) over broad topology restructuring.
   - Always require a rollback path before advising any structural change.
   - If topology evidence is missing, require it before giving any verdict.

## Output contract

Return this structure:

```markdown
# IONOS DCD Topology Review: <scope>
## Executive verdict
- Status: READY / READY WITH RISKS / NOT READY / NEEDS EVIDENCE
- Biggest risk:
- Evidence level:
## Scope and assumptions
- Datacenter region:
- GDPR residency: CONFIRMED / MISMATCH / UNKNOWN
- Confirmed:
- Unknown:
- Out of scope:
## Blast-radius assessment
- Impact scope: <resources affected if change fails>
- Rollback path: <documented or absent>
## Findings
| Severity | Domain | Finding | Evidence | Why it matters | Minimum safe action |
|---|---|---|---|---|---|
## Recommended actions
1. <action> — owner: <owner>, validation: <check>, rollback: <rollback>
## Evidence gaps
- <gap or explicit none>
## Residual risk
- <risk or explicit none>
```
