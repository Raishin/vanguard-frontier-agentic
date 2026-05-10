# Workflow and output contract

Use this reference only when performing domain classification, routing arbitration, or scope disambiguation for OVHcloud requests.

## Classification domains

Check these signals before assigning a domain verdict:

- **IAM** — policy URNs, `ovh_iam_policy` resources, OAuth2 credentials, identity groups, conditional access blocks
- **FinOps** — billing spikes, idle instances, unattached volumes, Savings Plans coverage gaps, tagging hygiene
- **Kubernetes** — MCK cluster lifecycle, node pool operations, version upgrades, RBAC, network policies
- **Networking** — vRack membership, VLAN segmentation, private network attachment, security groups, DNS zones
- **Live KMS guard** — key version destruction, key rotation decommission, scheduled deletion of `okms_service_key`

## Safe routing workflow

1. **Frame the request**
   - Requested operation and surface area:
   - Account / project ID (if stated):
   - Environment (production / staging / unknown):
   - Explicit non-goals:
2. **Collect classification signals**
   - Prefer OVHcloud API console or Terraform provider resource names as domain anchors.
   - Otherwise inspect repository IaC, user-provided config, or official OVHcloud docs.
   - Label each signal as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Resolve domain overlap**
   - If the request spans two domains, name both and route to the primary domain specialist with a note for the secondary.
   - If domain cannot be determined from context alone, ask exactly one clarifying question. Do not ask compound questions.
4. **Emit routing verdict**
   - Keep the verdict minimal: domain, specialist, signals, blockers, and one follow-up question if ambiguous.
   - Never attempt live OVHcloud API mutations from the routing layer.

## Output contract

Return this structure:

```markdown
# OVHcloud Routing Verdict: <brief task label>
## Domain verdict
- Primary domain: <IAM | FinOps | Kubernetes | Networking | Live KMS guard>
- Confidence: HIGH / MEDIUM / LOW
- Classification signals:
## Recommended specialist
- Skill: <ovhcloud-iam-policy-review | ovhcloud-cost-finops-analyst | ovhcloud-kubernetes-platform-operator | ovhcloud-network-architect | ovhcloud-live-kms-key-destruction-guard>
- Scope handed off: <concise scope statement>
## Blockers
- <list or explicit none>
## Clarifying question (if confidence is LOW)
- <single question or omit this section>
```
