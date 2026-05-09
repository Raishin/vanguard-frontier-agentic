# Workflow and output contract

Use this reference only when performing a full ACR governance audit, supply chain security review, or production-readiness assessment.

## Audit domains

Check these areas before giving a recommendation:

- ACR edition: Enterprise vs Personal, SLA coverage, rate limit exposure
- Namespace visibility: public vs private, IAM policy least privilege
- Vulnerability scanning: scanner configuration, severity thresholds, blocking policy
- Tag immutability: mutable vs immutable tag enforcement per repository
- Image retention: lifecycle policy, untagged image cleanup, storage cost management
- Cross-region replication: replication rules, target regions, replication lag monitoring
- Supply chain security: image signing, provenance attestation, SBOM availability
- China/international separation: CN-* vs international instance isolation

## Safe workflow

1. **Frame the audit scope**
   - ACR instance type (Enterprise/Personal) and region(s):
   - Production namespaces and repositories in scope:
   - Regulatory or compliance requirements:
   - Explicit out-of-scope items:
2. **Collect evidence**
   - Prefer live ACR console or API evidence if available.
   - Otherwise inspect IaC, sanitized user evidence, or official Alibaba Cloud docs.
   - Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Stress-test the posture**
   - Are any production namespaces publicly accessible?
   - Are HIGH or CRITICAL CVEs present in production images?
   - Are mutable tags used in production deployments?
   - Is there a single region of failure for any production image?
   - What evidence is missing?
4. **Recommend the smallest safe hardening sequence**
   - Prioritize public namespace remediation above all other findings.
   - If the safest action is to gather more evidence, say that plainly.

## Output contract

Return this structure:
```markdown
# Alibaba Cloud ACR Governance Assessment: <scope>
## Executive summary
- Security verdict (pass / warn / fail):
- Evidence level:
- Critical findings:
## ACR edition assessment
- Edition in use:
- Production workload suitability:
- SLA coverage:
## Namespace IAM and access control
- Public namespaces detected:
- IAM policy posture:
- Least privilege gaps:
## Vulnerability scanning
- Scanner enabled:
- Severity threshold configuration:
- Unresolved HIGH/CRITICAL CVEs:
## Tag immutability and retention
- Mutable tags in production:
- Retention policy configured:
## Cross-region replication
- Replication rules configured:
- Single-region failure exposure:
## Supply chain security
- Image signing enforced:
- Provenance attestation:
## Recommended hardening actions
| Priority | Finding | Action | Effort |
|---|---|---|---|
## Open questions
1. <question> — owner: <owner>, impact: <impact if unresolved>
```
