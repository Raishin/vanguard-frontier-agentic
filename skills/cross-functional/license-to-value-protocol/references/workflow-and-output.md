# License to Value Protocol — Workflow and Output Contract

## Detailed Workflow

### Phase 1: Baseline Definition

**Trigger events**
- A new Microsoft 365 or Dynamics 365 licence cohort has been deployed
- A periodic licence review has been scheduled
- Leadership has requested a value realisation report
- A business unit has submitted a request for additional licences

**Step 1.1 — Licence inventory**
Collect the current licence inventory:
- SKU name and total assigned count
- Active user count per SKU (users who performed at least one licensed activity
  in the past 28 days)
- Licence type (user, device, add-on, Copilot)
- Cohort or department (if licence tracking is segmented)

**Step 1.2 — Adoption Score baseline**
Retrieve the Microsoft 365 Adoption Score from the Microsoft 365 admin center:
- Overall score (and maximum possible)
- Per-category score: Communication, Meetings, Content Collaboration, Teamwork,
  Mobility, AI Adoption (if Microsoft 365 Copilot is licensed)
- Peer benchmark comparison
- Score trend (28-day, 90-day, 180-day)

If Adoption Score is unavailable (GCC High, GCC, DoD tenants, or reporting
not enabled): stop. Require the licence owner to enable reporting or identify
an alternative telemetry source before this protocol proceeds.

**Step 1.3 — Copilot usage baseline (if applicable)**
If Microsoft 365 Copilot licences are assigned, retrieve the Microsoft 365
Copilot usage report:
- Active users count (users who used Copilot on at least one day in the period)
- Daily active users trend
- Usage by application (Outlook, Teams, Word, PowerPoint, Excel, OneNote,
  Copilot Chat, Loop)
- AI Adoption Score

**Step 1.4 — Business KPI mapping**
Identify the business outcomes the licence cohort was intended to drive:
- Example: "Reduce email volume by 20% via Teams migration"
- Example: "Enable 100% of knowledge workers to use Copilot for meeting
  summaries within 90 days"
Map each business KPI to the relevant Adoption Score category or Copilot
usage signal.

**Step 1.5 — Baseline documentation**
Document the baseline: date, licence counts, Adoption Score per category,
Copilot usage if applicable, and business KPI starting values.

---

### Phase 2: Gate 1 — Baseline Defined

**Required confirmation:**
- microsoft-business-impact-value-realization-agent attests that:
  - Adoption Score data is available and current
  - Licence inventory is documented
  - At least one business KPI is mapped to a measurable signal
  - Baseline date is recorded

If baseline data is unavailable: stop. Do not set adoption targets or produce
value recommendations without a defined baseline.

---

### Phase 3: Adoption Target Setting

**Step 3.1 — Peer benchmark review**
Review the Adoption Score peer benchmark (organisations of similar size, region,
industry, and tenure with Microsoft 365). Identify categories where the
organisation is below the peer benchmark — these are the highest-value
improvement opportunities.

**Step 3.2 — AI Adoption target**
For Copilot-licensed cohorts, the AI Adoption Score benchmark is that all
licensed users achieve an average of at least three Copilot-active days per week
(score of 100). Set a realistic time-bound target based on the baseline:
- Example: "AI Adoption Score from 33 to 66 within 90 days" (one day/week to
  two days/week habit formation)

**Step 3.3 — Category targets**
For each Adoption Score category below peer benchmark, set a time-bound target.
Targets must be:
- Specific (which category)
- Measurable (target score or percentage)
- Time-bound (by what date)
- Owned by a named business or IT lead

---

### Phase 4: Value Instrumentation

**Step 4.1 — Leading indicator mapping**
Map Adoption Score and Copilot usage signals to business outcomes:

| Business KPI | Adoption Score signal | Copilot usage signal |
|---|---|---|
| Meeting efficiency | Meetings score | Copilot meeting summary adoption |
| Email reduction | Communication score | Copilot Outlook usage |
| Document collaboration | Content Collaboration score | Copilot Word/PowerPoint usage |
| Knowledge worker productivity | AI Adoption Score | Active Copilot days per user |

**Step 4.2 — Forrester research anchoring**
Reference Forrester research anchored in the Adoption Score framework:
content collaboration in the cloud vs. email attachments can save up to 100
minutes per user per week. Use such benchmarks to estimate productivity impact
from adoption improvement.

---

### Phase 5: Inactive Licence Identification

**Step 5.1 — Inactive user threshold agreement**
Confirm the inactive user threshold with the licence owner (e.g. no licensed
activity in 30 days). Document the agreed threshold and date.

**Step 5.2 — Inactive user identification**
Using Microsoft 365 usage reports, identify users who have not performed any
licensed activity within the agreed threshold period. Segment by:
- SKU (Microsoft 365 E3, E5, Copilot, Dynamics 365, etc.)
- Department or cohort
- Last activity date

**Step 5.3 — Inactive licence classification**
Classify inactive licences:
- Confirmed inactive: no activity in threshold period, no known business
  justification for dormancy
- Pending review: no activity but a business justification is possible
  (e.g. parental leave, project pause)
- Candidate for reclaim: confirmed inactive with no blocking justification

**Step 5.4 — Estimated waste calculation**
For each confirmed inactive licence: calculate estimated annualised licence cost.
This is the reclaim opportunity. Present to the licence owner as a cost-saving
recommendation.

---

### Phase 6: Gate 2 — Inactive-Licence Reclaim Before Purchase

**Required confirmation:**
- If a purchase request is active: block the purchase recommendation until
  inactive-licence reclaim has been reviewed.
- Licence owner confirms the reclaim decision for each inactive cohort:
  - Suspend: remove licence assignment immediately
  - Downgrade: move to a lower-tier SKU
  - Reassign: transfer to a new user with an active business need
  - Retain: confirmed business justification for dormancy

If this gate is not completed before a purchase recommendation is made: refuse
the purchase recommendation and require gate completion first.

---

### Phase 7: Reclaim Recommendation

Produce the reclaim recommendation:
- List of users recommended for suspension, downgrade, or reassignment
- Estimated annualised saving
- Recommended action per user cohort
- Route to the licence owner for confirmation (this protocol never executes
  licence assignment changes autonomously)

---

### Phase 8: Value Realisation Report

Produce the value realisation summary:

| Section | Content |
|---|---|
| Baseline summary | Date, licence counts, Adoption Score at baseline |
| Current state | Current Adoption Score per category vs. targets |
| Progress vs. targets | Delta per category, trend direction |
| AI Adoption (Copilot) | Current AI Adoption Score, active user rate, usage by app |
| Estimated productivity impact | Based on Adoption Score improvement and Forrester benchmarks |
| Inactive licences | Count, estimated annualised waste, reclaim decisions |
| Reclaim savings | Confirmed savings from completed reclaim actions |
| Recommended next actions | Adoption interventions, reclaim actions, purchase decisions |

---

### Phase 9: Renewal / Expansion Recommendation

**If adoption targets are met and inactive licences are remediated:**
Produce a data-supported expansion or renewal recommendation with:
- Evidence: Adoption Score at or above peer benchmark, active user rate,
  business KPI progress
- Scope: which SKUs and how many additional licences
- Gate 2 status: confirmed that inactive reclaim is complete

**If adoption targets are not met:**
Recommend an adoption intervention programme before expansion:
- Targeted training or enablement for low-adoption cohorts
- Copilot success kit deployment (where applicable)
- Re-evaluation at the next review cycle

---

### Phase 10: Human Confirmation

Route all purchase, reclaim, and adoption intervention recommendations to the
licence owner, IT procurement, and finance team. This protocol never executes
licence changes, purchase orders, or adoption programmes autonomously.

---

## Decision Tree

```
Licence review triggered
  └── Adoption Score data available?
        ├── No  → Stop; require licence owner to enable reporting
        └── Yes → Baseline documented?
                    ├── No  → Document baseline (Gate 1)
                    └── Yes → Inactive-licence threshold agreed?
                                ├── No  → Agree threshold with licence owner
                                └── Yes → Inactive licences identified?
                                            └── Purchase request active?
                                                  ├── Yes → Gate 2: reclaim review required before purchase
                                                  │          └── Reclaim decisions confirmed?
                                                  │                ├── No  → Block purchase recommendation
                                                  │                └── Yes → Purchase recommendation (with reclaim savings offset)
                                                  └── No  → Adoption targets met?
                                                                ├── Yes → Value realisation report; renewal/expansion recommendation
                                                                └── No  → Adoption intervention recommendation; next review cycle
```

---

## Output Contract

### Licence review record
| Field | Type | Description |
|---|---|---|
| licence_cohort_scope | string | SKU or user group assessed |
| skill_id | string | `license-to-value-protocol` |
| skill_version | string | `0.1.0` |
| invoked_by | string | Agent or human who invoked this protocol |
| baseline_date | ISO 8601 | Date baseline was established |
| adoption_score_overall | float | Overall Adoption Score at assessment |
| ai_adoption_score | float or null | AI Adoption Score (null if no Copilot licences) |
| active_user_rate | float | Active users / assigned licences |
| inactive_licence_count | integer | Licences confirmed inactive |
| estimated_annualised_waste | number | Estimated cost of inactive licences |
| gate_1_baseline_defined | boolean | Whether Gate 1 is confirmed |
| gate_2_reclaim_reviewed | boolean | Whether Gate 2 is confirmed |
| reclaim_recommendation | enum | suspend / downgrade / reassign / retain / pending |
| value_recommendation | enum | expand / renew / intervene / hold |
| open_questions | array | Unresolved questions |
| do_not_do_list | array | Actions excluded from this protocol's scope |
| timestamp | ISO 8601 | Protocol execution timestamp |

### Gate verdicts
| Gate | Verdict options |
|---|---|
| Baseline defined (Gate 1) | confirmed / not-confirmed |
| Inactive-licence reclaim before purchase (Gate 2) | confirmed / not-confirmed |

### Refusal record (when triggered)
| Field | Description |
|---|---|
| refusal_reason | Which refusal trigger was hit |
| escalation_target | Licence owner / IT procurement / finance |
| timestamp | ISO 8601 |

---

## Quality Assurance Notes
- This protocol never executes licence assignment changes, purchase orders, or
  adoption programme deployments. All actions require human licence owner,
  IT procurement, and finance sign-off.
- Purchase recommendations without Gate 2 (inactive-licence reclaim) sign-off
  are refused unconditionally.
- All analysis is based on aggregate and anonymised signals. Individual-level
  user activity data requires privacy-team approval before use.
- Adoption Score is unavailable in GCC High, GCC, and DoD tenants. In those
  environments, an alternative aggregate telemetry source must be agreed with
  the licence owner before this protocol proceeds.
