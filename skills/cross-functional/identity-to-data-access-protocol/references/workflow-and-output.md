# Identity to Data Access Protocol — Workflow and Output Contract

## Detailed Workflow

### Phase 1: Identity Verification

**Trigger events**
- Joiner event: new employee or guest requires access provisioning
- Mover event: role change requires access modification
- Leaver event: departure requires access de-provisioning
- Ad-hoc access request: user requests a resource, application, or role
- PIM activation request: user requests a privileged role activation
- Periodic access review: scheduled review of a group, application, or role

**Step 1.1 — Identity claim intake**
Collect the identity claim:
- UPN and identity type (employee, guest, service principal, managed identity)
- Lifecycle event type
- Requested resource, application, or role with business justification
- Requestor's manager or sponsor (for guest access)

**Step 1.2 — Authentication strength check**
Verify MFA registration and device compliance status:
- If MFA is not registered: block and direct the user to MFA registration.
- If device is non-compliant: block and direct the user to device remediation.
- If authentication strength meets Conditional Access requirements: proceed.

---

### Phase 2: Conditional Access Evaluation

**Step 2.1 — Policy mapping**
Map the requested access to applicable Conditional Access policies in the
Microsoft Entra tenant. Identify:
- Which policies apply to this user, resource, and sign-in context
- Whether any policy requires additional controls (MFA, compliant device,
  location restriction, session lifetime)
- Whether any policy exception or exclusion list applies

**Step 2.2 — Gap identification**
If the requested access is not covered by any Conditional Access policy,
flag as a governance gap. Do not recommend access to an unprotected resource
without flagging the gap to the identity owner.

---

### Phase 3: Least-Privilege Check

**Step 3.1 — Existing assignment enumeration**
Enumerate the identity's current:
- Direct role assignments in Microsoft Entra ID
- Group memberships (and inherited role assignments)
- Entitlement management access package assignments
- Application role assignments

**Step 3.2 — Over-privilege identification**
Compare existing assignments against the stated business purpose. Flag:
- Roles or permissions not required for the stated purpose
- Standing assignments to privileged roles that should be JIT
- Group memberships that grant broader access than needed
- Inactive assignments (not used in the defined inactive threshold)

**Step 3.3 — Recommended scope**
Produce a recommended scope: the minimum permissions required for the stated
business purpose, for the minimum duration required.

---

### Phase 4: Gate 1 — Access Review Sign-off

**Step 4.1 — Review currency check**
For the target resource or role, check whether an access review is configured
and whether the identity has been attested in the current review cycle.

- If review is current and identity is attested: proceed.
- If review is overdue: block the access recommendation; flag to the review
  owner. Do not recommend access to a resource with an overdue review.
- If no review is configured for a resource that should have one: flag as a
  governance gap to the identity owner.

---

### Phase 5: Data Governance Layer

**Step 5.1 — Data sensitivity assessment**
Invoke m365-copilot-readiness-governance-agent to assess:
- Whether the target resource contains sensitive content (per sensitivity labels
  or Microsoft Purview DSPM signals)
- Whether the resource is over-shared and the requested identity would receive
  broader effective access than intended
- Whether sensitivity labels are applied to the content in scope

**Step 5.2 — Data governance gate**
If the target resource contains unlabelled or over-shared sensitive content:
- Hold the access recommendation.
- Route the finding to the data owner and m365-copilot-readiness-governance-agent.
- Do not recommend access until the data owner confirms the content is
  appropriately governed or accepts the residual risk.

---

### Phase 6: Entitlement Management

**Step 6.1 — Access package check**
If access is via a Microsoft Entra entitlement management access package:
- Confirm the package policy is satisfied: approval workflow, expiry, and SoD.
- Confirm the requestor is eligible for the package.
- Confirm the package has not expired or been revoked.

**Step 6.2 — Separation of duties**
Check whether granting the requested access would create a SoD conflict with
existing assignments. If yes: block the recommendation and escalate to the
identity owner and security team.

---

### Phase 7: PIM Gate (Privileged Roles Only)

**Step 7.1 — Privileged role detection**
If the requested access includes a privileged role (Global Administrator,
Privileged Role Administrator, Security Administrator, Exchange Administrator,
or any role with equivalent blast radius):
- Confirm that PIM is configured for just-in-time activation.
- Confirm that the requestor has an active, documented business justification.
- Confirm the activation window is bounded (not permanent).

**Step 7.2 — PIM activation assessment**
If standing permanent assignment is requested:
- Block the recommendation unconditionally for Global Administrator equivalent.
- Flag all other standing privileged assignments for PIM migration.

---

### Phase 8: Gate 2 — Least-Privilege Validation

Produce the least-privilege attestation:
- Recommended access: resource/role, scope, duration
- Evidence: Conditional Access policy satisfied, MFA confirmed, device
  compliant, no SoD conflict, data governance confirmed
- Residual risk: any acknowledged gaps accepted by the identity owner
- Open questions: anything not yet confirmed

---

### Phase 9: Recommendation and Human Confirmation

Produce the access recommendation (approve / deny / reduce-scope) with full
evidence basis. Route to the identity owner or security team for human
confirmation. This protocol never approves or denies access autonomously.

---

## Decision Tree

```
Identity claim received
  └── MFA registered + device compliant?
        ├── No  → Block; direct to remediation
        └── Yes → Map to Conditional Access policies
                    └── Policy gap identified?
                          ├── Yes → Flag governance gap to identity owner; hold
                          └── No  → Check existing assignments for over-privilege
                                      └── SoD conflict?
                                            ├── Yes → Block; escalate to security owner
                                            └── No  → Access review current?
                                                          ├── No  → Block; notify review owner
                                                          └── Yes → Data governance check
                                                                        └── Unlabelled/over-shared sensitive content?
                                                                              ├── Yes → Hold; route to data owner
                                                                              └── No  → Privileged role?
                                                                                            ├── Yes → PIM JIT confirmed?
                                                                                            │          ├── No  → Block; require PIM migration
                                                                                            │          └── Yes → Least-privilege attestation
                                                                                            └── No  → Least-privilege attestation
                                                                                                          └── Human confirmation required
```

---

## Output Contract

### Access recommendation record
| Field | Type | Description |
|---|---|---|
| identity_id | string (anonymised) | Internal identity reference (not UPN in transit) |
| skill_id | string | `identity-to-data-access-protocol` |
| skill_version | string | `0.1.0` |
| invoked_by | string | Agent or human who invoked this protocol |
| lifecycle_event | enum | joiner / mover / leaver / ad-hoc / pim-activation / access-review |
| access_scope | string | Resource or role requested |
| recommendation | enum | approve / deny / reduce-scope / hold-pending-data-governance |
| conditional_access_status | enum | satisfied / gap-identified / exception-applied |
| least_privilege_attestation | boolean | Whether least-privilege validation passed |
| access_review_status | enum | current / overdue / not-configured |
| pim_required | boolean | Whether PIM JIT is required for this access |
| sod_conflict | boolean | Whether a SoD conflict was detected |
| data_governance_status | enum | confirmed / hold / risk-accepted-by-owner |
| evidence_quality | enum | high / medium / low |
| open_questions | array | Unresolved questions at handoff |
| do_not_do_list | array | Actions explicitly excluded from this protocol's scope |
| timestamp | ISO 8601 | Protocol execution timestamp |

### Gate verdicts
| Gate | Verdict options |
|---|---|
| Access review sign-off | current / overdue / not-configured |
| Least-privilege validation | pass / fail-over-privilege / fail-sod |

### Refusal record (when triggered)
| Field | Description |
|---|---|
| refusal_reason | Which refusal trigger was hit |
| escalation_target | Identity owner / security team / data owner |
| timestamp | ISO 8601 |

---

## Quality Assurance Notes
- This protocol never approves, denies, or executes access changes. All
  recommendations require human identity or security owner sign-off.
- PIM standing-assignment blocks for Global Administrator equivalent are
  unconditional and cannot be overridden by this protocol.
- Data governance sub-reports from m365-copilot-readiness-governance-agent
  are always preserved as attachments to the access recommendation record.
