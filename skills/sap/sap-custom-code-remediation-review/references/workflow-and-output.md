# Workflow and output contract — SAP Custom Code Remediation Review

Use this reference for ATC finding classification, deprecated API mapping, remediation path selection, and output formatting.

## ATC finding severity taxonomy (S/4HANA readiness variant)

| Severity | ATC priority | Conversion impact | Assessment action |
|----------|-------------|-----------------|------------------|
| `blocker` | Priority 1 (error) | Conversion cannot complete until resolved | Must fix or formally exempt before conversion; add to critical remediation path |
| `error` | Priority 2 (error) | Conversion may complete but system behavior is undefined or degraded | Strongly recommended to resolve before conversion; document risk if deferred |
| `warning` | Priority 3 (warning) | No conversion block; functional or performance risk post-conversion | Plan remediation in Realize phase; include in post-conversion testing scope |
| `information` | Priority 4 (info) | Advisory only; no functional impact | Document; no remediation action required |

## Custom code disposition taxonomy

| Disposition | Criteria | Preferred action |
|------------|---------|-----------------|
| `retire` | Business function now covered by SAP S/4HANA standard; custom object has no residual business value | Decommission after confirming standard process covers the requirement |
| `replace-standard` | SAP standard S/4HANA process exists but requires configuration or fit-to-standard adoption | Remove custom code; configure standard process |
| `remediate-released-api` | Deprecated API or internal FM used; released C1/C2 replacement exists | Refactor to consume released API; validate release contract on API Business Hub |
| `remediate-abap-cloud` | Language construct or object type violates ABAP Cloud tier constraints; ABAP Cloud rewrite feasible | Rewrite using ABAP Cloud permitted constructs; validate via ATC with ABAP_CLOUD variant |
| `side-by-side-btp` | In-system requirement cannot be met with released API or ABAP Cloud; side-by-side BTP extension feasible | Rebuild as BTP CAP or SAP Build extension consuming released OData or API Management APIs |
| `defer-with-exemption` | Conversion-blocking risk, but business-critical and no replacement available before go-live | Request formal ATC exemption with documented justification and owner; plan post-go-live remediation |
| `escalate` | Cannot classify without live system data or additional ATC output from user | Redirect to `sap-live-readonly-landscape-discovery` or request ATC export |

## Deprecated API replacement patterns (common examples)

These patterns appear frequently in ATC S/4HANA readiness findings. Verify all replacement candidates on SAP API Business Hub before recommending.

| Deprecated pattern | Replacement direction |
|-------------------|----------------------|
| Direct SELECT on `MARA`, `MARC`, `MAKT` and other material master transparent tables | Released OData APIs for Material Master (verify C1 contract on api.sap.com) |
| `CALL FUNCTION 'BAPI_MATERIAL_GETLIST'` (non-released BAPI) | Released SAP API for material data with C1 contract; verify on api.sap.com |
| `CALL FUNCTION 'ENQUE_*'` internal enqueue FMs | Use `cl_abap_enq_*` classes or lock objects with released contracts |
| `CALL SCREEN` / `CALL DIALOG` in ABAP Cloud objects | Replace with Fiori/UI5 or SAP Build Apps for UI-facing logic; no screen calls in ABAP Cloud |
| Classic BAdI via `CALL CUSTOMER FUNCTION` / user exit | Implement new ABAP Objects BAdI with released enhancement spot |
| Access to internal DDIC structures (no release contract) | Consume data via released CDS views (exposed via OData) or released ABAP APIs |

Note: This table is illustrative. Verify all replacements against the user's specific S/4HANA release and API Business Hub contract for the exact API proposed.

## Remediation decision tree (per ATC finding or deprecated object)

1. **Is the custom object's entire business function now covered by SAP standard S/4HANA?**
   - Yes → Retire. Confirm with business stakeholder. No remediation coding required.
   - No → Continue to step 2.

2. **Is there a released C1/C2 SAP API that covers the deprecated function call or internal table access?**
   - Yes → Refactor to consume the released API. Verify contract on api.sap.com.
   - No → Continue to step 3.

3. **Does the deprecated construct violate ABAP Cloud language constraints (forbidden statement)?**
   - Yes → Rewrite using ABAP Cloud permitted constructs. Run ATC with `SCI_ABAP_CLOUD_READINESS` variant to verify.
   - No → Continue to step 4.

4. **Is the requirement a transactional ABAP extension feasible in ABAP Cloud RAP?**
   - Yes → Plan ABAP Cloud RAP remediation. Use `sap-abap-cloud-rap-review` for detailed RAP modeling guidance.
   - No → Continue to step 5.

5. **Is the requirement addressable as a side-by-side BTP extension?**
   - Yes → Plan CAP/SAP Build extension consuming released OData or API Management APIs. No in-system ABAP retained.
   - No → Formally assess for ATC exemption with documented justification and post-go-live remediation plan.

## Workflow

1. **Receive artifacts** — ATC export (XML, XLS, or described findings), SYCM output, ABAP source code, or object list.
2. **Identify ATC check variant** — confirm `SCI_S4HANA_READINESS`, `SCI_ABAP_CLOUD_READINESS`, or describe variant if unknown.
3. **Classify each finding** by severity (blocker / error / warning / information).
4. **Apply disposition taxonomy** — assign retire / replace-standard / remediate-released-api / remediate-abap-cloud / side-by-side-btp / defer-with-exemption / escalate per finding class.
5. **Map deprecated APIs** — identify deprecated patterns and proposed released replacements; direct to api.sap.com for contract verification.
6. **Prioritize** — blocker and error findings first; then warning with business impact; then advisory.
7. **Return output** per the output contract below.

## Output contract

Return:

1. ATC check variant used and finding severity distribution
2. Evidence label per finding group (documentation-based / user-provided evidence / inference)
3. Finding classification table: object, deprecated pattern, severity, disposition
4. Deprecated API replacement mapping (with api.sap.com verification instruction for each replacement)
5. Risk level per dimension (upgrade-blocking / clean-core-non-compliant / technically-functional / compliant)
6. Prioritized remediation plan with timeline recommendation relative to conversion phases
7. Escalation trigger if live system ATC execution or additional artifact is needed before classification
