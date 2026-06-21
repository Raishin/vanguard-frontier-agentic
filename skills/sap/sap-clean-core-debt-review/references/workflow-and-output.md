# Workflow and output contract — SAP Clean Core Debt Review

Use this reference for all classification, remediation path selection, and output formatting.

## Clean core violation taxonomy

| Violation class | Description | Upgrade risk |
|----------------|-------------|-------------|
| `implicit-enhancement` | Implicit enhancement spots in standard SAP include programs or function groups | High — SAP may remove or rename the enhancement spot |
| `modification` | Customer modifications of SAP standard objects (via modification assistant) | High — overwritten at upgrade unless re-applied |
| `customer-include` | Use of customer include sections (e.g., `INCLUDE MF_xxxxFxx`) in SAP objects | High — deprecated in ABAP Cloud |
| `user-exit` | Classic user exits via `CALL CUSTOMER FUNCTION` | Medium-High — deprecated; SAP may stop calling them |
| `badi-classic` | Classic BAdI implementations (before ABAP Objects BAdIs) | Medium — must migrate to new BAdI concept |
| `not-released-api` | Direct consumption of SAP internal APIs (release contract NOT_RELEASED or no contract) | High — no stability guarantee; breaks at any SAP update |
| `z-object-unreleased-dep` | Custom Z/Y objects that transitively depend on NOT_RELEASED SAP APIs | Medium — inherited upgrade risk |
| `z-standalone` | Custom Z/Y objects with no SAP dependency violations | Low — clean core compliant if no modification |
| `rap-violation` | ABAP Cloud objects using forbidden language constructs (e.g., `SELECT *` on SAP tables directly) | Medium — blocks ABAP Cloud tier-2 compliance |

## Remediation path decision tree

For each flagged object:

1. **Is there a released SAP API (C1/C2) that covers the business function?**
   - Yes → refactor custom code to consume the released API. Validate on API Business Hub.
   - No → continue to step 2.

2. **Is there a BAdI (new ABAP Objects BAdI concept) with a released enhancement spot?**
   - Yes → implement a new BAdI enhancement. Remove the implicit enhancement or user exit.
   - No → continue to step 3.

3. **Can the requirement be met with key-user extensibility (custom fields, custom logic, custom forms via UI adaptation)?**
   - Yes → replace custom ABAP with key-user extensibility app configuration. No code deployment.
   - No → continue to step 4.

4. **Is the requirement a transactional extension on S/4HANA data?**
   - Yes → build a RAP business object extension or RAP-based side-by-side app on BTP ABAP Environment.
   - No → continue to step 5.

5. **Is the requirement a process extension or integration with non-SAP systems?**
   - Yes → build a side-by-side extension on BTP using SAP CAP (Node.js or Java) consuming released OData/API Management APIs.
   - No → evaluate for retirement. Assess business value vs. clean core compliance cost.

## Workflow

1. **Receive artifacts** — ATC output, custom code migration app report, object list, or user-described code.
2. **Classify each object** by violation class above.
3. **Assign upgrade risk** (High / Medium / Low / None).
4. **Apply remediation path decision tree** per object class.
5. **Prioritize** — High upgrade-risk objects first; then non-compliant with low business value; then strategic re-architecture.
6. **Return output** per the output contract below.

## Output contract

Return:

1. Object(s) reviewed and violation class
2. Evidence label (documentation-based / user-provided / context7-supplementary / inference)
3. Upgrade risk per object
4. Recommended remediation path (with API Business Hub verification step if released API is proposed)
5. Clean core compliance posture after remediation
6. Prioritized remediation sequence
7. Escalation trigger if live system inspection is required before proceeding
