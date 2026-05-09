# Official sources

Use this reference only when you need source grounding for GCP VPC Service Controls or Access Context Manager behavior or the detailed source list.

## GCP documentation

Use these as starting points, not as proof of the user's live GCP state:

- https://cloud.google.com/vpc-service-controls/docs/overview — VPC-SC overview, perimeter types, access policy structure
- https://cloud.google.com/vpc-service-controls/docs/dry-run-mode — dry-run mode setup, violation log format, transition to enforcement
- https://cloud.google.com/vpc-service-controls/docs/troubleshooting — violation reason codes, root cause analysis methodology
- https://cloud.google.com/access-context-manager/docs/overview — ACM overview, access level types (basic, custom), policy structure
- https://cloud.google.com/vpc-service-controls/docs/create-service-perimeters — perimeter creation, ingress/egress rules, bridge perimeter design
- https://cloud.google.com/vpc-service-controls/docs/ingress-egress-rules — ingress and egress rule syntax, ACM level integration
- https://cloud.google.com/vpc-service-controls/docs/supported-products — supported GCP services list and restrictions per service
- https://cloud.google.com/run/docs/securing/using-vpc-service-controls — Cloud Run inside VPC-SC perimeters, VPC Accessible Services configuration

## Grounding rule

Official documentation explains GCP service behavior. It does not prove the user's current perimeter configuration, dry-run violation count, ACM level conditions, or enforcement mode status. Prefer sanitized user-provided evidence (access policy exports, Terraform/IaC, violation log exports) for current-state claims. Never infer production perimeter state from documentation alone.
