# Workflow and Output Contract

## Safe Workflow

1. **Classify the workload and blast radius**
   - What is the app?
   - Who uses it?
   - What breaks if it fails?
   - What are the RTO/RPO and maintenance-window assumptions?
2. **Confirm plan and tier fit**
   - Verify plan SKU/tier, OS, region, instance count, and whether the plan supports the required features.
   - Challenge any design that wants production slots, autoscale, advanced networking, or stronger resiliency on an undersized or mismatched tier.
3. **Review ingress and dependency networking**
   - Separate inbound from outbound.
   - Inbound: public, access-restricted public, private endpoint, gateway-mediated, or internal-only.
   - Outbound: VNet integration, route-all behavior, private DNS, database/storage/Key Vault reachability, and container-registry pathing.
   - Reject any answer that says “private” without naming DNS, subnet ownership, and dependency routing.
4. **Review deployment and rollback posture**
   - Prefer nonproduction slot deployment and validated swap when supported.
   - Challenge direct-to-production deployment.
   - Verify slot-specific settings, warm-up assumptions, sticky config, and rollback speed.
5. **Review identity, secrets, and config hygiene**
   - Prefer managed identity over embedded secrets.
   - Use Key Vault references when secret separation is required.
   - Verify slot settings for environment-specific values.
   - Reject secrets in repo, pipeline variables, copied app settings, or chat.
6. **Review scaling and runtime safety**
   - Verify whether scale-up or scale-out is the real bottleneck.
   - Check autoscale assumptions, instance minimums, session state behavior, dependency bottlenecks, and background-job coupling.
   - Reject single-instance production unless the downtime and maintenance consequences are accepted explicitly.
7. **Review diagnostics and health signals**
   - Confirm health check endpoint design.
   - Confirm logs, metrics, alerting, and diagnostic ownership.
   - Use App Service detectors and diagnostics when available, but do not confuse detector output with complete observability.
8. **Review resilience and recovery**
   - Check zone-redundancy fit, regional failure expectations, backup coverage, restore method, restore frequency, and drill evidence.
   - Distinguish backup existence from restore readiness.
9. **Review operator readiness**
   - Name the on-call owner, release owner, rollback approver, dashboard, alert routing, escalation path, and last drill date.
   - If no one owns the failure path, it is not production-ready.
10. **Return a go/no-go verdict**
    - Name blockers, residual risks, required evidence, and the safest next action.

## Role-Specific Stress Checks

- Reject “Premium is production-ready by default.” SKU alone is not readiness.
- Reject “we use private endpoint” if public access, DNS, subnet separation, and dependency routing were not checked.
- Reject “we have VNet integration” if the team cannot explain whether it is solving outbound reachability, inbound exposure, or both. Those are different problems.
- Reject any release strategy that deploys continuously to the production slot when staging slots are available and downtime matters.
- Reject any slot strategy that ignores sticky settings, warm-up, health-check behavior, or rollback timing.
- Reject any secret model that uses plaintext app settings when managed identity plus Key Vault references are viable.
- Reject autoscale claims that ignore dependency bottlenecks, minimum safe instance count, or stateful session behavior.
- Reject “backups are enabled” unless restore target, restore time, slot/app overwrite behavior, and drill evidence are known.
- Reject “zone redundant” claims unless the plan actually supports it and instance count is sufficient.
- Reject shallow observability posture: no health endpoint, no actionable alerts, no release-event correlation, no named owner.
- Do not treat App Service diagnostics or Azure Advisor as authoritative proof that architecture risk is fully covered.

## Output Template

```markdown
# Azure App Service Production Readiness Review: <scope>

## Verdict
- Status: READY / READY WITH RISKS / NOT READY
- Biggest risk:
- Evidence level: live evidence / documentation-based / sanitized evidence / inference

## Scope
- Subscription or boundary:
- Resource group:
- App Service plan:
- Web app(s):
- Environment:
- Requested decision:

## Workload assumptions
- Traffic pattern:
- Availability target:
- RTO / RPO:
- Stateful dependencies:
- Release frequency:

## Findings
| Area | Finding | Severity | Evidence | Why it matters | Recommendation | Owner |
|---|---|---|---|---|---|---|

## Production controls review
| Control area | Expected state | Observed state | Gap | Blocking |
|---|---|---|---|---|
| Plan tier fit |  |  |  |  |
| Deployment slots and rollback |  |  |  |  |
| Ingress and private access |  |  |  |  |
| Outbound dependency reachability |  |  |  |  |
| Identity and secrets |  |  |  |  |
| Scale and capacity |  |  |  |  |
| Diagnostics and alerts |  |  |  |  |
| Backup and restore |  |  |  |  |
| Operator readiness |  |  |  |  |

## Safe next actions
1.
2.
3.

## Rollback / recovery posture
1.
2.
3.

## Open questions
-
```

## Red Flags

- The team wants a production verdict but cannot name the App Service plan SKU, instance count, or dependency network path.
- The rollout plan is direct-to-production even though slots are available and downtime matters.
- Private endpoint is enabled but DNS, public access posture, or subnet separation is unclear.
- VNet integration exists, but no one knows whether outbound traffic, backup, image pulls, Key Vault, or managed-identity paths are routed as intended.
- Secrets live in app settings with copied plaintext values instead of managed identity and vault-backed references.
- There is a health check path, but it is untrusted, unactionable, or not safe during swap.
- Autoscale is assumed to solve application bottlenecks with no dependency or state analysis.
- Backups exist, but restore has never been tested or the restore target behavior is misunderstood.
- Alerts fire, but nobody owns them.
- The answer depends on undocumented MCP capabilities or on inference presented as fact.
