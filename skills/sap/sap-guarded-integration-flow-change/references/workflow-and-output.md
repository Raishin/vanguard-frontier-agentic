# Workflow and output contract — SAP Guarded Integration Flow Change

Use this reference for all step-by-step execution, command patterns, and output formatting.

## Step execution protocol

For each of the 17 steps, the following must be true before advancing to the next:

1. The step's required evidence is on record in this session.
2. The user has confirmed the evidence is accurate.
3. Any step-specific gate (e.g., integration-owner approval, SoD check) has been explicitly cleared.

If any of these conditions is false, the session pauses at the current step until resolved.

## Command patterns by step

### Step 8 — Read-only current state

#### Cloud Integration OData API (read — deployed artifact status)
```
GET https://{tenant}.it-cpi{N}.cfapps.{region}.hana.ondemand.com/api/v1/IntegrationRuntimeArtifacts
  (returns all deployed iFlows with version and status)

GET https://{tenant}.it-cpi{N}.cfapps.{region}.hana.ondemand.com/api/v1/IntegrationRuntimeArtifacts('{artifactId}')
  (returns status: STARTED | ERROR | STOPPING for a specific artifact)
```

#### Cloud Integration Monitoring API (read — message processing)
```
GET https://{tenant}.it-cpi{N}.cfapps.{region}.hana.ondemand.com/api/v1/MessageProcessingLogs
  ?$filter=IntegrationFlowName eq '{iFlowName}' and Status eq 'FAILED'
  &$orderby=LogStart desc&$top=20
  (returns recent error messages as baseline for step 15 comparison)
```

### Step 9 — Diff of artifact changes

#### Cloud Integration Design API (read — retrieve current design workspace version)
```
GET https://{tenant}.it-cpi{N}.cfapps.{region}.hana.ondemand.com/api/v1/IntegrationDesigntimeArtifacts(Id='{artifactId}',Version='active')/$value
  (download current active design workspace version for diff comparison)

GET https://{tenant}.it-cpi{N}.cfapps.{region}.hana.ondemand.com/api/v1/IntegrationDesigntimeArtifacts(Id='{artifactId}',Version='active')
  (returns artifact metadata including version, description, and last modified timestamp)
```

Document in the diff:
- Changed iFlow steps (adapters, routing conditions, content modifier, script steps)
- Changed adapter configurations (endpoint URLs, authentication methods, retry settings)
- Changed value mappings or external parameters
- Changed security material references
- Version number delta: deployed version → proposed version

### Step 14 — Execute approved deployment

#### Cloud Integration OData API (mutating — step 14 only, after step 13 gate is cleared)
```
POST https://{tenant}.it-cpi{N}.cfapps.{region}.hana.ondemand.com/api/v1/DeployIntegrationDesigntimeArtifact
  ?Id='{artifactId}'&Version='{version}'
  Body: (empty for standard deploy; artifact must be in active workspace state)
```

#### Undeploy (only when change type from step 1 is undeploy and explicitly approved)
```
DELETE https://{tenant}.it-cpi{N}.cfapps.{region}.hana.ondemand.com/api/v1/IntegrationRuntimeArtifacts('{artifactId}')
```

**Never call POST deploy or DELETE undeploy before step 13 approval gate is documented.**

### Step 11 — Rollback (previous version redeploy)

#### Cloud Integration — restore previous version
```
1. Navigate to: Integration Suite → Cloud Integration → Design → locate package
2. Select artifact → Version History → identify last known good version
3. Activate previous version in design workspace
4. Execute step 14 pattern (POST deploy) with the previous version ID

OData API equivalent:
PUT https://{tenant}.it-cpi{N}.cfapps.{region}.hana.ondemand.com/api/v1/IntegrationDesigntimeArtifacts(Id='{artifactId}',Version='{previousVersion}')
  (restore previous version to active) — then POST deploy as per step 14
```

## Step completion checklist

| Step | Evidence type | Completion marker |
|------|--------------|------------------|
| 1 | user-provided / documentation-based | Artifact type and change type confirmed |
| 2 | user-provided evidence | Target tenant ID, tier, and sub-domain confirmed |
| 3 | user-provided evidence | Criticality level assigned |
| 4 | user-provided evidence | Requester name and role on record |
| 5 | user-provided evidence | Integration owner name, role, and accountability scope on record |
| 6 | user-provided evidence | Ticket number confirmed |
| 7 | user-provided evidence | Artifact list with IDs, versions, and dependency map on record |
| 8 | live evidence | Deployed artifact status and message monitoring baseline from target tenant |
| 9 | live evidence | Diff of artifact changes documenting changed components |
| 10 | user-provided / inference | Blast radius document confirmed (downstream partners, throughput impact) |
| 11 | user-provided evidence | Rollback procedure (previous version redeploy) confirmed feasible |
| 12 | user-provided evidence | SoD confirmed: requester ≠ integration owner / approver |
| 13 | user-provided evidence | Explicit integration-owner approval statement on record |
| 14 | live evidence | Deployment command log with timestamp |
| 15 | live evidence | Message monitoring verification — no new errors; throughput confirmed |
| 16 | all of the above | Audit record compiled |
| 17 | user-provided evidence | Report delivered and acknowledged |

## Output contract

Return after each step:

1. Step number and name
2. Evidence gathered for this step (labeled by evidence type)
3. Gate status: cleared / pending / blocked
4. Next step and what is needed to advance
5. Refusal marker if any gate condition is unmet (do not advance)

Return after step 17:

1. Complete audit record (all 17 steps with evidence and timestamps)
2. Final deployment status (success / partial / failed)
3. Post-deployment message monitoring result (error count, throughput status, partner acknowledgements)
4. Rollback status (not needed / standing by / triggered)
5. Report delivery confirmation
