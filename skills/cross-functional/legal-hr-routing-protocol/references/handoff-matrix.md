# Legal-HR Overlap Handoff Matrix

Each row is a recurring matter that crosses the Legal/HR boundary. The matter
moves as a `legal-hr-case-capsule`. The **primary agent** owns synthesis; the
**secondary agents** run parallel review; the **escalation gate** names the
human owner the matter must reach.

| # | Scenario | Primary agent | Secondary agents | Escalation gate |
|---|---|---|---|---|
| 1 | Termination with potential retaliation risk | hr-termination-readiness-agent | legal-employment-law-risk-agent, hr-employee-relations-agent | Employment counsel sign-off before any action |
| 2 | Harassment or discrimination complaint | hr-workplace-investigations-agent | legal-employment-law-risk-agent, legal-ethics-investigations-agent | Employment counsel; investigation lead |
| 3 | Medical leave or accommodation | hr-leave-accommodation-agent | legal-employment-law-risk-agent, legal-privacy-data-protection-agent | Employment counsel; privacy owner |
| 4 | Workforce reduction or restructuring | hr-workforce-planning-rif-agent | legal-employment-law-risk-agent, legal-public-disclosure-agent (if material) | Employment counsel; disclosure owner if material |
| 5 | Pay equity or promotion dispute | hr-compensation-equity-agent | legal-employment-law-risk-agent, hr-analytics-people-data-agent | Employment counsel; total-rewards owner |
| 6 | Vendor handling employee data | legal-vendor-procurement-risk-agent | legal-privacy-data-protection-agent, hr-hris-process-controls-agent | Privacy owner; procurement owner |
| 7 | Internal whistleblower complaint | legal-ethics-investigations-agent | hr-workplace-investigations-agent, legal-litigation-discovery-hold-agent | Ethics owner; litigation-hold owner |
| 8 | Executive misconduct | legal-ethics-investigations-agent | hr-employee-relations-agent, legal-policy-governance-agent | Board / audit committee escalation |
| 9 | HR AI tool adoption | hr-recruiting-selection-agent | hr-analytics-people-data-agent, legal-privacy-data-protection-agent, legal-regulatory-compliance-agent | Privacy owner; compliance owner |
| 10 | Employee data breach | legal-privacy-data-protection-agent | hr-hris-process-controls-agent, hr-employee-relations-agent, legal-litigation-discovery-hold-agent | Privacy owner; incident commander |
| 11 | Contractor vs. employee classification | legal-employment-law-risk-agent | hr-benefits-payroll-agent, legal-vendor-procurement-risk-agent | Employment counsel; payroll owner |
| 12 | Union, labor, or collective-action matter | hr-employee-relations-agent | legal-employment-law-risk-agent, legal-litigation-discovery-hold-agent | Labor counsel |
| 13 | Employee complaint involving monitoring | hr-employee-relations-agent | hr-analytics-people-data-agent, legal-privacy-data-protection-agent | Privacy owner; ER owner |
| 14 | Compensation + payroll + protected activity | hr-compensation-equity-agent | hr-benefits-payroll-agent, legal-employment-law-risk-agent | Employment counsel |
| 15 | Publicly sensitive executive / regulator / media issue | hr-maestro-agent + legal-maestro-agent | legal-public-disclosure-agent, legal-policy-governance-agent | Board / audit; disclosure owner |

## Mandatory do-not-do items (all rows)

Every capsule produced from this matrix includes, at minimum:

- Do not approve, deny, terminate, discipline, settle, or file.
- Do not notify a regulator or make a public disclosure.
- Do not send an employee communication.
- Do not mutate an HR or legal system of record.
- Do not request or retain personal data beyond the minimum necessary.
- Do not backdate, retroactively create, or alter documentation.

## Scenario-specific do-not-do additions

- Rows 1, 11, 14: do not characterize the matter as purely performance or
  policy when protected activity is in the timeline.
- Rows 2, 7, 8: do not contact the complainant or witnesses outside the
  investigation plan; do not compromise investigation confidentiality.
- Rows 3, 6, 9, 10, 13: do not collect or forward medical, special-category, or
  monitoring data beyond the minimum the matter requires.
- Row 8: do not resolve at the management layer; board / audit escalation is
  mandatory.
- Rows 4, 15: do not communicate externally before the disclosure owner and
  counsel have reviewed materiality.
