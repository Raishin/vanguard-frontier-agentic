# ERP / CRM Cutover Protocol — Workflow and Output Contract

## Detailed Workflow

### Phase 1: Mock Cutover Review

**Trigger**
The Dynamics 365 implementation project has reached the Prepare phase. At least
one mock cutover has been completed in a sandbox environment.

**Step 1.1 — Mock cutover results intake**
Review the mock cutover results:
- Task completion status (each task in the cutover plan: completed / failed /
  skipped)
- Timing: actual duration vs. planned duration per task and total
- Errors and issues: critical (blocks go-live), major (requires resolution),
  minor (can be resolved post go-live)
- Integration cutover steps: all external systems tested in the mock

**Step 1.2 — Issue triage**
For each critical or major issue identified in the mock:
- Document the root cause
- Assign an owner and resolution target date
- Determine whether a second mock cutover is required

If critical unresolved issues remain and a second mock is not planned: stop.
Require the project lead to schedule a second mock before the real cutover
is authorised.

**Step 1.3 — Timing assessment**
Confirm that the mock cutover completed within the planned cutover window (with
buffer). If the mock exceeded the window: the real cutover plan must be revised
before proceeding.

---

### Phase 2: Data Reconciliation

**Step 2.1 — Reconciliation methodology**
Confirm the data reconciliation methodology is documented:
- What data sets are being validated (customers, vendors, items, open
  transactions, balances, etc.)
- What tolerance thresholds are accepted for each data set
- Who is the data owner for each data set

**Step 2.2 — Reconciliation execution (review)**
Review the reconciliation results from the mock cutover:
- Records migrated vs. records in source system (count)
- Financial balances: trial balance match (if applicable)
- Open transactions: all open orders, cases, or projects reconciled
- Exceptions: document and classify (acceptable variance / data quality issue /
  migration script error)

**Step 2.3 — Exception resolution**
For each exception beyond the tolerance threshold:
- Require the data owner to either resolve the exception or formally accept
  the variance with a documented rationale.

---

### Phase 3: Gate 1 — Reconciliation Sign-off

**Required sign-off:**
- Named data owner confirms that reconciliation thresholds are met for all
  critical data sets, or formally accepts documented variances with rationale.
- d365-data-migration-cutover-agent produces the reconciliation report.
- Project lead countersigns.

If sign-off cannot be obtained: stop. Do not proceed to rollback validation
without reconciliation sign-off.

---

### Phase 4: Rollback Plan Validation

**Step 4.1 — Rollback plan document review**
Confirm the rollback plan is documented and includes:
- Trigger criteria (what conditions invoke rollback)
- Who has authority to invoke rollback (named individual, not just a role)
- Step-by-step rollback sequence
- Estimated rollback duration
- Data integrity approach during rollback (snapshot, backup, point-in-time
  restore)

**Step 4.2 — Rollback rehearsal confirmation**
Confirm that the rollback has been rehearsed in the sandbox. d365-data-migration-
cutover-agent attests:
- Rollback was executed in the sandbox
- Rollback completed within the estimated duration
- System was returned to a known good state after rollback

If the rollback has not been rehearsed: stop. Require rehearsal before the real
cutover is authorised.

---

### Phase 5: Gate 2 — Rollback Tested

**Required confirmation:**
- d365-data-migration-cutover-agent confirms rollback rehearsal completed
  within the cutover window.
- Project lead confirms rollback authority is assigned to a named individual
  who is available during the cutover window.

---

### Phase 6: Separation of Duties Validation

**Step 6.1 — Production security role review**
d365-security-sod-governance-agent reviews all security role assignments planned
for production go-live:
- Identify any SoD conflicts (e.g. a user who can both create and approve
  purchase orders, or both post and reconcile financial transactions)
- Flag conflicts to the security and business owners
- Require remediation or compensating controls before go-live

**Step 6.2 — SoD sign-off**
Security owner and business process owner confirm that SoD conflicts are
remediated or compensating controls are documented and accepted.

---

### Phase 7: SbD Go-live Gate

**Step 7.1 — Success by Design go-live readiness review**
d365-success-by-design-governance-agent runs the SbD go-live readiness review
against the full go-live checklist:

| Checklist area | Status |
|---|---|
| Solution scope aligned with stakeholders | signed-off / not-signed-off |
| UAT complete and signed off | signed-off / not-signed-off |
| System integration testing complete | signed-off / not-signed-off |
| Performance testing complete | signed-off / not-signed-off |
| Data migration readiness (mock + reconciliation) | signed-off / not-signed-off |
| External dependencies aligned | signed-off / not-signed-off |
| Change management complete | signed-off / not-signed-off |
| Production environment ready | signed-off / not-signed-off |
| Cutover plan complete | signed-off / not-signed-off |
| Rollback plan tested | signed-off / not-signed-off |
| Operational support plan ready | signed-off / not-signed-off |
| Security and SoD validated | signed-off / not-signed-off |

**Step 7.2 — SbD verdict**
- **Go**: all checklist items signed off.
- **Conditional-go**: minor open items with owner-committed resolution dates
  and business acceptance.
- **No-go**: one or more blocking issues remain.

---

### Phase 8: Gate 3 — SbD Go-live Gate Sign-off

**Required sign-off:**
- d365-success-by-design-governance-agent issues the SbD verdict.
- Project steering committee formally confirms the go/no-go decision.
- Microsoft FastTrack (where applicable) confirms go-live readiness.

This protocol never issues the final go/no-go decision. That decision belongs
to the human project steering committee.

---

### Phase 9: Cutover Execution Readiness

**Step 9.1 — People readiness**
Confirm that all required personnel are available during the cutover window:
- Data migration owner
- Integration owner
- Rollback authority
- Production environment owner
- Business process lead
- Support team lead

**Step 9.2 — Communication plan**
Confirm the communication plan is active: stakeholders are notified, cutover
start/stop messages are drafted, escalation contacts are confirmed.

**Step 9.3 — Final production environment check**
Confirm production environment version matches development/test. Confirm no
in-flight platform updates are scheduled during the cutover window.

---

### Phase 10: Post-Cutover Validation

After go-live:
- Confirm data accuracy in production (spot-check reconciliation)
- Confirm integration health (all integration endpoints responding)
- Confirm user access is functional (security roles are assigned and working)
- Invoke hypercare plan

---

## Decision Tree

```
Cutover requested
  └── Mock cutover completed?
        ├── No  → Stop; require mock cutover
        └── Yes → Critical issues resolved?
                    ├── No  → Require second mock; stop
                    └── Yes → Mock completed within cutover window?
                                ├── No  → Revise cutover plan; stop
                                └── Yes → Reconciliation thresholds met?
                                            ├── No  → Resolve exceptions; stop
                                            └── Yes → Gate 1 sign-off obtained?
                                                          ├── No  → Stop
                                                          └── Yes → Rollback rehearsed?
                                                                        ├── No  → Require rehearsal; stop
                                                                        └── Yes → Gate 2 sign-off obtained?
                                                                                      ├── No  → Stop
                                                                                      └── Yes → SoD conflicts resolved?
                                                                                                    ├── No  → Require remediation; stop
                                                                                                    └── Yes → SbD checklist complete?
                                                                                                                  ├── No  → Resolve blocking items; stop
                                                                                                                  └── Yes → Gate 3 steering committee sign-off
                                                                                                                                └── Human go/no-go decision
```

---

## Output Contract

### Cutover readiness record
| Field | Type | Description |
|---|---|---|
| project_id | string | Dynamics 365 implementation project identifier |
| skill_id | string | `erp-crm-cutover-protocol` |
| skill_version | string | `0.1.0` |
| invoked_by | string | Agent or human who invoked this protocol |
| mock_cutover_status | enum | pass / fail / not-run |
| mock_timing_within_window | boolean | Whether mock completed within planned window |
| gate_1_reconciliation | enum | signed-off / not-signed-off |
| gate_2_rollback_tested | enum | confirmed / not-confirmed |
| sod_validation_status | enum | pass / conflicts-open / remediated |
| sbd_verdict | enum | go / conditional-go / no-go |
| gate_3_steering_committee | enum | signed-off / not-signed-off |
| open_issues | array | Unresolved issues with owners and target dates |
| do_not_do_list | array | Actions excluded from this protocol's scope |
| rollback_status | enum | tested / not-tested / authority-assigned |
| timestamp | ISO 8601 | Protocol execution timestamp |

### Gate verdicts
| Gate | Verdict options |
|---|---|
| Reconciliation sign-off (Gate 1) | signed-off / not-signed-off |
| Rollback tested (Gate 2) | confirmed / not-confirmed |
| SbD go-live gate (Gate 3) | signed-off / not-signed-off |

### Refusal record (when triggered)
| Field | Description |
|---|---|
| refusal_reason | Which refusal trigger was hit |
| escalation_target | Data owner / security owner / project lead / FastTrack |
| timestamp | ISO 8601 |

---

## Quality Assurance Notes
- This protocol never executes migration scripts, production deployments, or
  environment configurations. All execution steps require the infrastructure
  or project owner.
- The go/no-go decision is always made by the human project steering committee.
  d365-success-by-design-governance-agent issues a recommendation, not a
  decision.
- All gate sign-offs must include the named decision-maker, the date, and a
  statement of accepted residual risk.
- Rollback plans that have not been rehearsed in a sandbox are a hard block
  on the cutover recommendation with no exceptions.
