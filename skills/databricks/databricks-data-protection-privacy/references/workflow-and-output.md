# Workflow And Output

Data protection and privacy review sequence and output contract.

## Workflow

1. Establish the sensitive-data inventory: which tables and columns contain PII, PCI, healthcare, financial data?
2. Check mask and filter coverage: is every sensitive column masked? Are row filters in place for data-level access control?
3. Review UDF definitions: are they deterministic (enabling optimisation)? Do they use string operations (cheaper) or regex?
4. Assess ABAC policies: which scopes (catalog/schema/table) carry policies? Will new objects automatically inherit?
5. Check data classification: is it enabled? Is backfill enabled (intentional decision)? Are frameworks (PII, PCI, GDPR, etc.) identified?
6. Verify deletion mechanics: for sensitive data, are DELETE/MERGE followed by REORG (if deletion vectors enabled) and VACUUM? Is the VACUUM window shorter than GDPR deadlines?
7. Evaluate Delta Sharing: how many recipients? Are they IPv4 only? Is cross-region egress cost quantified?
8. Confirm encryption and residency: is the organisation on Enterprise tier (CMK eligible)? Is inter-node traffic exposure understood? Is data residency in-Geo?

## Evidence labels

Label every claim: `confirmed` (artifact or first-party documentation provided) > `inference` (partial artifact) > `assumption` (artifact absent) > `unknown`. Distinguish documentation evidence (how Databricks behaves) from workspace evidence (how this deployment is configured). Never present an assumption as confirmed, and never let a documentation claim stand in for workspace state.

## Output contract

- A verdict (privacy-compliant / privacy-with-conditions / privacy-risk) with explicit confidence.
- Sensitive-data inventory and mask/filter coverage audit; UDF analysis (deterministic, string vs regex cost).
- ABAC policy scope inventory and object-creation auto-evaluation findings.
- Data classification status: backfill enabled (and justification), framework coverage, PUBLIC PREVIEW impact.
- Deletion mechanics audit: DELETE/MERGE/VACUUM/REORG coordination, retention windows, GDPR deadline alignment.
- Delta Sharing recipient and egress-cost findings; OpenSharing IPv4 CIDR cap check.
- Encryption eligibility (Enterprise tier?) and Geo residency compliance; inter-node traffic exposure findings.
