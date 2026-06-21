# Workflow and output contract — SAP Security IAM GRC and SoD Review

Use this reference for all finding classification, SoD conflict risk assignment, remediation path selection, and output formatting.

## IAM finding taxonomy

| Domain | Finding class | Description |
|--------|--------------|-------------|
| `ias` | `missing-mfa-enforcement` | IAS application or user group with privileged access has no MFA or risk-based authentication policy enforced |
| `ias` | `unconfigured-corporate-idp` | IAS tenant accepts login without federated corporate IdP — users authenticate directly to IAS without corporate credential lifecycle |
| `ias` | `unused-application-assignment` | IAS application assignment configured for a BTP application that is no longer active |
| `ips` | `production-provisioning-no-approval` | IPS connector provisioning to a production target (BTP, S/4HANA, SuccessFactors) without an approval workflow |
| `ips` | `overly-broad-transformation` | IPS transformation script provisions all source groups to the target without filtering — over-provisioning risk |
| `ips` | `orphaned-account-source` | IPS source connector returns deprovisioned user records that remain active in target systems |
| `xsuaa` | `over-broad-scope` | XSUAA scope grants access to a broad resource category (e.g., all documents, all tenants) where a narrower scope would suffice |
| `xsuaa` | `wildcard-authorization-value` | XSUAA role template or authorization object uses wildcard (`*`) for organizational-level attributes (company code, plant, sales org) |
| `xsuaa` | `missing-foreign-scope-reference` | xs-security.json references foreign scopes from another app without explicit `granted-apps` restriction |
| `xsuaa` | `direct-user-role-assignment` | Role collection assigned directly to a named user rather than to an IdP group — access lifecycle bypassed |
| `grc` | `incomplete-sod-ruleset` | GRC Access Control ruleset does not cover key business process combinations (e.g., FI-MM SoD, procure-to-pay, order-to-cash) |
| `grc` | `stale-mitigation-control` | Mitigation control assigned to a SoD conflict has expired or has no owner/approver on record |
| `sod` | `critical-sod-conflict` | User or role set has access to incompatible transaction pairs at critical risk level (e.g., create vendor + approve payment, create PO + goods receipt + invoice verification) |
| `sod` | `unmitigated-critical-conflict` | Critical SoD conflict exists with no approved compensating mitigation control — immediate escalation required |
| `sod` | `privilege-accumulation` | User has accumulated access across multiple roles that individually pass SoD but together create an incompatible combination |

## SoD conflict risk classification

| Risk level | Criteria |
|-----------|---------|
| `critical` | SoD conflict in a key control process: financial fraud enablement (procure-to-pay, order-to-cash, financial close), data exfiltration path, or compliance-mandated separation (SOX IT general controls) |
| `high` | SoD conflict with significant operational risk: access to incompatible transaction pairs that could cause material financial error or unauthorized system configuration change |
| `medium` | SoD conflict in non-key control process, or privilege accumulation without a direct incompatible transaction pair |
| `low` | Potential future SoD risk from role design that, if a second role were added, would create a conflict |

## Remediation path decision tree

For each finding:

1. **Is this an unmitigated critical SoD conflict?**
   - Yes → `critical`. Immediately escalate to GRC / audit team. Do not approve, defer, or propose a workaround without documented compensating control and approver sign-off. State this explicitly in the response.
   - No → continue.

2. **Is this a critical SoD conflict with an existing mitigation control?**
   - Yes → `critical` until the mitigation control is confirmed valid (non-expired, active owner, documented approval). Verify mitigation control validity before marking as managed risk.
   - No → continue.

3. **Is this excessive privilege in XSUAA (wildcard scope, over-broad role template)?**
   - Yes → `high`. Redesign the XSUAA scope to use the narrowest attribute set required. Replace wildcard organizational-level values with explicit values per the user's landscape. Redeploy xs-security.json and rebind the XSUAA service instance.
   - No → continue.

4. **Is this a missing MFA or unapproved IPS production provisioning?**
   - Yes → `high`. Enforce risk-based authentication in IAS for the affected application or user group. Add an IPS approval workflow for production target connectors. Do not defer MFA enforcement for privileged users.
   - No → continue.

5. **Is this a governance or lifecycle gap (orphaned account, stale mitigation, direct user assignment)?**
   - Yes → `medium`. Remove orphaned accounts from target systems via IPS deprovisioning or manual cleanup. Reassign stale mitigation controls to an active owner. Replace direct user role assignments with IdP group-mapped role collections.
   - No → classify as `low` and provide best practice guidance.

## Workflow

1. **Receive artifacts** — SoD conflict reports, role lists, xs-security.json files, IAS exports, IPS connector configurations, GRC ruleset exports, or user descriptions.
2. **Classify each finding** by IAM domain and finding class.
3. **Assign risk level** per SoD classification table above (critical / high / medium / low).
4. **Flag any unmitigated critical SoD conflicts** immediately — these must be escalated before any other remediation is discussed.
5. **Apply remediation decision tree** per finding.
6. **Prioritize** — unmitigated critical SoD conflicts first; then critical mitigated conflicts requiring validation; then high privilege/MFA findings; then medium lifecycle gaps; then low best-practice items.
7. **Return output** per the output contract below.

## Output contract

Return:

1. IAM domain and finding class per finding
2. Evidence label (documentation-based / user-provided evidence / inference)
3. Risk level per finding (critical / high / medium / low)
4. SoD conflict details (if applicable): transaction pair, affected user/role, GRC risk ID if provided
5. Recommended remediation action (role redesign, mitigation control assignment, IPS workflow addition, XSUAA scope reduction, MFA policy enforcement, etc.)
6. IAM posture after remediation
7. Escalation notice for any unmitigated critical SoD conflict — explicit statement that this requires GRC/audit team sign-off before proceeding
8. Prioritized remediation sequence
