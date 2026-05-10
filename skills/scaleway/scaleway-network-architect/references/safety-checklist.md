# Safety checklist

Use this reference before privileged, traffic-changing, production-impacting, or security-weakening Scaleway network recommendations.

## Non-negotiables

- Never ask users to paste `SCW_ACCESS_KEY`, `SCW_SECRET_KEY`, project IDs, organization IDs, or raw network resource IDs into chat. Work from sanitized Terraform state or sanitized network diagrams only.
- Do not invent security group rule sets, IP ranges, Load Balancer backend configurations, or zone-to-zone routing behavior.
- Always flag `enforced` placement group risk before recommending it — it can block pod or instance scheduling under node failure in the same zone.
- Always flag Private Network zone-boundary gaps where cross-zone communication is required but attachment is incomplete.
- Require explicit user approval before recommending security group rule changes on a production environment or Load Balancer configuration changes that could drop active connections.
- Never recommend opening inbound 0.0.0.0/0 on any port except 80/443 on a public-facing Load Balancer without explicit justification.
- Use Context7 or official Scaleway documentation for current VPC, security group, and Load Balancer behavior when the answer depends on Scaleway service details.

## Stress checks

- Which security group changes open inbound access to unintended IP ranges or ports?
- Which Private Network attachment gaps leave inter-service traffic routed over public interfaces?
- Which Load Balancer changes could cause a service disruption if health checks are reconfigured under live traffic?
- Could `enforced` placement group leave instances unschedulable after a zone maintenance event?
- What is the blast radius of a security group rule deletion applied to the wrong instance or zone?
- What cross-zone routing assumption is undocumented and could break under zone failure?

## Evidence labels

Use `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference`. Documentation alone never proves the user's live security group rules, active Private Network attachments, or Load Balancer backend health state. Findings labeled `inference` must be clearly marked as unconfirmed.
