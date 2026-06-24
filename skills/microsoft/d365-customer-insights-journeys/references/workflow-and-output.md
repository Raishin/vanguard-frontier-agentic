# Workflow and output contract

Use this reference only when performing the full Customer Insights — Data or Customer Insights — Journeys review, or formatting the final answer.

## Review domains

Check these areas before giving a verdict:

- Data source ingestion: source connectivity, table selection, schema mapping, refresh schedule
- Data unification: source field mapping completeness, deduplication rule quality, match rule coverage, merge policy correctness
- Unified profile quality: profile completeness, identity resolution evidence, known duplicates or splits
- Segment design: rule logic, profile source validation, membership count reasonableness, refresh cadence, downstream usage
- Measures and KPIs: definition correctness, data currency, calculation logic, usage in segments or journeys
- Consent model: compliance profile configuration, purpose and topic hierarchy, contact point consent coverage, double opt-in status, legacy consent migration completeness
- Journey design: trigger or segment entry configuration, branch logic, channel steps, personalization token validity, suppression and frequency cap configuration
- Compliance posture: GDPR, CAN-SPAM, CASL, or other applicable regulation alignment; unsubscribe link presence; preference center availability
- Stakeholder sign-off: named compliance owner and marketing operations lead approvals, dated
- Post-launch monitoring: engagement metrics baseline, consent opt-out rate tracking, journey error rate monitoring

## Safe workflow

1. **Frame scope**
   - Customer Insights workloads in scope (Data, Journeys, or both):
   - Data sources being unified:
   - Journey types in scope (segment-based, trigger-based, or both):
   - Regulatory jurisdictions in scope (GDPR, CAN-SPAM, CASL, other):
   - Explicit non-goals:

2. **Collect evidence**
   - Prefer documented artifacts: unification run logs, segment inspection exports, consent audit reports, journey test run results, compliance profile configuration, stakeholder sign-off records.
   - Otherwise inspect sanitized user-provided summaries or official Dynamics 365 Customer Insights documentation.
   - Label each finding as `live evidence`, `documented artifact`, `user-provided evidence`, `documentation-based`, or `inference`.

3. **Stress-test risk**
   - Have all data sources been ingested and schema-mapped before unification runs?
   - Has the deduplication and match configuration been validated with sample profile results?
   - Do segment rules reference correct profile attributes and are refresh schedules aligned with journey entry cadence?
   - Has the consent model been configured with at least one compliance profile, purpose, and topic before journey publish?
   - Is double opt-in configured on the relevant compliance profiles if required by applicable regulation?
   - Has legacy consent been migrated from outbound marketing or external systems?
   - Have all journey branches been tested including suppression, exit, and error paths?
   - Has the compliance owner signed off on the consent model and unsubscribe path before production journey publish?
   - What evidence is missing that would change the verdict?

4. **Recommend the smallest safe action**
   - Prefer additional unification runs over proceeding to production journeys with incomplete profiles, staged consent migration over bulk outreach, and journey test runs over direct production publish.
   - If the safest action is to stop and complete consent model configuration or validate segment membership, say that plainly.
   - Production journey publish and bulk outreach require live-guard escalation. Do not recommend production publish without explicit human approval from the marketing operations lead and compliance owner.

## Output contract

Return this structure:

```markdown
# D365 Customer Insights Review: <scope>
## Executive verdict
- Status: READY / READY WITH CONDITIONS / NOT READY / NEEDS EVIDENCE
- Biggest risk:
- Evidence level:
## Scope and assumptions
- Confirmed:
- Unknown:
- Out of scope:
## Findings
| Severity | Finding | Evidence | Why it matters | Minimum safe action |
|---|---|---|---|---|
## Recommended actions
1. <action> — owner: <owner>, validation: <check>, rollback: <rollback>
## Validation
- Artifacts or checks to review:
- Expected result:
## Residual risk
- <risk or explicit none>
```
