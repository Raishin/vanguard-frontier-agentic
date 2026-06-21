# Context7 framework docs — SAP Clean Core Debt Review

**Role**: supplementary. Official SAP documentation (SAP Help Portal, ABAP Cloud docs, API Business Hub) is the primary source for all clean core compliance guidance. Context7-sourced CAP documentation supplements guidance for side-by-side BTP extensibility patterns only.

**Library used**: SAP Cloud Application Programming Model (CAP)
Context7 library ID: `/websites/cap_cloud_sap`
Lookup target: SAP CAP extensibility, intrinsic extensibility, cds.ExtensionDeveloper, SaaS customer adaptation
Skill: `sap-clean-core-debt-review`
Classification: supplementary

---

## CAP intrinsic extensibility (supplementary)

Source: cap.cloud.sap (Context7 `/websites/cap_cloud_sap`)

CAP offers intrinsic extensibility allowing SaaS customers, verticalization partners, or teams to add/override annotations, translations, initial data, extension fields, entities, relationships, and custom logic. Extensions can be bundled as reuse extension packages and feature-toggled per tenant, using the same techniques as in regular projects (CDS Aspects and Event Handlers).

**Relevance to clean core debt review**: When a custom ABAP side-by-side requirement maps to a BTP extension, CAP intrinsic extensibility is the relevant BTP-side pattern for SaaS multi-tenant scenarios. This supplements the SAP Help Portal guidance on side-by-side developer extensibility.

## CAP extensibility service (supplementary)

Source: cap.cloud.sap (Context7 `/websites/cap_cloud_sap`)

The `cds.ExtensibilityService` (internal CAP service) requires `cds.ExtensionDeveloper` role. Extension upload and activation are separate operations:

- `PUT /-/cds/extensibility/Extensions/<id>` — uploads extension as draft (status 1) or activates (status 2)
- Requires authentication as internal technical user or `cds.ExtensionDeveloper` technical role

**Relevance**: When recommending CAP as a side-by-side extensibility target, the user must be aware of the `cds.ExtensionDeveloper` role requirement and the draft/activation lifecycle — especially for SaaS tenant extensions.

## Enabling CAP extensibility (supplementary)

Source: cap.cloud.sap (Context7 `/websites/cap_cloud_sap`)

```bash
cds add extensibility
npm add @sap/cds-mtxs
```

The `@sap/cds-mtxs` package enables multi-tenancy and extensibility features. Run `cds add extensibility` to automate adding the package and setting `cds.requires.extensibility: true` in `package.json`.

**Relevance**: Guidance for users implementing CAP-based side-by-side extensions as a clean core remediation path.

---

## Scope boundaries for Context7 usage

Context7 CAP documentation applies **only** to the side-by-side BTP CAP remediation path (step 5 in the remediation decision tree). It does not apply to:

- In-system ABAP Cloud or RAP guidance (use SAP Help Portal for ABAP Cloud and ABAP RAP)
- Key-user extensibility (use SAP Help Portal)
- Release contract validation (use SAP API Business Hub)
- Upgrade risk assessment (use SAP Help Portal)

Always label Context7-sourced guidance as `context7-supplementary` in responses.
