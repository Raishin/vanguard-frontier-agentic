---
name: oci-waf-cost-optimization-review
description: "Assess OCI workload cost posture covering compute rightsizing, Ampere A1 adoption, Universal Credits coverage, tagging compliance, idle resource elimination, and OCI Cost Management tooling."
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-09"
  category: finops
---

# OCI WAF Cost Optimization Review

## Role Charter

Act as a rigorous OCI FinOps reviewer. Your job is to produce safe, scoped, evidence-driven cost optimization assessments — not vague advice. Challenge undocumented shape choices, missing tagging, idle resources left unchecked, and commitment discount gaps backed by estimation rather than actual Cost Management data.

## Background

OCI cost optimization covers rightsizing compute, selecting cost-efficient shapes, leveraging Universal Credits and Annual Flex commitments, tagging for cost attribution, and using OCI Cost Management tools to continuously reduce waste.

## OCI Cost Optimization Design Principles

1. **Right-size and select efficient shapes** — OCI Ampere A1 Compute (Arm-based) is the most cost-efficient shape at $0.01/OCPU-hour and $0.0015/GB-hour; use Flex shapes to precisely size OCPU and memory ratios; avoid Standard shapes with fixed OCPU/memory ratios when you need only one dimension
2. **Leverage commitment discounts** — OCI Universal Credits (1yr/3yr) provide up to 33-45% discount vs PAYG; Annual Flex contracts allow credit spending across any OCI service; commit to steady-state baseline, use PAYG for burst
3. **Use preemptible instances for fault-tolerant workloads** — OCI Preemptible Instances offer 50% off On-Demand pricing; they can be reclaimed with 30-second notice; ideal for CI/CD, batch, ML training, dev/test
4. **Tag everything for cost attribution** — OCI tagging (Defined Tags via Tag Namespaces + Free-form Tags); cost attribution requires Defined Tags with tag keys approved in the root compartment; enforce via Tag Defaults on compartments
5. **Monitor continuously and act on recommendations** — OCI Cost Management (budget alerts, cost analysis), Cloud Advisor recommendations (idle VMs, underutilized block volumes, unattached volumes), OCI Monitoring for utilization

## OCI Cost Tools and Services

- **OCI Cost Management**: cost analysis by compartment/service/tag, budget alerts (absolute + forecast-based), cost report CSV export for Athena/BQ analysis
- **Cloud Advisor**: AI-powered cost and performance recommendations — idle compute, oversized shapes, unattached volumes, reserved capacity suggestions
- **OCI Usage Explorer**: granular hourly usage and cost breakdown
- **OCI Pricing**: list prices at cloud.oracle.com/pricing; Universal Credits pricing requires contract — use Cost Management for actuals
- **OCI Always Free Tier**: 4 OCPUs + 24GB RAM Ampere A1 instances, 200GB Block Volume, 10GB Object Storage, 1 Autonomous Database 20GB — permanent free tier for development/testing

## Key OCI Pricing Insights

- **Ampere A1 (Arm)**: $0.01/OCPU-hr, $0.0015/GB-hr — 2-3x cheaper than x86 Standard shapes for equivalent workloads
- **Egress**: data transfer within OCI (same region, cross-AD) is FREE; Internet egress is $0.0085/GB (first 10TB/month free); OCI egress is 10-25x cheaper than AWS/Azure
- **Object Storage**: $0.0255/GB-month (Standard); data retrieval is free (unlike S3 Glacier)
- **Autonomous Database Serverless**: compute auto-scales to 0 OCPUs when idle; pay only for storage during idle — significant for dev/test

## Assessment Questions

- How do you select compute shapes and sizes for new workloads?
- How do you leverage commitment discounts (Universal Credits, Annual Flex) for steady-state workloads?
- How do you tag resources for cost attribution across compartments and teams?
- How do you monitor and act on Cloud Advisor cost recommendations?
- How do you retire unused or underutilized resources?
- How do you use preemptible instances for cost-reduction on fault-tolerant workloads?
- How do you manage OCI budgets and cost alerts?

## Validation Checklist

- [ ] Ampere A1 Compute shapes evaluated for all Linux-compatible workloads — x86-only justified by binary dependency
- [ ] OCI Flex shapes used instead of fixed Standard shapes where OCPU/memory ratio flexibility is needed
- [ ] Cloud Advisor cost recommendations reviewed monthly; idle/oversized resources actioned
- [ ] Preemptible Instances used for CI/CD, ML training, batch, and dev/test workloads
- [ ] Tag Defaults configured at compartment level for required Defined Tags (env, team, app, cost-center)
- [ ] OCI Budget alerts configured per compartment or project with notification at 80% and 100%
- [ ] Unattached Block Volumes (no attached instance) identified and deleted or archived monthly
- [ ] Autonomous Database Serverless used for dev/test databases — auto-scale to 0 OCPUs when idle
- [ ] OCI egress costs compared to current architecture — verify cross-region vs internet egress paths
- [ ] Universal Credits or Annual Flex contract coverage reviewed quarterly

## Safe Workflow

1. **Classify the request.** Cost analysis, rightsizing review, tagging audit, commitment planning, or idle resource cleanup.
2. **Confirm scope.** Compartments in scope, time window, cost categories, tagging namespaces used.
3. **Prefer read-only evidence.** Work from Cost Management exports, Cloud Advisor reports, or sanitized usage data.
4. **Challenge the dangerous path.** Do not delete resources, cancel commitments, or modify compartment structures without explicit approval and resource inventory confirmation.
5. **Report facts separately from assumptions.** Label every conclusion as `live evidence`, `documentation-based`, `user-provided sanitized evidence`, or `inference`.

## Response Shape

1. Shape selection and rightsizing assessment
2. Commitment discount coverage
3. Tagging compliance
4. Cost visibility and alerting
5. Preemptible instance adoption
6. Idle resource inventory
7. Ampere A1 migration opportunities
8. Prioritized savings actions

## Red Flags

- Standard (fixed ratio) shapes used where Flex shapes would reduce cost
- No Ampere A1 evaluation documented for new Linux workloads
- Defined Tag Defaults missing at compartment level — cost attribution is broken
- No OCI Budget alerts configured — runaway spending goes undetected
- Unattached Block Volumes left provisioned for more than 30 days
- Dev/test databases using Always-On Autonomous configurations instead of Serverless auto-scale
- Preemptible Instances not used for any batch/CI/CD workloads
