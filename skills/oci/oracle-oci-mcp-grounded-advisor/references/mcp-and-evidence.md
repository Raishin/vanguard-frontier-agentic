# MCP and Evidence Discipline

## Evidence classes

- **Official OCI documentation evidence**: proves documented service behavior, supported concepts, command references, and stated caveats. It does not prove the user's tenancy, compartments, IAM policies, limits, deployed resources, or production readiness.
- **OCI API evidence through the user’s configured read-only OCI MCP**: can prove sampled command shape, API availability, and sanitized configured-environment observations. It does not prove broad regional availability or full account posture.
- **Inference**: mark as inference when connecting evidence to a risk, remediation, or operational recommendation.
- **Unknown**: say unknown when the available evidence does not prove the claim.

## Boundaries

- Use read-only discovery first.
- Do not expose local connector labels, profile names, internal tool names, account-specific identifiers, or environment-specific paths in committed docs or final reports.
- Do not ask for credentials, tokens, private keys, wallets, tenancy identifiers, compartment identifiers, resource identifiers, support identifiers, customer data, or config contents.
- Do not treat documentation as evidence of the user's deployed state.
- Do not treat sampled API evidence as complete coverage.

## Reporting labels

Use these labels in final output:

- `official_docs`: documented OCI behavior with URL.
- `sampled_api_evidence`: sanitized read-only observation or command/API shape.
- `inference`: risk or recommendation derived from evidence.
- `unknown`: missing proof.
