# Workflow and output contract

Use this reference only when performing a full cost anomaly investigation or FinOps alert pipeline review.

## Review domains

Check these areas before giving a recommendation:

- Billing account context: CN-* mainland China vs international — confirmed before analysis begins
- MaxCompute billing mode: CU subscription vs on-demand; CU coverage of actual workload; query cost estimation gates
- ECS spot interruption: Auto Scaling group instance type priority; pay-as-you-go fallback trigger conditions
- CDN billing: bandwidth billing model (95th percentile vs traffic); bandwidth cap configured; WAF protection present
- OSS API request cost: bucket access control; Referer whitelist; request rate monitoring alerts
- Budget alerts: threshold configuration; notification channels (DingTalk webhook + email); alert lead time
- Preventive controls: instance quantity limits; MaxCompute job cost quotas; credit package hard limits
- Remediation playbook: documented response steps per anomaly type; owner assigned; last tested date

## Safe workflow

1. **Frame the anomaly**
   - Billing account context (CN-* vs international):
   - Anomaly type (MaxCompute / ECS spot / CDN / OSS / other):
   - Anomaly detection source (budget alert / manual review / billing API):
   - Approximate spend delta (sanitized, no specific customer figures):
2. **Collect evidence**
   - Prefer live Cost Management console screenshots or BSS API output.
   - Otherwise inspect billing reports, sanitized user evidence, or official Alibaba Cloud docs.
   - Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Stress-test the cost controls**
   - Is the budget alert threshold set below the anomaly spend level?
   - Does the DingTalk webhook deliver notifications reliably in the CN-* context?
   - Are there preventive controls beyond notification-only budgets?
   - Is the Auto Scaling group configured to prefer spot types over pay-as-you-go?
   - Is MaxCompute on-demand billing at risk from unguarded large queries?
4. **Recommend the smallest safe next step**
   - Prioritize by risk: active cost blowout requiring immediate stop > missing preventive control > missing DingTalk notification > incomplete remediation playbook.
   - If active cost blowout is detected, recommend immediate action before further analysis.

## Output contract

Return this structure:
```markdown
# Alibaba Cloud Cost Anomaly Review: <anomaly scope>
## Executive summary
- Anomaly verdict:
- Evidence level:
- Immediate action required:
## Billing account context
- Account type: CN-* mainland China / international
- Confirmed by:
## MaxCompute billing posture
- Billing mode: CU subscription / on-demand
- CU coverage: adequate / gap
- Query cost estimation gate: present / absent
- Anomaly risk:
## ECS spot and Auto Scaling cost risk
- Spot interruption risk: low / medium / high
- Pay-as-you-go fallback configured: yes / no
- Instance type priority order: correct / gap
## CDN and OSS cost anomalies
- CDN bandwidth cap: set / not set
- CDN WAF protection: present / absent
- OSS bucket access control: correct / gap
- OSS Referer whitelist: set / not set
## Budget alert and notification configuration
- Budget threshold set:
- DingTalk webhook configured:
- Email notification configured:
- Preventive controls beyond alerts:
## Remediation playbook
- Documented response steps: present / absent
- Owner assigned: yes / no
- Last tested date:
## Cost anomaly response prioritization
1. <action> — priority: <critical/high/medium>, effort: <low/medium/high>
## Open questions
1. <question> — owner: <owner>, impact: <impact if unresolved>
```
