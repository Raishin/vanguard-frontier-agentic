# Copilot Data Readiness Protocol — Workflow and Output Contract

## Detailed Workflow

### Phase 1: Oversharing Baseline

**Trigger**
An organisation is planning to enable Microsoft 365 Copilot for a new user
population, or a periodic readiness review has been initiated.

**Step 1.1 — Purview DSPM assessment**
Run (or review the most recent) Microsoft Purview Data Security Posture
Management (DSPM) Data Risk Assessment. Identify:
- Sites with sensitive data exposed to broad audiences (EEEU, company-wide links)
- Risky sharing links (Anyone links, organisation-wide links on sensitive sites)
- Content frequently accessed by large audiences that contains sensitive data
- Oversharing posture score

If the DSPM assessment has not been run: stop. Require the data owner to
initiate the assessment before this protocol proceeds.

**Step 1.2 — SAM Content Management Assessment**
Run (or review) the SharePoint Advanced Management (SAM) Content Management
Assessment. Identify:
- Sites with oversized audiences
- EEEU usage at site or folder level
- Broken permission inheritance (files or folders with unique permissions
  broader than the parent site)
- Inappropriate sharing (external sharing on internally-classified sites)
- Inactive or ownerless sites

**Step 1.3 — High-risk site inventory**
Produce a ranked inventory of high-risk sites. Classify as:
- Critical: sensitive content + EEEU or Anyone links
- High: sensitive content + broad internal sharing
- Medium: oversharing without confirmed sensitive content
- Low: inactive/ownerless without confirmed sensitive content

---

### Phase 2: Interim Protections

**Step 2.1 — SAM Restricted Content Discovery**
For all critical and high-risk sites not yet remediated, apply SAM Restricted
Content Discovery (RCD) to exclude them from Microsoft 365 Copilot discovery.
Validate via Purview Auditing that Copilot is no longer surfacing content from
these sites.

**Step 2.2 — DLP for Copilot location**
Configure (or confirm) Microsoft Purview DLP policies scoped to the Copilot
location. These policies should exclude sensitive content from Copilot
grounding. Validate policy coverage against the high-risk site inventory.

**Step 2.3 — EEEU tenant-level check**
Confirm whether EEEU is enabled at the tenant level in SharePoint. If yes:
escalate to the SharePoint admin as a high-priority remediation item. Recommend
disabling EEEU before Copilot enablement.

---

### Phase 3: Sensitivity Label Review

**Step 3.1 — Label taxonomy check**
Confirm that a sensitivity label taxonomy is configured and published in
Microsoft Purview. Verify that labels are published to all users in the Copilot
enablement scope.

**Step 3.2 — Label coverage on high-risk sites**
For critical and high-risk sites: assess the percentage of files that carry a
sensitivity label. If label coverage is below the agreed threshold:
- Draft labelling recommendations for the data owner.
- Flag as a conditional readiness blocker (not a hard block if interim
  protections are in place, but must be resolved before interim protections
  are lifted).

**Step 3.3 — Legacy IRM content**
Identify any content using legacy Information Rights Management (IRM) protection.
Flag for migration to Microsoft Purview sensitivity labels, as IRM-protected
documents are not used in Copilot grounding.

---

### Phase 4: Permissions Remediation

**Step 4.1 — High-risk site remediation tasks**
For each high-risk site identified in Phase 1, produce a remediation task list:
- Remove EEEU and company-wide sharing links
- Rescope sharing links to approved users or groups
- Correct broken permission inheritance at library and folder level
- Confirm site ownership (assign or confirm a site owner)

**Step 4.2 — SAM site access reviews**
For sites where the data owner needs to manage access, initiate SAM site access
reviews. Site owners review down to the file level and take the recommended
actions.

**Step 4.3 — Interim protection lifecycle**
Once a site is fully remediated, remove the SAM RCD exclusion and Copilot DLP
exclusion for that site. Validate via Purview Auditing that Copilot grounding
now works as expected for that site.

---

### Phase 5: Gate 1 — Oversharing Remediation Sign-off

**Required sign-off:**
- Data owner confirms that all critical-risk sites are either remediated or have
  accepted interim protections with a time-bound remediation commitment.
- m365-copilot-readiness-governance-agent attests that the oversharing baseline
  is established and the inventory is documented.

If sign-off cannot be obtained: stop. Do not proceed to identity layer or DLP
guardrails without this gate. This is a hard prerequisite for any Copilot
enablement recommendation.

---

### Phase 6: Identity Layer Check

**Step 6.1 — Conditional Access for Copilot users**
Invoke m365-identity-zero-trust-agent to confirm:
- Conditional Access policies enforce MFA for all Copilot-enabled users
- Device compliance is required for access to Microsoft 365 Copilot workloads
- No broad exceptions or exclusions exist for the Copilot user population

**Step 6.2 — Identity layer gaps**
If gaps are found: flag to the identity owner. Hold the Copilot enablement
recommendation until m365-identity-zero-trust-agent confirms remediation.

---

### Phase 7: DLP Guardrails

**Step 7.1 — DLP coverage validation**
Confirm that Purview DLP policies:
- Cover the Microsoft 365 Copilot location (Copilot interactions)
- Are actively monitoring sensitive content interactions
- Align with the sensitivity label taxonomy

**Step 7.2 — Insider Risk Management**
Confirm that Microsoft Purview Insider Risk Management (IRM) adaptive protection
is configured to detect patterns of inappropriate or noncompliant Copilot usage
and to automatically apply more restrictive policies to risky users.

---

### Phase 8: Gate 2 — Permissions Baseline

Produce the permissions baseline document. This is the minimum viable sign-off
before any Copilot enablement recommendation:

| Metric | Status |
|---|---|
| DSPM assessment run | yes / no |
| SAM assessment run | yes / no |
| EEEU tenant-level status | enabled / disabled |
| Critical-risk sites (open) | count |
| High-risk sites (open) | count |
| Sites with RCD interim protection | count |
| Sensitivity label coverage (high-risk sites) | % |
| DLP policy covers Copilot location | yes / no |
| IRM adaptive protection configured | yes / no |

---

### Phase 9: Copilot Readiness Recommendation

copilot-governance-maestro-agent reviews the permissions baseline and produces
a per-user-group recommendation:
- **Go**: oversharing baseline complete, interim protections in place or
  remediated, identity layer confirmed, DLP guardrails active.
- **Conditional-go**: enablement permitted for the user group with time-bound
  remediation commitments for residual gaps.
- **No-go**: critical prerequisites not met; Copilot must not be enabled.

---

### Phase 10: Human Confirmation

Route the readiness recommendation to the data owner, security team, and Copilot
programme owner for final sign-off. This protocol never enables Copilot
autonomously.

---

## Decision Tree

```
Copilot enablement requested for user group
  └── DSPM assessment run?
        ├── No  → Stop; require data owner to run assessment
        └── Yes → High-risk sites identified?
                    └── Interim protections applied (RCD + DLP)?
                          ├── No  → Apply RCD + DLP before continuing
                          └── Yes → EEEU at tenant level?
                                      ├── Yes → Escalate to SharePoint admin; flag as high priority
                                      └── No  → Sensitivity label coverage meets threshold?
                                                    ├── No  → Flag as conditional blocker; continue with interim controls
                                                    └── Yes → Permissions remediation complete for critical sites?
                                                                  ├── No  → Continue with RCD protection; set remediation timeline
                                                                  └── Yes → Gate 1 sign-off obtained?
                                                                                ├── No  → Stop
                                                                                └── Yes → Identity layer gaps?
                                                                                              ├── Yes → Hold; route to identity owner
                                                                                              └── No  → DLP guardrails confirmed?
                                                                                                            ├── No  → Hold; configure DLP
                                                                                                            └── Yes → Gate 2 permissions baseline
                                                                                                                          └── copilot-governance-maestro-agent readiness recommendation
                                                                                                                                └── Human confirmation required
```

---

## Output Contract

### Readiness assessment record
| Field | Type | Description |
|---|---|---|
| tenant_scope | string | User group or tenant segment assessed |
| skill_id | string | `copilot-data-readiness-protocol` |
| skill_version | string | `0.1.0` |
| invoked_by | string | Agent or human who invoked this protocol |
| assessment_date | ISO 8601 | Date of assessment |
| dspm_assessment_status | enum | run / not-run |
| sam_assessment_status | enum | run / not-run |
| eeeu_tenant_level | enum | enabled / disabled / unknown |
| critical_risk_sites_open | integer | Sites not yet remediated |
| high_risk_sites_open | integer | Sites not yet remediated |
| sites_with_rcd | integer | Sites with interim RCD protection |
| label_coverage_rate | float | % of files on high-risk sites with sensitivity labels |
| dlp_copilot_location | boolean | Whether DLP covers Copilot location |
| irm_adaptive_protection | boolean | Whether IRM adaptive protection is active |
| gate_1_status | enum | signed-off / not-signed-off |
| gate_2_status | enum | baseline-complete / incomplete |
| readiness_recommendation | enum | go / conditional-go / no-go |
| open_questions | array | Unresolved questions |
| do_not_do_list | array | Actions excluded from this protocol's scope |
| timestamp | ISO 8601 | Protocol execution timestamp |

### Gate verdicts
| Gate | Verdict options |
|---|---|
| Oversharing remediation sign-off (Gate 1) | signed-off / not-signed-off |
| Permissions baseline (Gate 2) | complete / incomplete |

### Refusal record (when triggered)
| Field | Description |
|---|---|
| refusal_reason | Which refusal trigger was hit |
| escalation_target | Data owner / security team / SharePoint admin |
| timestamp | ISO 8601 |

---

## Quality Assurance Notes
- The oversharing baseline gate is unconditional. There is no path to a Copilot
  enablement recommendation that bypasses Gate 1.
- DSPM and SAM assessments must be run — estimated or anecdotal oversharing
  assessments are not accepted as substitutes.
- copilot-governance-maestro-agent's readiness recommendation is the final
  cross-pillar artefact and cannot be bypassed by any individual agent or owner.
- This protocol does not execute SharePoint sharing setting changes, DLP policy
  changes, or Entra Conditional Access changes. All configuration changes
  require the relevant service owner.
