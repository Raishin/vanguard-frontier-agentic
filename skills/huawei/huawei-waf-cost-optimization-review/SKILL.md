---
name: huawei-waf-cost-optimization-review
description: "Assess Huawei Cloud cost efficiency using the Well-Architected Framework Cost Optimization pillar: ECS flavor selection including Kunpeng Arm, Yearly/Monthly vs Pay-Per-Use billing, Spot Instances, Enterprise Project cost attribution, and Cost Center monitoring."
allowed-tools: Read Grep Glob
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-05-09"
  category: finops
---

# Huawei WAF Cost Optimization Review

## Purpose

Act as the Huawei Cloud Well-Architected Framework Cost Optimization reviewer who assesses workloads through ECS flavor selection (including Kunpeng Arm-based), Yearly/Monthly vs Pay-Per-Use billing modes, Spot Instance adoption for interruptible workloads, Enterprise Project cost attribution, and Cost Center continuous spend monitoring.

## When to use

Use this skill for:

- ECS flavor selection review: flavor naming convention, Kunpeng Arm opportunity, price/performance comparison
- Billing mode optimization: Yearly/Monthly subscription vs Pay-Per-Use vs Spot for each workload type
- Spot Instance adoption: identify batch, ML training, CI/CD, and dev/test candidates
- Enterprise Project cost attribution: resource assignment, tag policy, cost center filtering
- Cost monitoring and governance: Cost Center budgets, Cloud Advisor rightsizing, idle resource detection
- OBS storage tier optimization: Standard vs Infrequent Access vs Archive lifecycle rules
- EIP bandwidth billing mode review: Traffic billing vs Bandwidth billing vs Shared Bandwidth Pool

## Cost Optimization Design Principles

1. **Select cost-efficient ECS flavors** — Huawei Cloud ECS flavors use a naming convention (e.g., c7n.xlarge.4 = compute-optimized, 7th gen, 4 vCPUs : 16GB RAM); Kunpeng-based flavors (k suffix, e.g., c7n.xlarge.4k) provide ~20-30% better price/performance for Arm-compatible workloads; use flavor advisor or pricing page for comparison
2. **Choose the right billing mode** — Yearly/Monthly (subscription): 20-50% discount vs Pay-Per-Use; lock-in for 1/2/3 months or 1/2/3 years; cannot be refunded after 5 days; Pay-Per-Use: flexible, good for auto-scaled and short-lived workloads; Spot: significant discounts for interruptible batch/ML/CI workloads
3. **Use Spot ECS for fault-tolerant workloads** — Huawei Cloud Spot Instances bid against a market price; interrupted with a 2-minute warning when market price exceeds bid; ideal for MapReduce/MRS jobs, model training, CI/CD agents, dev/test
4. **Attribute costs with Enterprise Projects** — Enterprise Projects (not separate accounts) are the primary cost attribution unit in Huawei Cloud; assign all resources to an Enterprise Project; Cost Center filters by Enterprise Project; enforce via tag policy or account governance
5. **Monitor and optimize continuously** — Huawei Cloud Cost Center provides: cost analysis by service/Enterprise Project/region/tag, budget alerts, reserved instance recommendations, and optimization suggestions; Huawei Cloud Advisor for rightsizing; CBR (Cloud Backup and Recovery) cost review for excessive snapshot retention

## Huawei Cloud Cost Tools

- **Cost Center (Billing Console)**: multi-dimensional cost analysis, budgets and alerts, cost trends, optimization suggestions, Reserved Instance recommendations
- **Cloud Advisor (Cost module)**: idle ECS detection (CPU <10% for 7 days), oversized RDS, unattached EVS disks, unused EIPs
- **Enterprise Project**: cost attribution grouping; up to 200 enterprise projects per account; assign resources at creation time or migrate existing
- **Yearly/Monthly Pre-Purchase**: up to 50% off Pay-Per-Use for stable workloads; auto-renewal supported; conversion from Pay-Per-Use possible for running instances
- **Reserved Instances**: similar to Yearly/Monthly but at the account or organization level; newer accounts may prefer Yearly/Monthly for simplicity

## Key Huawei Cloud Pricing Insights

- **ECS Kunpeng (Arm, k suffix)**: typically 20-30% cheaper than equivalent Intel/AMD flavor for compatible workloads; significant savings for large-scale stateless services
- **OBS pricing**: Standard $0.0199/GB-month, Infrequent Access $0.015/GB, Archive $0.0045/GB — OBS retrieval from Archive takes minutes; plan accordingly
- **EVS (Elastic Volume Service)**: SSD ($0.0015/GB-hour Pay-Per-Use) vs HDD ($0.0005/GB-hour); unattached EVS volumes still incur storage charges — identify and delete or snapshot+delete
- **EIP (Elastic IP)**: Bandwidth billing vs Traffic billing; Traffic billing is better for bursty internet workloads; Bandwidth billing for steady high-throughput; Shared Bandwidth Pool for multiple EIPs on shared allocation
- **Inter-AZ traffic**: free within same region (unlike AWS $0.01/GB); internet egress tiered starting at $0.12/GB (CN regions) — significant for media/CDN workloads

## Assessment Questions

- How do you select ECS flavors and evaluate Kunpeng Arm options?
- How do you use Yearly/Monthly vs Pay-Per-Use billing for different workload types?
- How do you leverage Spot ECS for interruptible batch or dev workloads?
- How do you attribute costs to teams or projects via Enterprise Projects?
- How do you monitor costs and act on Cloud Advisor recommendations?
- How do you manage OBS storage tiers and avoid unnecessary archive retrieval costs?
- How do you identify and eliminate idle resources (EVS, EIP, stopped ECS)?
- How do you manage EIP bandwidth billing vs traffic billing?

## Validation Checklist

- [ ] Kunpeng (Arm) ECS flavors evaluated for all stateless Linux workloads — x86-only justified by binary dependency
- [ ] Yearly/Monthly pricing used for ECS instances with stable, predictable workloads (uptime >60% monthly)
- [ ] Spot ECS used for interruptible workloads (MRS jobs, model training, CI/CD agents, dev/test)
- [ ] All resources assigned to an Enterprise Project at creation time; cost attribution 100% for production
- [ ] Cost Center budgets configured per Enterprise Project with email alerts at 80% and 100%
- [ ] Cloud Advisor cost recommendations reviewed monthly; idle ECS (CPU <10% for 7 days) actioned
- [ ] Unattached EVS volumes and unused EIPs identified and released monthly
- [ ] OBS lifecycle rules: transition to Infrequent Access after 30 days, Archive after 90 days for non-critical data
- [ ] EIP billing mode reviewed: Traffic billing for bursty workloads, Bandwidth billing for steady high-throughput
- [ ] Dev/test ECS instances scheduled to stop outside business hours via CTS-triggered automation

## Response Shape

1. ECS flavor selection and Kunpeng opportunity
2. Billing mode optimization
3. Spot Instance adoption
4. Enterprise Project cost attribution
5. Cost monitoring and alerting
6. OBS storage tiers
7. Idle resource inventory
8. EIP bandwidth optimization
9. Prioritized savings actions
