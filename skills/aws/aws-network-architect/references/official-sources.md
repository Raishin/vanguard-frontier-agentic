# Official sources

Use this reference only when you need source grounding for AWS service behavior or the detailed source list.

## AWS documentation

Use these as starting points, not as proof of the user's live AWS state:
- https://docs.aws.amazon.com/vpc/latest/userguide/vpc-security-best-practices.html
- https://docs.aws.amazon.com/vpc/latest/tgw/what-is-transit-gateway.html
- https://docs.aws.amazon.com/vpc/latest/tgw/tgw-vpc-attachments.html
- https://docs.aws.amazon.com/vpc/latest/privatelink/what-is-privatelink.html

## Grounding rule

Official documentation explains AWS service behavior. It does not prove the user's current account, Region, quota, resource configuration, IAM boundary, pricing, entitlement, or operational state. Prefer read-only AWS MCP or CLI evidence, repository evidence, or sanitized user-provided evidence for current-state claims.

## Current MCP/documentation refresh (2026-06-02)

Service facts from official docs:
- VPC security best practices include security groups, network ACLs, VPC Flow Logs, Network Firewall, and GuardDuty threat detection.
- Transit Gateway acts as a hub to interconnect VPCs and on-premises networks using attachments and route tables.

Sampled live evidence:
- Read-only regional availability sampling reported VPC, Transit Gateway, and PrivateLink as `isAvailableIn` in `us-east-1`, `us-west-2`, `eu-west-1`, and `ap-southeast-1`.
- Sampled APIs `EC2+DescribeVpcs`, `EC2+DescribeTransitGateways`, and `EC2+DescribeVpcEndpoints` were reported `isAvailableIn` in those regions.

Review implications:
- Network design review needs CIDR/IPAM, routing, segmentation, DNS, ingress/egress, endpoint policies, inspection path, logs, and failure-domain evidence.
- Service availability does not prove route-table correctness, asymmetric path safety, or blast-radius control.
