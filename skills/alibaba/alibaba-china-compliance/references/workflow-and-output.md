# Workflow and output contract

Use this reference only when performing a full China compliance review, regulatory gap analysis, or MLPS 2.0 preparation.

## China compliance areas to check

- MLPS 2.0: security level grading, annual self-assessment status, technical control coverage (audit, boundary protection, intrusion detection, backup)
- DSL Article 31: cross-border data transfer inventory, important data classification, CAC security assessment filing status
- CSL: network operator designation, real-name registration, security incident reporting procedures
- PIPL: personal data processing basis, consent management, cross-border transfer mechanism (SCC or CAC assessment), breach notification procedures
- ICP Beian: filing status for all internet-facing services in CN-* regions, domain coverage, license number validity
- Alibaba Cloud service alignment: ActionTrail (audit), Cloud Firewall/WAF (boundary), Security Center HSS (intrusion detection), OSS/RDS backup (data backup)

## Safe workflow

1. **Frame scope** — confirm target CN-* workloads, compliance driver, evidence available, and explicit non-goals
2. **Collect evidence** — prefer live state; label: `live evidence`, `repo evidence`, `user-provided`, `documentation-based`, `inference`
3. **Stress-test** — what cross-border transfers exist? what services lack ICP filing? what MLPS controls are missing?
4. **Recommend safest action** — narrow scope, staged rollout, rollback path; recommend legal counsel for regulatory submissions

## Output contract

Return this structure:

```markdown
# Alibaba Cloud China Compliance: <scope>
## Scope and evidence level
## Findings
## Risks
## Recommended actions
## Open questions
```

Each section must include an evidence level label.
