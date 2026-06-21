# Workflow and output contract — SAP Guarded BTP Entitlement Change

Use this reference for all step-by-step execution, command patterns, and output formatting.

## Step execution protocol

For each of the 17 steps, the following must be true before advancing to the next:

1. The step's required evidence is on record in this session.
2. The user has confirmed the evidence is accurate.
3. Any step-specific gate (e.g., cost-impact assessment, dual approval, SoD check) has been explicitly cleared.

If any of these conditions is false, the session pauses at the current step until resolved.

## Command patterns by step

### Step 8 — Read-only current state

#### BTP Entitlements API (read — current assignments)
```
GET https://entitlements-service.cfapps.{region}.hana.ondemand.com/entitlements/v1/globalAccountAllowances
  (returns all service plans assigned to the global account)

GET https://entitlements-service.cfapps.{region}.hana.ondemand.com/entitlements/v1/subAccountServicePlans
  ?subAccountGUID={subaccountGuid}
  (returns entitlements currently assigned to the specific subaccount)
```

#### BTP Usage Data Management API (read — current consumption baseline)
```
GET https://uas.cfapps.{region}.hana.ondemand.com/reports/v1/monthlyUsage
  ?fromDate={YYYY-MM}&toDate={YYYY-MM}&globalAccountId={globalAccountId}
  (returns metered consumption per service and subaccount for cost baseline)
```

#### BTP cockpit (read)
```
BTP Cockpit → Global Account → Entitlements → Entity Assignments
  → select subaccount → view current service plan assignments and quota
BTP Cockpit → Global Account → Entitlements → Service Assignments
  → view all subaccount assignments for a specific service
```

### Step 9 — Diff and cost-impact assessment

Document for each changed entitlement:

```
Service: <service name>
Plan: <service plan name>
Billing model: <free | metered | subscription | block>
Current quota: <current value or "not assigned">
Proposed quota: <proposed value or "remove">
Quota delta: <+N / -N / new / remove>
Estimated cost delta: <EUR/USD per month based on SAP BTP pricing for this plan>
  Source: <user-provided pricing confirmation | SAP contract reference | inference (label accordingly)>
Active consumers: <list of applications or services consuming this entitlement>
Risk if removed: <disruption to consumers identified above>
```

### Step 14 — Execute approved change

#### BTP Entitlements API (mutating — step 14 only, after step 13 dual approval gate is cleared)
```
PUT https://entitlements-service.cfapps.{region}.hana.ondemand.com/entitlements/v1/subAccountServicePlans
Body:
{
  "entitlementsToAdd": {
    "{subaccountGuid}": [
      { "serviceName": "<service>", "servicePlanName": "<plan>", "quota": <N>, "enable": true }
    ]
  },
  "entitlementsToDelete": {
    "{subaccountGuid}": [
      { "serviceName": "<service>", "servicePlanName": "<plan>" }
    ]
  }
}
```

#### BTP Subscription Management API (mutating — step 14 only)
```
POST https://{subaccountSubdomain}.{region}.hana.ondemand.com/saas-manager/v1/applications/{appName}/subscription
  (subscribe to a multitenant application — approved services only)

DELETE https://{subaccountSubdomain}.{region}.hana.ondemand.com/saas-manager/v1/applications/{appName}/subscription
  (unsubscribe — only when change type from step 1 is subscription removal and explicitly approved)
```

**Never call PUT entitlement or POST/DELETE subscription before step 13 dual approval gate is documented.**

## Step completion checklist

| Step | Evidence type | Completion marker |
|------|--------------|------------------|
| 1 | user-provided / documentation-based | Change type, service name, service plan, and billing model confirmed |
| 2 | user-provided evidence | Target global account ID, subaccount ID, and environment tier confirmed |
| 3 | user-provided evidence | Criticality level and cost sensitivity assigned |
| 4 | user-provided evidence | Requester name and role on record |
| 5 | user-provided evidence | Platform owner and FinOps approver names, roles, and authorization on record |
| 6 | user-provided evidence | Ticket number confirmed |
| 7 | user-provided evidence | Entitlement change list with service, plan, quota delta, and subaccount on record |
| 8 | live evidence | Current entitlement assignments and consumption baseline from global account |
| 9 | live evidence + user-provided / inference | Diff with quota delta and cost-impact assessment (monthly cost delta documented) |
| 10 | user-provided / inference | Blast radius document confirmed (dependent applications, disruption risk, cost risk) |
| 11 | user-provided evidence | Rollback procedure (entitlement reversion steps) confirmed feasible |
| 12 | user-provided evidence | SoD confirmed: requester ≠ platform owner ≠ FinOps approver |
| 13 | user-provided evidence | Explicit dual approval (platform owner + FinOps) on record, naming services and quota amounts |
| 14 | live evidence | Execution log with timestamp |
| 15 | live evidence | Post-change entitlement snapshot confirmed; dependent applications healthy; metered tracking active |
| 16 | all of the above | Audit record compiled |
| 17 | user-provided evidence | Report delivered and acknowledged |

## Output contract

Return after each step:

1. Step number and name
2. Evidence gathered for this step (labeled by evidence type)
3. Gate status: cleared / pending / blocked
4. Next step and what is needed to advance
5. Refusal marker if any gate condition is unmet (do not advance)
6. Cost-impact status if step 9 or step 13 (documented / missing / pending FinOps review)

Return after step 17:

1. Complete audit record (all 17 steps with evidence and timestamps)
2. Final change status (success / partial / failed)
3. Post-change entitlement and application health verification result
4. Cost-impact assessment summary (baseline → projected monthly delta)
5. Rollback status (not needed / standing by / triggered)
6. Report delivery confirmation
