# Workflow and Output Contract

## Safe Workflow

1. **Classify migration scope**
   - application, server, database, web app, or mixed wave,
   - pilot, low-risk wave, or business-critical cutover,
   - same-region, cross-region, or hybrid coexistence phase.
2. **Check discovery and assessment quality**
   - Was the workload actually discovered?
   - Is performance data sufficient and current?
   - Were workloads tagged and grouped correctly?
   - Are recommendations point-in-time snapshots that may already be stale?
3. **Check landing-zone readiness**
   - connectivity,
   - identity,
   - policy and governance,
   - target subscriptions and resource groups,
   - DNS,
   - monitoring,
   - and operator ownership.
4. **Check dependency realism**
   - upstream/downstream systems,
   - database coupling,
   - authentication dependencies,
   - certificate or secret paths,
   - network path and private access requirements.
5. **Check permissions and tooling**
   - who can assess,
   - who can deploy,
   - who can execute migration,
   - whether least-privilege roles are used,
   - whether temporary elevated access is bounded.
6. **Check cutover mechanics**
   - cutover trigger,
   - freeze window,
   - validation gates,
   - fallback time limit,
   - data consistency checks,
   - DNS or traffic switch behavior.
7. **Check rollback credibility**
   - can the source stay authoritative,
   - how long rollback remains viable,
   - what changes become irreversible,
   - who approves rollback.
8. **Check post-cutover operations**
   - alerting,
   - dashboards,
   - ownership,
   - incident path,
   - cost visibility,
   - decommission sequencing.
9. **Return a cutover go / no-go verdict**
   - what is ready,
   - what is not,
   - what evidence is missing,
   - and what must be proven before execution.

## Role-Specific Stress Checks

- Reject “assessment says ready” if dependency and landing-zone evidence are weak.
- Reject migration waves built from stale or low-quality discovery data.
- Reject landing-zone claims that skip hybrid connectivity, DNS, RBAC, or monitoring readiness.
- Reject cutover plans with no explicit freeze window, validation owner, or rollback decision point.
- Reject permission models that require permanent broad admin rights for routine migration steps.
- Reject plans that treat migration tooling recommendations as architecture truth.
- Reject rollback claims that ignore replicated data divergence, DNS TTLs, or irreversible writes.

## Output Template

```markdown
# Azure Migration Cutover Review: <scope>

## Verdict
- Status: READY / READY WITH RISKS / NOT READY
- Biggest risk:
- Evidence level: live evidence / documentation-based / sanitized evidence / inference

## Scope
- Migration wave:
- Workload type:
- Source environment:
- Target landing zone:
- Requested cutover date or window:

## Findings
| Area | Finding | Severity | Evidence | Recommendation | Owner |
|---|---|---|---|---|---|

## Cutover readiness review
| Control area | Expected state | Observed state | Gap | Blocking |
|---|---|---|---|---|
| Discovery and assessment quality |  |  |  |  |
| Landing-zone readiness |  |  |  |  |
| Dependency mapping |  |  |  |  |
| Permissions and tooling |  |  |  |  |
| Cutover validation gates |  |  |  |  |
| Rollback posture |  |  |  |  |
| Post-cutover operations |  |  |  |  |

## Safe next actions
1.
2.
3.

## Open questions
- 
```

## Red Flags

- The team has no current assessment data or cannot explain how old the data is.
- The landing zone exists on paper but connectivity, DNS, or monitoring is still incomplete.
- A migration wave groups systems by convenience instead of dependency reality.
- The plan has no defined rollback cut-off or authority.
- Target ownership after cutover is ambiguous.
- The cutover depends on undocumented MCP or automation behavior.
