# Workflow and Output Contract

## Safe Workflow

1. **Frame the incident**  
   Confirm exact symptom, affected resource or workload, incident start time, environment, subscription or resource-group boundary, region, and current customer impact.
2. **Check platform-health signals first**  
   Review Resource Health status for the named resource or scoped set of resources. Check whether the signal is `Available`, `Unavailable`, `Degraded`, or `Unknown`, and capture reason/details if present.
3. **Check broader service-impact signals**  
   Review Service Health events relevant to the subscription, services, region, and time window. Distinguish active issues, planned maintenance, advisories, and resolved history.
4. **Correlate with Activity Log and alert evidence**  
   Check recent activity-log events, Resource Health notifications, and Service Health or activity-log alert behavior to see whether the timeline matches a platform event, a user or automation change, or neither.
5. **Classify the likely failure domain**  
   Put the incident in one of these bins: `likely provider incident`, `likely tenant misconfiguration or change`, `resource-local issue with no broad Azure evidence`, or `unresolved`.
6. **Return bounded next actions**  
   Recommend the next safest move: monitor, escalate to Microsoft, inspect specific tenant changes, hand off to application/SRE owners, or collect missing evidence.

## Role-Specific Stress Checks

- Do not treat `Unknown` as proof of Azure outage. It is a signal gap, not a verdict.
- Do not treat Service Health absence as proof Azure is healthy for the affected resource.
- Do not treat Resource Health `Unavailable` as proof the tenant did nothing wrong; Microsoft documents both platform and non-platform events.
- Check timing. If the incident started immediately after a deployment, policy change, networking change, identity change, or stop/start action, tenant causality is still on the table.
- Distinguish subscription-level or service-level notices from resource-level degradation.
- Do not recommend broad failover, rollback, or routing changes before confirming blast radius and approval path.
- Do not rewrite history from alerts alone; alerts show configured detection, not the full causal chain.

## Output Template

```markdown
# Azure Health Triage: <scope>

## Current verdict
- Status: LIKELY PROVIDER INCIDENT / LIKELY TENANT CHANGE OR MISCONFIGURATION / RESOURCE-LOCAL ISSUE / UNRESOLVED
- Confidence: HIGH / MEDIUM / LOW
- Evidence level: live evidence / documentation-based / sanitized evidence / inference

## Incident frame
- Affected scope:
- Subscription or resource group:
- Region:
- Resource(s):
- Symptom:
- Reported start time:

## Health evidence
| Signal | Finding | Time window | Evidence type | What it proves | What it does not prove |
|---|---|---|---|---|---|

## Blast radius assessment
- Single resource:
- Multiple resources:
- Service/region pattern:
- User-visible impact:

## Likely failure domain
- Provider incident evidence:
- Tenant-side change evidence:
- Remaining unknowns:

## Immediate next actions
1.
2.
3.

## Escalation and handoff
- Escalate to:
- Include these artifacts:
- Do not claim:
```

## Red Flags

- The request says "Azure is down" but no subscription, region, resource, or time boundary is given.
- The conclusion relies on social media, public status chatter, or a single screenshot without tenant-scoped evidence.
- Resource Health shows no current issue, but the answer still declares a confirmed provider outage.
- A recent deployment or access-policy change exists, but the analysis ignores it.
- The skill is being pushed into full RCA when only first-pass platform-health triage is justified.
- The response recommends destructive remediation before separating provider signal from tenant error.
