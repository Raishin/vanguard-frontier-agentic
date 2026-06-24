# Safety checklist — SAP License and BTP Consumption FinOps Review

Use before making any licensing or FinOps recommendation, especially for findings that affect True-Up exposure, digital access obligations, CPEA credit commitments, or license type reclassification.

## Non-negotiables

- Do not access, connect to, or request access to any live SAP system, SAP for Me portal, License Administration Workbench, BTP cockpit usage data, or SAP Global License Audit and Compliance team portal. This skill reviews artifacts only.
- Do not accept or request SAP contract portal credentials, customer-specific contract pricing, True-Up settlement amounts, or confidential commercial negotiation positions.
- Do not provide legal advice on SAP license contract terms. This skill provides advisory risk classification. SAP licensing counsel must be engaged for binding contract interpretation.
- Do not recommend license type downgrades without confirming which SAP functional scope the user actually exercises. A user performing Professional User functions on a Limited User license is a compliance exposure, not a cost optimization.
- Do not assert a definitive True-Up cost without the user's actual LAW measurement output and current contract pricing. Provide estimates only and label them as estimates.
- Do not conflate BTP service entitlement with BTP service consumption. An entitlement to use a BTP service does not mean the service is consuming credits or generating cost — only active provisioned instances and consumed service units generate charges.
- Do not recommend canceling or reducing a CPEA commitment without first confirming the commitment term and early termination clauses in the user's contract. CPEA commitments are typically non-cancellable within the term.

## What people get wrong

- **Treating all inactive users as safe to deprovision for license savings**: Deprovisioning named users reduces future measurement exposure, but does not retroactively reduce an already-measured True-Up obligation. Confirm the measurement date before treating deprovisioning as a retroactive cost saving.
- **Assuming CPEA credits cover all BTP services**: Not all BTP services are available under CPEA. Some services are subscription-only or available on Pay-As-You-Go only. Check service eligibility before assuming a service can be covered by CPEA credits.
- **Treating credit expiry as a fixed date**: CPEA credit validity depends on the contract start date and the credit tranche. Different credit tranches within the same CPEA contract may have different expiry dates. Confirm the expiry schedule per tranche from the user's order form.
- **Ignoring the Digital Access Adoption Program**: SAP offered a structured transition to digital access licensing under the Digital Access Adoption Program. Customers who have not activated this program may be measured under indirect access terms that are significantly more expensive. Flag this as a `high` finding if the user's system has significant third-party integrations.
- **Assuming BTP subaccount equals a cost center**: BTP subaccounts can be used to isolate cost allocation, but they are not automatically mapped to cost centers. Cost allocation requires deliberate tagging and reporting configuration in the BTP cockpit.
- **Treating LAW measurement as a one-time exercise**: SAP license audit readiness requires that LAW measurement be run and evidence be retained for each measurement period specified in the contract. Measurement that is run once and not maintained is not audit-ready.
- **Confusing BTP service plan tiers**: Many BTP services have multiple service plans (e.g., free, standard, premium). The free tier is limited in capacity and often not suitable for production. Recommending a free plan for a production use case creates a reliability risk that outweighs the cost saving.

## When to push back

- Push back when the user asks to assess True-Up exposure without providing LAW measurement output or the entitlement quantities in their contract. Exposure cannot be quantified without both sides of the measurement.
- Push back when the user asks for a definitive CPEA credit optimization plan without providing the credit expiry schedule, current burn rate, and service consumption by service. Optimization without this data is inference.
- Push back when a request requires live BTP cockpit data or SAP for Me entitlement data — state that the assessment cannot be completed without the relevant exports and ask the user to provide them.
- Push back when the user asks for SAP license contract interpretation or True-Up settlement advice. Risk classification is in scope; legal and commercial counsel is required for those activities.

## Evidence labels

- `documentation-based` — grounded in SAP BTP commercial model documentation, SAP licensing guidelines, SAP digital access documentation, or SAP Support Portal license audit resources
- `user-provided evidence` — license entitlement reports, BTP consumption exports, SAP for Me data, LAW measurement output, or commercial model summaries provided by the user
- `inference` — derived reasoning not directly confirmed by official SAP documentation or user-provided evidence; must always be labeled as such
