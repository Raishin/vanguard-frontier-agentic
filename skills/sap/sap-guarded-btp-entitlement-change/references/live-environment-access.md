# Live environment access — SAP Guarded BTP Entitlement Change

This reference defines credential setup, role requirements, command restrictions by step, and audit log format for guarded BTP entitlement change sessions.

## Credential requirements

### BTP Entitlements API (Cloud Management Service)

Required service instance and binding:
- Service: `cis` (Cloud Management Service), plan: `central` (for global account operations)
- Required scope for read-only access (steps 8–9): `EntitlementsManagement.View`
- Required scope for write access (step 14): `EntitlementsManagement.Edit`
- Token endpoint: from service key `uaa.url` + `/oauth/token` with client credentials grant

Not permitted:
- Global Account Administrator user credentials for API calls — use service key with minimum scope
- Service plan `local` (subaccount-scoped only) for global account entitlement changes

### BTP Subscription Management API

Required service instance:
- Service: `saas-registry`, plan: `application` or `service`
- Required scope for read-only (step 8): `SaaS-Registry.read`
- Required scope for write (step 14): `SaaS-Registry.write`

### BTP Usage Data Management Service

Required service instance:
- Service: `uas` (Usage Data Management Service), plan: `reporting-ga-admin` (for global account usage reports)
- Required scope: `uas.usage.report.read` (read-only; no write scope exists)

### BTP Cockpit (human operator access)

Read-only (steps 1–13):
- Role collection: `Global Account Viewer` (read entitlement assignments without modification rights)

Write (step 14 only):
- Role collection: `Entitlements Administrator` scoped to the specific global account
- Not permitted: `Global Account Administrator` role collection (grants directory management, trust configuration, and subaccount creation rights beyond entitlement scope)

## Step-by-step access matrix

| Step | Live system access | Access type | Credential scope |
|------|--------------------|-------------|-----------------|
| 1–7 | None | Advisory only | Not applicable |
| 8 | BTP Entitlements API / Usage API / cockpit | Read-only | EntitlementsManagement.View + uas.usage.report.read |
| 9 | BTP Entitlements API / Usage API | Read-only (diff and cost baseline) | EntitlementsManagement.View + uas.usage.report.read |
| 10–13 | None | Advisory only | Not applicable |
| 14 | BTP Entitlements API / Subscription API | Mutating — entitlement/quota/subscription only | EntitlementsManagement.Edit + SaaS-Registry.write |
| 15 | BTP Entitlements API / Usage API / cockpit | Read-only | EntitlementsManagement.View + uas.usage.report.read |
| 16–17 | None | Reporting only | Not applicable |

## Audit log format

Every command executed (steps 8, 9, 14, 15) must be logged before execution:

```
[BTP_ENTITLEMENT_AUDIT_LOG]
Step: <step number and name>
Timestamp (UTC): <ISO 8601>
Global account: <global account ID and display name>
Subaccount: <subaccount GUID and display name>
Tool: <BTP Entitlements API | BTP Subscription API | Usage Data API | BTP cockpit>
Command/API call: <exact API endpoint and method, credentials omitted>
Service(s) in scope: <service name, service plan, quota delta>
Cost delta: <estimated monthly cost change>
Output summary: <one-line summary>
Errors/Warnings: <none | list>
Redactions applied: <yes/no>
Dual approval gate cleared: <yes (step 13 documented: platform owner + FinOps) | not yet reached>
```

## Pre-execution gate checklist (step 13 → step 14)

Before executing any entitlement, quota, or subscription change, all of the following must be true:

- [ ] Step 1: Change type, service name, service plan, and billing model classified
- [ ] Step 2: Target global account ID, subaccount ID, and environment tier confirmed
- [ ] Step 3: Criticality level and cost sensitivity assigned
- [ ] Step 4: Requester name and role documented
- [ ] Step 5: Platform owner and FinOps approver names, roles, and authorization documented (both different from requester)
- [ ] Step 6: Change management or FinOps budget ticket number confirmed
- [ ] Step 7: Entitlement change list with service, plan, quota delta, and subaccount documented
- [ ] Step 8: Current entitlement assignments and consumption baseline obtained (live evidence on record)
- [ ] Step 9: Diff with quota delta and cost-impact assessment (monthly cost delta) documented
- [ ] Step 10: Blast radius document confirmed (dependent applications, disruption risk, cost risk)
- [ ] Step 11: Rollback procedure (exact quota values and API calls to restore) documented and confirmed feasible
- [ ] Step 12: SoD confirmed — requester differs from both platform owner and FinOps approver
- [ ] Step 13: Explicit written approval from platform owner on record, naming services and quota amounts
- [ ] Step 13: Explicit written approval from FinOps approver on record, naming services and quota amounts

If any item is unchecked, refuse step 14 execution until it is resolved.
If the change is an entitlement increase and FinOps approval is missing, refuse step 14 execution regardless of other approvals.
