# Environment-to-Production Release Protocol — Detailed Workflow and Output Contract

## Overview

This document provides the step-by-step workflow, decision tree, and output contract for the `environment-to-production-release-protocol` skill. It is the reference for `power-platform-alm-pipelines-agent`, `copilot-studio-agent-governance-alm-agent`, and human release managers who need to understand the gate structure, the deployment record format, and the rollback initiation procedure.

---

## Detailed Workflow

### Phase 1 — Release Candidate Preparation

**Step 1.1 — Solution packaging validation**
- Confirm solution is exported as a managed solution (not unmanaged)
- Confirm version number is incremented from the last deployed version in the target environment
- Confirm no unmanaged layers targeting test or production environments
- Output: `solution_package_record` with `solution_name`, `version`, `managed: true|false`, `version_increment_confirmed: true|false`

**Step 1.2 — Solution checker execution**
- Run Power Platform solution checker against the managed solution
- Severity thresholds:
  - Critical violations: deployment is blocked until remediated
  - High violations: flagged; release manager must acknowledge before proceeding
  - Medium / Low: recorded; no block
- Output: `solution_checker_result` with `critical_count`, `high_count`, `medium_count`, `low_count`, `check_passed: true|false`

**Step 1.3 — Dependency verification**
- Confirm all solution dependencies (other solutions, components) are present and at the required version in the target environment
- Flag any missing or mismatched dependencies
- Output: `dependency_check` with `all_satisfied: true|false`, `missing_dependencies[]`

---

### Phase 2 — Pipeline Stage Validation

**Step 2.1 — Confirm pipeline configuration**
- Confirm the pipeline is configured in the Power Platform admin center or a custom host environment
- Confirm the pipeline stages exist and the target stage is the next valid stage in sequence
- Power Platform pipelines enforce stage order: you cannot deploy to production before test has succeeded with the same solution version
- Output: `pipeline_stage_check` with `pipeline_name`, `stage_name`, `stage_order_valid: true|false`, `prior_stage_status`

**Step 2.2 — Connection references and environment variables**
- Confirm all connection references in the solution are configured and have valid connections in the target environment
- Confirm all required environment variable values are set for the target environment
- Output: `config_readiness` with `connection_refs_status: ready|incomplete`, `env_vars_status: ready|incomplete`, `missing_items[]`

**Step 2.3 — Rollback artifact confirmation**
- Confirm that a prior successful managed solution deployment exists in the deployment history for the target environment (recoverable from pipelines host)
- Confirm rollback steps are documented and a named rollback owner is identified
- Output: `rollback_readiness` with `prior_artifact_available: true|false`, `rollback_steps_documented: true|false`, `rollback_owner`

---

### Phase 3 — Gate Enforcement

**Gate 1 — Managed-Solution-Only Gate**
```
IF solution_package_record.managed = false
OR unmanaged_layers_in_target = true:
  → STOP
  → Refuse deployment
  → Escalate to Power Platform admin
  → Do NOT proceed until solution is re-exported as managed
```

**Gate 2 — Pre-Flight Gate**
```
IF solution_checker_result.critical_count > 0
OR dependency_check.all_satisfied = false:
  → STOP
  → Return to development for remediation
  → Do NOT initiate pipeline deployment
```

**Gate 3 — Stage Order Gate**
```
IF pipeline_stage_check.stage_order_valid = false:
  → STOP
  → Enforce stage progression
  → Do NOT skip stages
```

**Gate 4 — Configuration Readiness Gate**
```
IF config_readiness.connection_refs_status = incomplete
OR config_readiness.env_vars_status = incomplete:
  → PAUSE
  → Escalate to environment owner to complete configuration
  → Do NOT deploy until all connection references and env vars are ready
```

**Gate 5 — Rollback Tested Gate**
```
IF rollback_readiness.prior_artifact_available = false
OR rollback_readiness.rollback_steps_documented = false:
  → FLAG as risk item
  → Require release manager written acknowledgment before proceeding
  → Record acknowledgment reference in deployment record
```

**Gate 6 — Human Approval Gate**
```
ALWAYS required before production deployment:
  → Request approval from environment owner or release manager
  → Record: approver_id, approval_timestamp, approval_reference
  → Do NOT initiate production deployment without recorded approval
```

---

### Phase 4 — Deployment Execution

**Step 4.1 — Initiate pipeline deployment**
- Trigger pipeline deployment for the confirmed stage and solution version
- Monitor status: in-progress → succeeded | failed | canceled
- Record: `deployment_run_id`, `start_time`, `end_time`, `status`

**Step 4.2 — Deployment failure handling**
```
IF deployment status = failed:
  → Capture failure reason from deployment log
  → Assess: is rollback required? (data migration, custom connector changes may require rollback)
  → If rollback required: initiate rollback procedure with rollback owner
  → Escalate to release manager with failure summary and rollback status
  → Do NOT retry deployment without root cause identified
```

---

### Phase 5 — Post-Deployment Confirmation

**Step 5.1 — Post-deployment validation**
- Confirm solution import status in target environment
- Confirm connection references are live (not in error state)
- Confirm environment variables are applied
- Request functional owner smoke-test confirmation for critical user paths
- Output: `post_deployment_validation` with `import_status`, `connection_refs_live: true|false`, `smoke_test_status: passed|pending|failed`

**Step 5.2 — Hypercare initiation**
- Record hypercare period start date and end date
- Confirm hypercare support owner is identified
- Schedule post-deployment review meeting
- Output: `hypercare_record` with `start_date`, `end_date`, `support_owner`, `review_scheduled_date`

---

## Decision Tree (Condensed)

```
Solution ready for promotion
  └─ Gate 1: Managed solution? → No → STOP
       └─ Gate 2: Pre-flight clear? → No → STOP (return to dev)
            └─ Gate 3: Stage order valid? → No → STOP
                 └─ Gate 4: Config ready? → No → PAUSE
                      └─ Gate 5: Rollback documented? → No → FLAG + require acknowledgment
                           └─ Gate 6: Human approval? → Not received → HOLD
                                └─ Initiate pipeline deployment
                                     ├─ Failed → Assess rollback → Escalate
                                     └─ Succeeded → Post-deployment validation → Hypercare
```

---

## Output Contract

### Deployment Record

| Field | Type | Required | Description |
|---|---|---|---|
| `deployment_record_id` | string (UUID) | Yes | Unique deployment identifier |
| `skill_id` | string | Yes | Must be `environment-to-production-release-protocol` |
| `skill_version` | string | Yes | Semantic version |
| `solution_name` | string | Yes | Power Platform solution name |
| `solution_version` | string | Yes | Managed solution version |
| `pipeline_name` | string | Yes | Pipeline name from pipelines configuration |
| `stage_name` | string | Yes | Target deployment stage |
| `target_environment` | string | Yes | Environment display name or ID |
| `gates_passed` | string[] | Yes | Which gates were cleared |
| `gates_blocked` | string[] | Yes | Which gates fired a stop or hold |
| `approval_reference` | string | Yes (for production) | Approver ID + timestamp |
| `rollback_owner` | string | Yes | Named rollback responsible party |
| `deployment_status` | enum | Yes | `succeeded | failed | canceled | in_progress` |
| `post_deployment_validation` | object | Yes | Import status, connection ref status, smoke test |
| `hypercare_record` | object | Yes | Start/end dates, support owner |
| `do_not_do_list` | string[] | Yes | Mandatory refusal items |
| `open_questions` | string[] | Yes | Unresolved items for human judgment |
| `timestamp` | string (ISO) | Yes | Deployment record creation datetime |

### Do-Not-Do List (always attached)

- Do not deploy an unmanaged solution to a test or production environment.
- Do not skip pipeline stage order; test must succeed before production is attempted.
- Do not initiate a production deployment without a recorded human approval.
- Do not proceed without a documented rollback plan and named rollback owner.
- Do not request service principal credentials, tenant IDs, or customer data to validate a deployment.
- Do not retry a failed deployment without identifying the root cause first.

---

## Rollback Initiation Procedure

1. Release manager declares rollback required.
2. Identify the prior managed solution version from the deployment history in the Power Platform admin center deployment hub (Import Solutions from Pipelines Host feature).
3. Import the prior managed solution version to the target environment using the standard import path.
4. Confirm all connection references and environment variables are correctly set for the restored version.
5. Run post-deployment validation for the restored version.
6. Document rollback completion in the deployment record.
7. Schedule a root-cause analysis for the failed deployment.

---

## Audit Log Fields

`deployment_record_id`, `skill_id`, `skill_version`, `invoked_by`, `solution_name`, `solution_version`, `stage_name`, `target_environment`, `gates_passed`, `gates_blocked`, `approval_reference`, `deployment_status`, `timestamp`
