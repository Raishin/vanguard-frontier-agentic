# Workflow and output contract

Use this reference only when performing a full Kubecost or OpenCost chargeback readiness review, producing FinOps implementation guidance, triaging a cost allocation discrepancy, or completing a cost governance production-readiness pass.

## Review domains

Check these areas before giving a verdict:

- Kubecost vs OpenCost distinction and version
- Cost allocation accuracy: all cost components enabled (compute, storage, network)
- Label taxonomy completeness: uncategorized cost percentage, missing label coverage
- Shared cost model: even split, proportional, or weighted — and whether it matches the chargeback agreement
- Idle cost attribution: absorbed centrally or allocated to namespace owners
- Budget alert configuration: thresholds, routing, and coverage
- Cost API and frontend authentication posture
- Savings recommendations status: HIGH-priority items and days unactioned

## Safe workflow

1. **Frame scope**
   - Cluster name and cloud provider:
   - Kubecost version (`helm list -n kubecost` or `kubectl get deployment -n kubecost -o json | jq '.items[].spec.template.spec.containers[].image'`):
   - OpenCost or Kubecost (free tier / Business / Enterprise):
   - Number of clusters in scope:
   - Required outcome of this review:
   - Explicit non-goals:

2. **Collect evidence**
   - Prefer Kubecost allocation API output, Helm values, and `kubectl` label query results as primary evidence.
   - Supplement with Kubecost UI screenshots and savings recommendations export if available.
   - Label each finding as `live evidence`, `user-provided evidence`, `documentation-based`, or `inference`.

3. **Verify all cost components are captured**
   Query the allocation API to confirm compute, storage, and network are all present:
   ```bash
   # Allocation API — last 7 days by namespace
   curl "http://localhost:9090/model/allocation?window=7d&aggregate=namespace&includeIdle=true"

   # Check whether network costs are enabled in Helm values
   helm get values kubecost -n kubecost | grep -A5 'networkCosts'

   # Check whether PV costs are included
   helm get values kubecost -n kubecost | grep -A5 'persistentVolumes'
   ```
   If `networkCosts.enabled: false` or PV costs are missing from the allocation response,
   flag as MEDIUM — stateful or egress-heavy teams face invisible costs.

4. **Assess label taxonomy completeness**
   Run a label audit to quantify unlabeled pods:
   ```bash
   # Count pods missing the cost-center label
   kubectl get pods -A --show-labels | grep -v 'cost-center=' | grep -v 'NAME' | wc -l

   # Count pods missing the team label
   kubectl get pods -A --show-labels | grep -v 'app.kubernetes.io/team=' | grep -v 'NAME' | wc -l

   # Query Kubecost for uncategorized spend share
   curl "http://localhost:9090/model/allocation?window=7d&aggregate=label:cost-center" | \
     jq '.data[0]["__unallocated__"].totalCost / (.data[0] | to_entries | map(.value.totalCost) | add)'
   ```
   If the `__unallocated__` or `__idle__` bucket represents more than 20% of total cost,
   label taxonomy is insufficient for chargeback — flag as HIGH.

5. **Check shared cost model configuration**
   Kubecost shared cost models in `values.yaml`:
   ```yaml
   # Option 1: even split (each tenant pays equal share of shared infra)
   kubecostModel:
     sharedCostConfiguration:
       shareIdle: false
       sharedNamespaces: "monitoring,ingress-nginx,cert-manager"
       shareByLabel: ""
       shareType: "even"   # even | weighted | proportional

   # Option 2: proportional (tenant pays proportional to their usage)
   shareType: "proportional"

   # Option 3: weighted (explicit percentage per tenant)
   shareType: "weighted"
   ```
   If the shared cost model does not match the documented business chargeback agreement, flag as MEDIUM.
   If no shared namespace is configured, monitoring and ingress costs are silently excluded from bills.

6. **Verify idle cost attribution**
   ```bash
   # Check idle allocation setting
   helm get values kubecost -n kubecost | grep -A3 'idle'

   # Idle cost API
   curl "http://localhost:9090/model/allocation?window=7d&aggregate=namespace&includeIdle=true" | \
     jq '.data[0].__idle__'
   ```
   If `shareIdle: false` and the `__idle__` bucket is large (>15% of total), idle waste is hidden
   from engineering teams. Allocating idle to namespaces creates incentive to right-size.
   Flag as MEDIUM if idle cost is absorbed centrally without a documented policy decision.

7. **Audit budget alert configuration**
   ```bash
   # Check for configured budget alerts via Kubecost API
   curl "http://localhost:9090/model/budget"

   # Check Kubecost alert configuration in values
   helm get values kubecost -n kubecost | grep -A20 'alerts'
   ```
   A well-configured alert:
   ```yaml
   alerts:
     - type: budget
       threshold: 80       # alert at 80% — not 100%
       window: monthly
       aggregation: namespace
       filter: "namespace=team-a"
       slackWebhookUrl: https://hooks.slack.com/services/...
   ```
   No budget alerts configured for any namespace is a MEDIUM finding.
   Alert threshold at 100% (no early warning) is a MEDIUM finding.
   Alert routing to a central ops black hole (not the owning team) is a MEDIUM finding.

8. **Check cost API and frontend authentication**
   ```bash
   # Test whether the cost API is publicly accessible without credentials
   curl -o /dev/null -s -w "%{http_code}" http://<kubecost-service>:9090/model/allocation?window=1d

   # Check ingress auth annotation
   kubectl get ingress -n kubecost -o yaml | grep -A5 'annotations'
   ```
   Expected annotations for SSO-gated ingress:
   ```yaml
   annotations:
     nginx.ingress.kubernetes.io/auth-url: "https://oauth2-proxy/oauth2/auth"
     nginx.ingress.kubernetes.io/auth-signin: "https://oauth2-proxy/oauth2/start"
   ```
   A 200 response from the allocation API without auth headers means any cluster pod can enumerate
   other teams' spend data — flag as HIGH.

9. **Savings recommendations review**
   ```bash
   # Get rightsizing recommendations
   curl "http://localhost:9090/model/savings/requestSizingV2"

   # Get abandoned workload recommendations
   curl "http://localhost:9090/model/savings/abandonedWorkloads"

   # Get orphaned PV recommendations
   curl "http://localhost:9090/model/savings/orphanedResources"
   ```
   Review the top 10 recommendations by estimated monthly savings. For each HIGH-priority item,
   confirm whether it has been reviewed. Items unactioned for more than 30 days represent
   measurable cash waste with a documented fix path — flag as HIGH.

## Output contract

Return this structure:

```markdown
# Kubecost Chargeback and Allocation Review: <cluster-name>

## Executive verdict
- Status: CHARGEBACK READY / PARTIALLY READY / NOT READY / NEEDS EVIDENCE
- Biggest risk:
- Evidence level:

## Scope and assumptions
- Cluster name and cloud provider:
- Kubecost version and tier:
- Review window:
- Confirmed:
- Unknown:
- Out of scope:

## Findings

| Severity | Area | Finding | Evidence | Why it matters | Minimum safe action |
|---|---|---|---|---|---|

## Cost component coverage

| Component | Enabled | Notes |
|---|---|---|
| Compute (CPU/RAM) | | |
| Persistent volume storage | | |
| Network egress (cross-AZ) | | |
| Network egress (cross-region) | | |
| GPU | | |

## Label taxonomy summary
- Total pod count:
- Pods missing `cost-center` label:
- Estimated uncategorized cost %:

## Shared cost and idle model
- Shared namespaces:
- Share type:
- Idle allocation policy:

## Budget alert coverage
- Namespaces with budget alerts:
- Earliest warning threshold:
- Alert routing:

## Top savings opportunities

| Recommendation | Est. monthly savings | Days open | Action |
|---|---|---|---|

## Recommended actions
1. <action> — owner: <owner>, validation: <check>, rollback: <rollback>

## Validation
- Commands or checks:
- Expected result:

## Residual risk
- <risk or explicit none>
```
