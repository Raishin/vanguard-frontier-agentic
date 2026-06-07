# OCI WAF Security Review Operations Reference

## What people get wrong

- No Cloud Guard findings means secure.
- Administrator access is acceptable because the account is trusted.
- Security Zones can be bolted on after risky design without migration planning.
- Public SSH, broad ingress, public buckets, and wildcard policies are normal until incident response.
- Encryption keys prove governance without rotation, access, audit, and backup checks.

## Officially grounded service shape

- OCI security posture evidence spans IAM policies, compartments, network controls, encryption, logging, Cloud Guard, Security Zones, and vulnerability scanning.
- Cloud Guard problem listing is compartment-scoped by default and can include subtree traversal only when allowed; filters include risk level, detector type, resource type, target, lifecycle, and Security Zone category.
- Vulnerability Scanning results are regional and can surface as Cloud Guard problems in the global reporting region.
- Security Zones can prevent policy-violating operations for associated compartments, while Cloud Guard can identify policy violations in existing resources.

## Non-negotiable design rules

- Require least-privilege IAM, network exposure, logging, encryption, vulnerability, and detector evidence before issuing a positive security verdict.
- Require explicit approval before IAM, network, key, Cloud Guard, Security Zone, or scanner mutations.
- Label Cloud Guard and scanner evidence as sampled configured-environment evidence unless the exact scope and timestamp are proven.
- Treat public exposure, wildcard IAM, disabled logging, unmanaged keys, inactive detectors, and missing scanner recipes as blockers until justified.
- Never commit policies, identifiers, customer topology, vulnerability details, or logs that reveal sensitive data.

## Minimal safe implementation flow

- Classify workload and security boundary.
- Ground security controls in official OCI security, Cloud Guard, Security Zone, and scanner docs.
- Use OCI API evidence through the user’s configured read-only OCI MCP only for sanitized problem-list or resource-list shape and sampled observations.
- Rank findings by exploitability, blast radius, compliance impact, and reversibility.
- Return verdict, blockers, safe next actions, and open questions.

## High-risk assumptions to kill

- Documentation proves service behavior; it does not prove the user's deployed posture.
- Sampled API evidence proves only the sampled command shape or observation.
- Read-only discovery is not approval for mutation.
- Missing evidence is a blocker, not a detail to smooth over.

## Safe command/code verification targets

- Prefer schema, manifest, link, and asset-integrity validation for repository edits.
- Prefer read-only list/get/help operations for cloud evidence.
- Redact or omit identifiers and sensitive values from notes and reports.

## Safe verification targets

- Official OCI documentation URL is attached to each service-behavior claim.
- Sampled API evidence is labeled with scope and limitation.
- Approval gates are explicit for every proposed mutation.
- Evidence gaps are listed as open questions.

## When to push back

- The request asks for a clean verdict from incomplete evidence.
- The user wants to mutate IAM, network, detector, or key controls without approval.
- The prompt would disclose sensitive topology, vulnerabilities, logs, or identifiers.
