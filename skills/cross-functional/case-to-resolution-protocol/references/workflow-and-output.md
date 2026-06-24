# Case to Resolution Protocol — Workflow and Output Contract

## Detailed Workflow

### Phase 1: Intake and Authentication

**Trigger events**
- Inbound email, chat, voice, social media, or web form submission
- Agent or bot escalation from self-service channel
- Manual case creation by a representative

**Step 1.1 — Case logging**
Log the case record in Dynamics 365 Customer Service. Capture:
- Channel of origin
- Customer/employee identity reference (do not log unverified PII as confirmed)
- Case description and initial priority signal
- Applicable entitlement or SLA tier

**Step 1.2 — Record creation rules**
Apply configured record creation and update rules to normalise fields. If the
case arrives via email, validate that the record creation rule mapped it
correctly to the right case type and queue.

**Step 1.3 — Authentication gate**
Verify the requester's identity before exposing account data. Options:
- Entitlement lookup (case linked to known customer account)
- Dynamics 365 authentication / Nuance Gatekeeper (voice)
- Representative manual verification (flag as manually verified)

If authentication cannot be confirmed, hold the case in a pending-authentication
queue and notify the customer. Do not expose account data on an unverified case.

---

### Phase 2: Routing

**Step 2.1 — Unified routing evaluation**
Apply Dynamics 365 unified routing rules:
- Skills matching (representative skill set vs. case requirements)
- Queue capacity (available capacity in target queue)
- SLA tier (premium entitlements route to priority queues)
- Case type (product, billing, technical, returns, etc.)

**Step 2.2 — Routing outcome**
- If rules produce a confident match: auto-assign to queue or representative.
- If rules produce an ambiguous match: flag for supervisor review; do not
  silently assign to a default queue.
- If case crosses multiple service domains: assign a primary representative and
  note the secondary domain for potential swarm.

**Step 2.3 — SLA clock start**
Once routed, start the SLA clock. Send the SLA-compliant acknowledgment to
the customer (configured in Dynamics 365 Customer Service SLA settings).

---

### Phase 3: Investigation and Resolution

**Step 3.1 — Knowledge base search**
Representative searches the Dynamics 365 knowledge base and any connected
SharePoint knowledge sources. Copilot suggestions (AI-suggested articles and
similar cases) are surfaced in Customer Service Workspace.

**Step 3.2 — Collaboration**
If the case requires cross-team input, invoke Microsoft Teams swarming from
within Dynamics 365 Customer Service. Log all collaboration as activities on
the case record.

**Step 3.3 — SLA attainment check (Gate 1)**
At 80% of the first-response or resolution SLA timer:
- If on track: continue.
- If at risk: escalate to supervisor via Dynamics 365 alert. Supervisor reviews
  and either reassigns or confirms continued ownership with an agreed plan.
  Do not allow silent SLA breach.

**Step 3.4 — Resolution**
Apply the resolution. Complete the resolution dialog in Dynamics 365 Customer
Service:
- Resolution type (how it was resolved)
- Resolution summary (what was done)
- Billable time (if entitlement tracks billable minutes)
Case status transitions to Resolved only when the Resolve Case action is
explicitly taken.

---

### Phase 4: Knowledge Capture and Closure

**Step 4.1 — Knowledge capture gate (Gate 2)**
Before closing, evaluate whether the resolution should produce a new or updated
knowledge article:
- Is this case type covered by an existing article? If yes, is the article
  accurate and complete given this resolution?
- If no: draft a new article.
- If yes but outdated: flag the article for update.

This gate is mandatory. If the representative cannot determine article coverage,
escalate to the knowledge base owner before closure.

**Step 4.2 — Knowledge article draft**
Draft the article in the Dynamics 365 knowledge authoring tool. Submit for
review through the configured publishing workflow. This protocol never
auto-publishes a knowledge article.

**Step 4.3 — Case closure**
Set case status to Resolved. Confirm open activities are handled (cancelled or
completed). Trigger the post-resolution CSAT survey via Dynamics 365 Customer
Voice.

---

### Phase 5: CSAT Review

**Step 5.1 — CSAT collection**
Dynamics 365 Customer Voice sends the configured post-resolution survey to the
customer. Respect opt-out preferences and survey fatigue controls.

**Step 5.2 — CSAT routing**
Route CSAT results to the responsible team or supervisor dashboard. Aggregate
results in Power BI dashboards connected to Dynamics 365 Customer Service
Insights.

**Step 5.3 — Low-CSAT escalation**
If CSAT score falls below the agreed threshold:
- Flag the case for quality review.
- Require a root-cause analysis within the agreed SLA.
- Feed findings into the continuous-improvement process.

---

## Decision Tree

```
Case received
  └── Authentication confirmed?
        ├── No  → Hold in pending-authentication queue; notify customer
        └── Yes → Log case; apply routing rules
                    └── Routing rules produce confident match?
                          ├── No  → Supervisor review; manual assignment
                          └── Yes → Auto-assign; start SLA clock
                                      └── SLA at risk (80% elapsed)?
                                            ├── Yes → Escalate to supervisor
                                            └── No  → Investigate and resolve
                                                          └── Resolution complete?
                                                                └── Knowledge article needed?
                                                                      ├── Yes → Draft article; submit for review
                                                                      └── No  → Close case; trigger CSAT
                                                                                    └── CSAT below threshold?
                                                                                          ├── Yes → Flag for quality review; root-cause required
                                                                                          └── No  → Close cycle
```

---

## Output Contract

Every execution of this protocol produces the following artefacts:

### Case capsule (per case)
| Field | Type | Description |
|---|---|---|
| case_id | string | Dynamics 365 case identifier |
| skill_id | string | `case-to-resolution-protocol` |
| skill_version | string | `0.1.0` |
| invoked_by | string | Agent or human who invoked this protocol |
| channel | enum | email / chat / voice / social / web / manual |
| routing_basis | string | Rule set applied for routing |
| sla_status | enum | on-track / at-risk / breached |
| knowledge_article_action | enum | none / drafted / updated |
| csat_score | number or null | Post-resolution CSAT (null if not yet received) |
| open_questions | array | Unresolved questions at handoff |
| do_not_do_list | array | Actions explicitly excluded from this protocol's scope |
| escalation_fired | boolean | Whether a human escalation was triggered |
| timestamp | ISO 8601 | Protocol execution timestamp |

### Gate verdicts (per gate)
| Gate | Verdict options |
|---|---|
| SLA attainment check | pass / escalated / breached |
| Knowledge capture | article-drafted / article-updated / no-action-required |

### Refusal record (when triggered)
| Field | Description |
|---|---|
| refusal_reason | Which refusal trigger was hit |
| escalation_target | Where the case was sent |
| timestamp | ISO 8601 |

---

## Quality Assurance Notes
- This protocol does not modify Dynamics 365 routing rules, SLA definitions,
  or queue configurations. Changes to these settings require the service owner.
- Knowledge articles drafted by this protocol are in draft state until a human
  knowledge manager approves and publishes them.
- CSAT data is handled in aggregate only. Individual representative CSAT data
  requires HR and privacy-team approval before use in performance review.
