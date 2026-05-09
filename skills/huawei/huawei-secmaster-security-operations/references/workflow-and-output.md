# Workflow and output contract

Use this reference only when performing a full security operations review or MLPS Level 3 security posture assessment.

## Security operations areas to check

- SecMaster: SIEM alert inventory (open/acknowledged/suppressed), SOAR playbook status (draft/active/dry-run required), threat intelligence feed health
- HSS: agent coverage (% of in-scope hosts), baseline check compliance, malware alert history, login audit enabled
- CFW: rule inventory, default-deny posture, east-west coverage gaps, IPS signature update status
- WAF: rule set version, CC protection threshold, IP whitelist entries with justification status, recent false positive rate
- Anti-DDoS: EIP inventory, binding coverage, protection threshold, recent attack traffic events
- VSS: scheduled scan tasks, open critical/high findings, remediation SLA tracking

## Safe workflow

1. **Frame scope** — confirm in-scope services, MLPS classification, enterprise project, and non-goals
2. **Collect evidence** — prefer live SecMaster/HSS/CFW dashboards; label all evidence types
3. **Stress-test** — HSS coverage gaps, CFW rule blast radius, SOAR playbook untested scenarios, WAF whitelist risk
4. **Recommend safest action** — HSS deployment to uncovered hosts, CFW rule dry-run in test environment, SOAR playbook dry-run

## Output contract

Return this structure:

```markdown
# Huawei Cloud Security Operations: <scope>
## Scope and evidence level
## SecMaster SIEM and SOAR status
## HSS coverage and MLPS compliance
## CFW policy assessment
## WAF and Anti-DDoS posture
## VSS vulnerability findings summary
## Recommended actions
## Open questions
```

Each section must include an evidence level label.
