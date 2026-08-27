# Workflow And Output

Identity and network security review sequence and output contract.

## Workflow

1. Establish admin role assignments: account admins, workspace admins, metastore admins, and their scope.
2. Assess admin population: is the account-admin group small? Are other admins assigned on a role-based need-to-know basis?
3. Check identity integration: is SCIM enabled? Is OAuth or federation configured? Do the limits (10K users+SPs, 5K groups) have headroom?
4. Inventory service principals: which workloads use service principals? Are they API-only or mistakenly assigned interactive roles?
5. Evaluate token design: is OAuth used where possible? For PAT, is the 730-day max lifetime and 90-day revocation understood?
6. Audit IP access lists: CIDR inventory, block-list effectiveness, PrivateLink bypass implications.
7. Check serverless network policies: FQDN allowlist, storage allowlist, destination count against the 2,500 cap.
8. Validate secret scopes: naming compliance, permission model, redaction coverage for logging scenarios.

## Evidence labels

Label every claim: `confirmed` (artifact or first-party documentation provided) > `inference` (partial artifact) > `assumption` (artifact absent) > `unknown`. Distinguish documentation evidence (how Databricks behaves) from workspace evidence (how this deployment is configured). Never present an assumption as confirmed, and never let a documentation claim stand in for workspace state.

## Output contract

- A verdict (identity-secure / identity-with-conditions / identity-risk) with explicit confidence.
- Admin role separation audit: account-admin population size, workspace-admin assignments, metastore-admin scope.
- Identity integration findings: SCIM and federation configuration, limit headroom, provider support gaps.
- Service principal inventory and usage patterns; OAuth vs PAT ratio and token lifecycle enforcement.
- IP access list evaluation findings: block-list effectiveness, PrivateLink bypass implications, CIDR headroom.
- Serverless network policy findings: destination-cap headroom, FQDN allowlist coverage, propagation-time implications.
- Secret scope and redaction findings: redaction coverage for all logging scenarios, secret-transformation risks.
