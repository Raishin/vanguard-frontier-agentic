# Official sources

Use this reference only when you need source grounding for GCP security posture service behavior or the detailed source list.

## GCP documentation

Use these as starting points, not as proof of the user's live GCP state:

- https://cloud.google.com/security-command-center/docs/concepts-security-command-center-overview — SCC overview, tier differences (Standard vs. Premium), finding categories
- https://cloud.google.com/security/benchmarks/google-cloud-cis-benchmarks — CIS GCP Benchmark v2.0 download and control reference
- https://cloud.google.com/resource-manager/docs/organization-policy/org-policy-constraints — full org policy constraint catalog with descriptions
- https://cloud.google.com/binary-authorization/docs/overview — Binary Authorization architecture, attestors, and policy design
- https://cloud.google.com/assured-workloads/docs/overview — Assured Workloads compliance boundary overview and supported frameworks
- https://cloud.google.com/security-command-center/docs/how-to-use-event-threat-detection — Event Threat Detection rule set and detection categories (Premium only)
- https://cloud.google.com/security-command-center/docs/concepts-vulnerabilities-findings — SCC vulnerability finding types and severity mapping
- https://cloud.google.com/vpc-service-controls/docs/overview — VPC Service Controls as complementary perimeter control

## Grounding rule

Official documentation explains GCP service behavior. It does not prove the user's current SCC tier, org policy state, Assured Workloads configuration, or Binary Authorization policy. Prefer sanitized user-provided evidence (SCC exports, `gcloud` output, Terraform state) for current-state claims. Never infer production posture from documentation alone.
