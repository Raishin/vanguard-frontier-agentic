# SoD and Role Design Guide

Use this reference for Dynamics 365 Finance & Operations SoD rule design, security role layering, common failure modes, safe review workflow, verification targets, and pushback criteria.

## What people get wrong

The lazy story is:

> Assign the standard roles and SoD is automatically handled.

Wrong. Standard roles reduce but do not eliminate SoD conflicts. Combinations of standard roles can still violate SoD rules. Custom roles created by duplicating standard roles inherit all duties and may accumulate excess privileges over time. Administrator-approved overrides without compensating controls create silent fraud vectors.

Common bad assumptions:

- Standard roles are always SoD-compliant in combination.
- SoD rules are automatically enforced on existing assignments when a new rule is created.
- Approving an SoD override once covers that user permanently without review.
- Legal entity restrictions fully substitute for duty segregation.
- The system administrator role is a safe temporary workaround during go-live.

## SoD failure modes

- A user is assigned multiple roles that individually comply but collectively violate a SoD rule (e.g., Accounts Payable Clerk + Accounts Payable Manager in the same legal entity).
- SoD rules are created after role assignments, so existing conflicts are not automatically validated.
- Override approvals accumulate without a periodic review cycle, creating undetected long-term SoD violations.
- Custom roles built from duplicated standard roles are not compared against updated standard role duties after platform updates.
- Break-glass or implementation team accounts retain system administrator access after go-live.
- Privilege separation validation shows high overlap percentages that are not investigated before production deployment.

## High-risk SoD duty pairs (examples from procure-to-pay and record-to-report)

- Maintain vendor information + Process vendor payments (vendor master + payment disbursement)
- Acknowledge goods receipt + Process vendor payments (goods receipt + payment)
- Maintain customer information + Apply customer payments (customer master + cash receipts)
- Post journals + Approve journals (journal entry + approval)
- Create purchase orders + Approve purchase orders (PO creation + approval)
- Maintain fixed assets + Post fixed asset transactions (asset master + depreciation posting)

These pairs represent the highest-risk SoD scenarios per SOX and IFRS internal control guidance. Verify that SoD rules covering these pairs exist and are enforced.

## Minimum safe review workflow

1. Confirm the scope: legal entities, business process domains, and compliance drivers.
2. Run and review the **Security duty assignments report** to map all duties per role in scope.
3. Run **Validate duties and roles** under SoD rules to identify role-level violations.
4. Run **Verify compliance of user-role assignments** to identify user-level conflicts.
5. Review the **Roles violating segregation of duties** view for active violations and counts.
6. Review override history in **Segregation of duties conflicts** for documented justifications and compensating controls.
7. Review system administrator role membership and verify no production users hold it without documented justification.
8. Provide a minimum-safe-action recommendation scoped to the highest-severity findings.
9. Require live-guard escalation for any production change.

## Verification targets

- SoD rule set: duty pairs defined, severity levels, mitigation descriptions
- Role compliance: roles passing validate duties and roles check
- User compliance: user-role assignments passing verify compliance check
- Override log: documented reason for every allowed conflict
- System admin membership: documented, time-bound, break-glass only
- Security reports: evidence of periodic review by compliance or audit team
- Privilege separation validation: overlap percentage reviewed and accepted with justification

## When to push back

Push back if the user asks to:

- approve a role change that introduces a high-severity SoD conflict without compensating controls
- accept system administrator role assignment as a permanent user access solution
- rely on legal entity restrictions alone as SoD mitigations
- approve SoD overrides in bulk without individual justification
- skip SoD validation after creating or modifying rules
- make production role changes without live-guard escalation and explicit human approval
