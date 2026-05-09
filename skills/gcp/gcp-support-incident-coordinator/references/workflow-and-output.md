# Workflow and output contract

Use this reference only when performing the full support incident coordination, case creation checklist, TAM escalation, or PIR planning.

## Coordination domains

Check these areas before giving a case coordination verdict:
- Incident scope: affected services, regions, user populations, and business impact
- GCP platform status: status.google.com and Managed Incident (MI) declaration check
- Case severity: Severity 1 (production down), 2 (major impact), 3 (partial impact), 4 (general guidance)
- Evidence scrubbing: PII removal, credential removal, log sanitization before submission
- TAM escalation: P0 requires phone contact; P1 may require case escalation flag
- SLA tracking: Premium Sev1 = 15 min response, Sev2 = 4 hours, Sev3 = 8 hours, Sev4 = 24 hours
- Communication cadence: internal stakeholder updates every 30 min for P0, 1 hour for P1
- PIR coordination: within 5 business days for P0/P1, joint with GCP account team if platform involved

## Safe workflow

1. **Frame scope**
   - Service(s) affected and user impact:
   - First symptom and timestamp:
   - GCP support tier (Premium/Enhanced/Standard):
   - Business impact (revenue, users, SLA):
2. **Check platform status**
   - status.google.com — label result as live evidence.
   - Support portal — check for active Managed Incident declaration.
3. **Create support case**
   - Set severity matching business impact.
   - Attach only scrubbed evidence (no PII, no credentials).
   - Record case number and creation timestamp.
4. **Track SLA and escalate**
   - Log expected first response deadline.
   - If SLA breached: escalate to TAM via phone for P0, case portal escalation flag for P1+.
   - Document SLA breach timestamp and case number for credit claim.

## Output contract

Return this structure:
```markdown
# GCP Support Incident Coordinator: <incident title>
## Executive verdict
- Incident scope: <affected services and user impact>
- GCP platform status: <clean / MI declared / degradation>
- Support case severity: Severity <1/2/3/4>
- Evidence level:
## Case creation checklist
- [ ] Business impact documented
- [ ] Evidence scrubbed (no PII, no credentials)
- [ ] Case filed with correct severity
- [ ] Case number recorded: <case number>
- [ ] Case creation timestamp: <timestamp>
## TAM escalation
- TAM contact required: <yes/no> — trigger: <P0 default / SLA breach>
- Contact method: <phone / case portal escalation flag>
- SLA first response due: <timestamp>
## Stakeholder communication
- Next internal update: <time>
- Template: "Incident update [<time>]: <service> experiencing <impact>. GCP case #<number> filed. Next update in <interval>."
## Post-incident review
- PIR due: <5 business days from resolution for P0/P1>
- Joint PIR with GCP: <yes/no — required if platform involved>
## Residual risk
- <risk or explicit none>
```
