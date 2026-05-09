# Workflow and output contract

Use this reference only when performing a full network architecture design, hybrid connectivity assessment, or production-readiness review.

## Design domains

Check these areas before giving a verdict:

- VPC topology: CIDR planning, subnet segmentation, AZ spread, security groups, NACLs
- ELB: type selection (Dedicated vs Shared), listener protocol, backend group health, TLS policy
- Hybrid connectivity: VPN Gateway (IPsec/SSL) vs DC Gateway (Direct Connect via VBC), redundancy
- Cloud Connect: cross-region or cross-account VPC peering, bandwidth packages
- CFW: east-west policy, internet ingress/egress, IPS signature coverage
- Anti-DDoS: EIP binding coverage, protection threshold configuration
- DNS: zone design, private vs public resolution, split-horizon

## Safe workflow

1. **Frame scope**
   - Workload type and connectivity requirements:
   - On-prem or multi-cloud integration:
   - Current-state evidence:
   - Required outcome:
   - Explicit non-goals:
2. **Collect evidence**
   - Prefer live Huawei Cloud console evidence if available.
   - Otherwise inspect IaC/config, sanitized user evidence, or official Huawei Cloud docs.
   - Label each finding as `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
3. **Stress-test design**
   - What fails if a single AZ loses connectivity?
   - What is the blast radius of a misconfigured CFW rule?
   - Which EIPs lack Anti-DDoS binding?
   - What DC Gateway redundancy exists?
   - What evidence is missing?
4. **Recommend the smallest safe action**
   - Prefer narrow scope, staged rollout, validation, and rollback.
   - If the safest action is to stop and gather evidence, say that plainly.

## Output contract

Return this structure:

```markdown
# Huawei Cloud Network Architecture: <scope>
## Connectivity requirements
## VPC topology
## ELB type selection
## Hybrid connectivity design
## CFW policy assessment
## Anti-DDoS coverage
## Open questions
```

Each section must include an evidence level label.
