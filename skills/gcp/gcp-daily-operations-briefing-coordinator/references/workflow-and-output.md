# Workflow and output contract

Use this reference only when performing the full daily operations briefing, action item assignment, or next 24-hour risk summary.

## Briefing domains

Review these areas in order at every daily standup:
- Cost delta: today vs. prior day, vs. 7-day rolling average; flag >15% anomalies
- Quota utilization: any service quota >80% requires immediate increase request; >90% is urgent
- Deployment health: Cloud Deploy pipeline status, Cloud Build trigger results, failed builds blocking promotion
- SCC findings: new HIGH/CRITICAL findings since last briefing, age of unowned findings (>24h = SLA breach)
- SLO burn rate: fast burn (>14.4×) = P1, slow burn (>1×) = warning; error budget percentage remaining
- Action items: carryover from prior briefing, new items from today's review, all with named owners and deadlines
- 24-hour risk watch: known risks, scheduled changes, quota headroom, error budget runway

## Safe workflow

1. **Frame scope**
   - Date and briefing participants:
   - Services and environments in scope:
   - Prior day baseline cost:
2. **Collect evidence**
   - Prefer live GCP evidence from sanitized billing, quota, SCC, and monitoring output.
   - Label each finding as `live evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Triage each domain**
   - Cost: flag anomalies, assign owner if >15% delta.
   - Quota: escalate increase request if >80%.
   - Deployments: block new approvals if failed pipelines are unreviewed.
   - SCC: assign owner to all HIGH/CRITICAL findings; escalate unowned >24h findings.
   - SLO: classify burn rate, recommend error budget freeze if <10% remaining.
4. **Assign action items**
   - Every anomaly exits the briefing with a named owner and deadline.
   - No items deferred without explicit documented justification and a next-review date.

## Output contract

Return this structure:
```markdown
# GCP Daily Operations Briefing: <date>
## Summary status
- Cost anomaly: <clean / ANOMALY — <delta>% — owner: <name>>
- Quota risk: <clean / WARNING — <service> at <pct>%>
- Deployment health: <green / BLOCKED — <pipeline> failed>
- SCC findings: <clean / <N> unowned HIGH/CRITICAL>
- SLO burn rate: <green / WARNING / P1 — fast burn>
- Evidence level:
## Cost delta
- Today: <amount> vs. prior day: <amount> — delta: <pct>%
- Anomaly: <yes/no> — owner: <name> — investigation deadline: <date>
## Quota warnings
| Service | Current | Limit | Utilization | Action |
|---|---|---|---|---|
## Deployment health
- Cloud Deploy pipelines: <summary>
- Cloud Build: <summary>
- Deployment approval: <unblocked / BLOCKED pending review>
## SCC finding triage
| Severity | Finding | Age | Owner | Escalated |
|---|---|---|---|---|
## SLO burn rate
| SLO | Burn rate | Error budget remaining | Status |
|---|---|---|---|
## Action items
| Item | Owner | Deadline | Priority |
|---|---|---|---|
## Next 24-hour risk watch
- <risk item or explicit none>
```
