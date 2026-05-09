---
name: gcp-waf-cost-optimization-review
description: "Evaluate GCP workload cost efficiency against the Google Cloud Well-Architected Framework cost optimization pillar — covering FinOps culture, cloud spending alignment with business value, resource rightsizing, commitment strategy, idle resource elimination, and continuous optimization. Use when reviewing cloud costs, designing cost-aware architectures, or identifying cost reduction opportunities in GCP."
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-09"
  category: finops
---

# GCP WAF Cost Optimization Review

## Purpose

Evaluate GCP workload cost efficiency against the Google Cloud Well-Architected Framework (WAF) cost optimization pillar. This skill supports the shift from CapEx to OpEx models, builds a FinOps culture with clear accountability, and drives continuous optimization aligned with business value.

## When to use

Use this skill for:

- Reviewing current cloud costs and identifying optimization opportunities
- Designing cost-aware architectures with managed services and right-sized resources
- Auditing a workload against the four WAF cost optimization principles
- Building cost attribution, visibility, and accountability structures
- Evaluating commitment strategies and idle resource elimination

## WAF Cost Optimization Pillar Overview

The GCP Well-Architected Framework cost optimization pillar guides organizations through the shift from capital expenditure (CapEx) to operational expenditure (OpEx), where costs are incurred on demand and tightly coupled to delivered business value. The pillar emphasizes FinOps culture — shared ownership of cost between engineering, finance, and product — alongside tooling for visibility, accountability, rightsizing, and continuous optimization.

## Core Principles

### 1. Align Cloud Spending with Business Value
Ensure that every dollar of cloud spend can be traced to a business outcome. Use resource labels and billing hierarchies to attribute costs to teams, products, and value streams. Evaluate cloud ROI by comparing cost against delivered user value.
- Reference: https://cloud.google.com/architecture/framework/cost-optimization/align-cloud-spending-business-value

### 2. Foster a Culture of Cost Awareness
Embed cost accountability into engineering teams rather than treating it as a finance function. Provide developers with cost visibility for the resources they own, set team-level budgets, and include cost in architectural and code review decisions.
- Reference: https://cloud.google.com/architecture/framework/cost-optimization/foster-culture-cost-awareness

### 3. Optimize Resource Usage
Select and configure GCP resources to match actual workload requirements. Apply rightsizing recommendations, use Spot VMs for fault-tolerant workloads, migrate to serverless or managed services where appropriate, and enforce storage lifecycle policies to eliminate unnecessary retention costs.
- Reference: https://cloud.google.com/architecture/framework/cost-optimization/optimize-resource-usage

### 4. Optimize Continuously
Treat cost optimization as an ongoing discipline, not a one-time exercise. Establish regular review cadences, act on Recommender and Active Assist insights, track savings against baselines, and iterate on commitment strategies as workload patterns evolve.
- Reference: https://cloud.google.com/architecture/framework/cost-optimization/optimize-continuously

## Relevant GCP Products

### Visibility
- **Cloud Billing Reports** — spend dashboards, cost breakdowns, and trend analysis in the GCP Console
- **BigQuery Billing Export** — export detailed billing data to BigQuery for custom analysis and Looker Studio dashboards
- **Looker Studio** — visualization layer for billing data, enabling team-level and product-level cost dashboards
- **Billing Alerts and Budgets** — automated alerts when spend exceeds defined thresholds per project or billing account

### Automation and Recommendations
- **Recommender / Active Assist** — rightsizing recommendations for Compute Engine, Cloud SQL, GKE, and more
- **Cloud Hub Optimization** — centralized view of optimization opportunities across the organization
- **FinOps Hub** — GCP-native FinOps dashboard aggregating savings opportunities and commitment coverage
- **Billing Quotas** — enforce per-project API quotas to prevent runaway spend

### Efficient Infrastructure
- **Cloud Run / Cloud Run Functions** — serverless compute with per-request billing, eliminating idle compute costs
- **GKE Autopilot** — managed GKE mode with per-pod billing and automatic node rightsizing
- **Spot VMs** — preemptible compute at up to 91% discount for fault-tolerant batch and stateless workloads
- **Committed Use Discounts (CUDs)** — 1-year and 3-year resource-based or spend-based commitments for predictable discount
- **Cloud Storage Lifecycle Policies** — automatically transition or delete objects based on age, class, and access frequency

### Governance
- **Resource Manager (Org / Folder / Project)** — hierarchy for cost boundary enforcement and policy inheritance
- **Labels** — key-value tags on resources enabling cost attribution to teams, products, and environments
- **Organization Policy Service** — enforce label requirements, restrict resource types, and prevent cost-inefficient configurations

## Assessment Question Bank

### Cost in Design
1. Are cost estimates produced before architectural decisions are finalized, not after deployment?
2. Are managed and serverless services evaluated as the default before choosing self-managed VMs?
3. Are storage classes and lifecycle policies defined at design time for all object storage workloads?
4. Are development and staging environments sized smaller than production and shut down outside working hours?
5. Are cost constraints treated as non-functional requirements alongside performance and reliability?
6. Are Spot VMs or preemptible nodes used for batch, CI/CD, or other fault-tolerant workloads?

### Team Cost Culture
1. Do engineering teams have visibility into the costs of the resources they own?
2. Are per-team or per-product cost budgets defined and communicated?
3. Is cost included as a review criterion in architecture and code reviews?
4. Are team cost targets tracked and reported in team-level dashboards?
5. Is there a FinOps champion or cost optimization guild coordinating cross-team efforts?
6. Are engineers recognized or incentivized for cost reduction contributions?

### Cost Monitoring and Attribution
1. Are all GCP resources labeled with team, product, environment, and cost center tags?
2. Is BigQuery billing export enabled and used for detailed cost analysis?
3. Are per-project billing budgets and alerts configured to surface anomalous spend?
4. Is there a process for investigating and explaining billing anomalies within 24 hours?
5. Are cost trends reviewed on a weekly or bi-weekly cadence?
6. Are unit economics tracked (cost per user, cost per transaction, cost per GB processed)?

### Compute Optimization
1. Are Compute Engine rightsizing recommendations from Recommender reviewed and acted on monthly?
2. Are sustained use discounts (SUDs) and committed use discounts (CUDs) coverage monitored and maintained?
3. Are GKE workloads using bin-packing or Autopilot to eliminate wasted node capacity?
4. Are Cloud Run or Cloud Run Functions used for workloads with variable or unpredictable request patterns?
5. Are GPU and TPU resources released when not actively in use?
6. Are dev/test VMs shut down automatically outside business hours?

### Over-Provisioning Prevention
1. Are Cloud SQL instances sized to actual p95 CPU and memory utilization, not peak-of-peak estimates?
2. Are GKE resource requests and limits set accurately to reflect actual pod utilization?
3. Are memory-optimized or compute-optimized machine families used where appropriate rather than general-purpose?
4. Are autoscaling policies configured to scale in aggressively as well as scale out?
5. Are network egress costs understood and minimized through routing and caching strategy?

### Data-Driven Continuous Optimization
1. Is there a monthly cost review meeting with finance, engineering, and product stakeholders?
2. Are Active Assist and Recommender insights reviewed, accepted, or dismissed with documented rationale?
3. Are savings realized from optimization tracked against baselines and reported to leadership?
4. Is the CUD commitment strategy reviewed quarterly as workload patterns change?
5. Are unattached persistent disks, unused IP addresses, and idle load balancers identified and removed?
6. Are BigQuery slot reservations and on-demand spend reviewed against actual query patterns?

## Validation Checklist

### Cost Attribution
- [ ] 100% of billable GCP resources labeled with team, product, environment, and cost-center labels
- [ ] BigQuery billing export enabled on the billing account and queried regularly
- [ ] Per-project billing budgets configured with email and Pub/Sub alerts
- [ ] Unit economics defined and tracked (cost per user, cost per transaction)

### Visibility and Monitoring
- [ ] Looker Studio or equivalent cost dashboard available to all engineering teams
- [ ] Billing anomaly alerts configured to notify within 24 hours of unexpected spend
- [ ] Weekly or bi-weekly cost review cadence established

### Rightsizing
- [ ] Recommender rightsizing recommendations reviewed and acted on monthly for Compute Engine
- [ ] GKE resource requests and limits set based on VPA recommendations or actual utilization
- [ ] Cloud SQL instances sized to p95 CPU and memory utilization

### Commitment Strategy
- [ ] CUD coverage reviewed monthly against eligible committed workloads
- [ ] Spend-based CUDs used where resource-based CUDs do not apply
- [ ] SUDs and CUD coverage tracked in FinOps Hub or equivalent dashboard

### Idle Resource Elimination
- [ ] Unattached persistent disks identified and deleted or snapshotted
- [ ] Unused static IP addresses released
- [ ] Idle load balancers and forwarding rules removed
- [ ] Dev/test environments shut down automatically outside business hours

### Serverless and Managed Services
- [ ] Serverless-first evaluation performed for all new compute workloads
- [ ] Cloud Run or Cloud Run Functions used for variable-traffic services
- [ ] GKE Autopilot evaluated for container workloads without node-level requirements

### Storage Lifecycle
- [ ] Cloud Storage lifecycle policies active on all buckets, transitioning to Nearline/Coldline/Archive by access pattern
- [ ] BigQuery table partitioning and clustering applied to reduce query costs
- [ ] Snapshot and backup retention policies defined and enforced

## Response Shape

1. **Scope** — workload name, GCP resource hierarchy scope, evidence level (live billing data / sanitized / documentation-based / inference)
2. **Cost Attribution Assessment** — review of labeling coverage, billing export configuration, and cost visibility tooling
3. **Visibility Gaps** — missing dashboards, alerts, or monitoring that reduce cost awareness
4. **Rightsizing Opportunities** — specific compute, database, and storage resources identified as over-provisioned with estimated savings
5. **Commitment Strategy** — review of CUD/SUD coverage, commitment gaps, and recommended commitment approach
6. **Idle Resources** — unattached disks, unused IPs, idle load balancers, and orphaned resources identified
7. **Managed Services Fit** — workloads that would reduce cost or operational overhead by moving to serverless or managed services
8. **Prioritized Savings Actions** — ordered by estimated annual savings impact (High / Medium / Low), each with minimum required change, validation step, and rollback procedure
