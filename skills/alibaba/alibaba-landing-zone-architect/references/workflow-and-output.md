# Workflow and output contract

Use this reference only when performing a full landing zone design, governance review, or implementation roadmap.

## Design domains

Check these areas before giving a recommendation:

- Resource Management org tree structure and OU hierarchy
- Control Policy baseline and deny-list coverage
- Cloud SSO configuration and IdP federation
- ActionTrail cross-account centralization to SLS
- RAM baseline and permission boundary application
- Billing account structure and tag-based cost allocation

## Safe workflow

1. **Frame requirements**
   - Number of accounts and intended OU groupings:
   - Identity provider and SSO requirements:
   - Compliance and audit trail requirements:
   - Billing and cost allocation structure:
   - Explicit non-goals:
2. **Collect evidence**
   - Prefer live Resource Management console or API evidence if available.
   - Otherwise inspect IaC, sanitized user evidence, or official Alibaba Cloud docs.
   - Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Stress-test the design**
   - What Control Policy gaps allow unrestricted actions in member accounts?
   - What ActionTrail gaps leave audit blind spots?
   - What automation roles lack permission boundaries?
   - What evidence is missing?
4. **Recommend the smallest safe next step**
   - Prefer bootstrapping master account controls before enrolling member accounts.
   - If the safest action is to gather more evidence, say that plainly.

## Output contract

Return this structure:
```markdown
# Alibaba Cloud Landing Zone Design: <scope>
## Executive summary
- Recommendation:
- Evidence level:
- Key risks:
## Org tree design
- Master account:
- OU structure:
- Member account assignments:
## Control Policy baseline
| Policy | Applied at | Effect | Rationale |
|---|---|---|---|
## Cloud SSO configuration
- IdP federation method:
- Permission set baseline:
## ActionTrail centralization
- Trail scope:
- SLS project target:
- Retention policy:
## RAM baseline
- Permission boundary template:
- Applied to:
## Billing structure
- Cost allocation tags:
- Billing account assignments:
## Implementation roadmap
1. <step> — owner: <owner>, prerequisite: <prerequisite>
## Open questions
1. <question> — owner: <owner>, impact: <impact if unresolved>
```
