# Workflow and output contract — SAP License and BTP Consumption FinOps Review

Use this reference for license and consumption category classification, FinOps finding severity, optimization path decision criteria, and output formatting.

## License and cost domain taxonomy

| Domain | Scope | Typical FinOps findings |
|--------|-------|------------------------|
| `fue-and-named-user` | Full Use Equivalent (FUE) counting, named user type classification (Professional, Limited, Self-Service), indirect named user usage | Users classified as Limited performing Professional-scope functions, more named users provisioned than active, FUE count exceeds entitlement creating True-Up exposure |
| `digital-access` | Digital access licensing for third-party integrations, document-type counts (orders, invoices, goods receipts), Digital Access Adoption Program participation | Third-party system creating SAP documents without a digital access license, incorrect document type count, Digital Access Adoption Program not activated to avoid per-document pricing |
| `btp-cpea-consumption` | CPEA credit burn rate vs forecast, credit expiry risk, service-level credit consumption patterns, CPEA vs subscription vs Pay-As-You-Go fit | Credits expiring unused, consumption consistently above forecast triggering overage charges, services priced at Pay-As-You-Go rates when CPEA credit consumption would be cheaper |
| `btp-service-cost-drivers` | Individual BTP service consumption by volume, identification of highest-cost services relative to business value, service plan selection | Expensive service plan selected when a lower plan meets the requirement, test and development services running at production scale, unused service instances accumulating charges |
| `cost-allocation` | BTP subaccount and service cost tagging, cost center allocation, chargeback and showback model, budget visibility | No subaccount-level cost attribution, business units consuming BTP resources with no cost visibility, no budget alert thresholds configured |
| `commercial-model-fit` | Match between consumption pattern and commercial model (CPEA vs subscription vs Pay-As-You-Go), volume commitment adequacy | Steady-state consumption at scale under Pay-As-You-Go (CPEA commitment would be more cost-effective), CPEA commitment too large for actual consumption pattern |
| `true-up-and-audit-readiness` | True-Up preparation, SAP audit readiness, LAW measurement output review, evidence preparation | LAW measurement not run within the required measurement period, measurement excludes relevant system types, no documented process for managing True-Up evidence |

## FinOps finding severity classification

| Risk level | Criteria |
|-----------|---------|
| `critical` | Active overage that is creating immediate audit exposure or unexpected commercial liability (e.g., measured user count already above entitlement between True-Up periods, digital access documents being created without a license) |
| `high` | Material underutilization of paid commitment (CPEA credits consistently under-consumed and at expiry risk, large license block with low active user count), or incorrectly classified license type that would be found in a True-Up |
| `medium` | Cost allocation gap (no subaccount-level cost visibility, no chargeback model), commercial model mismatch with a clear optimization opportunity that requires a contract event to act on |
| `low` | Best practice deviation in license management process (LAW measurement run but not reviewed, CPEA consumption not monitored monthly, no budget alert thresholds configured) |

## Optimization path decision criteria

For each finding, apply:

1. **Is there an active overage or audit exposure?**
   - Yes → `critical`. Quantify the exposure. Identify the remediation path (license purchase, user deprovisioning, digital access license activation). Do not defer.
   - No → continue.

2. **Is entitlement significantly larger than consumption with committed spend expiring?**
   - Yes → `high`. Identify the underutilized block. Recommend right-sizing at next contract event, reallocation to other use cases, or model conversion.
   - No → continue.

3. **Is the commercial model mismatched to the consumption pattern?**
   - Yes → `medium`. Model the cost difference between current model and optimal model. Flag as a renegotiation opportunity at the next contract event.
   - No → continue.

4. **Is there a cost visibility or allocation gap?**
   - Yes → `medium`. Recommend enabling BTP cost monitoring, subaccount cost tagging, and budget alerts. No contract action required — this is an operational improvement.
   - No → continue.

5. **Is this a process or governance deviation with no immediate cost impact?**
   - Yes → `low`. Provide guidance for alignment; do not block current operations.

## Workflow

1. **Receive artifacts** — license entitlement reports, BTP consumption exports, SAP for Me data, LAW output, commercial model summaries, or written descriptions of the licensing and BTP consumption posture.
2. **Classify each finding** by domain.
3. **Apply optimization path decision criteria** per finding.
4. **Assign risk level** (critical / high / medium / low).
5. **Quantify impact where evidence permits** — estimated overage cost, credit expiry value, or cost savings opportunity.
6. **Prioritize** — critical overage and audit exposure first; then high underutilization and misclassification; then medium commercial model and cost allocation gaps; then low process improvements.
7. **Return output** per the output contract below.

## Output contract

Return:

1. Domain and specific finding
2. Evidence label (documentation-based / user-provided evidence / inference)
3. Risk level per finding (critical / high / medium / low)
4. Quantified impact estimate (where evidence permits; label as estimate)
5. Recommended action (license reclass, True-Up evidence preparation, CPEA reallocation, model conversion, cost tag enablement)
6. Contract event required (yes / no / at renewal) — whether the optimization requires a contract action or can be done operationally
7. Escalation trigger if live SAP for Me data, LAW output, or legal contract interpretation is required
