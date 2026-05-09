---
name: azure-waf-cost-optimization-review
description: "Review Azure workload cost posture against the Well-Architected Framework Cost Optimization pillar: cost modeling, rightsizing, reservations, hybrid benefit, storage lifecycle, and idle resource elimination."
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-09"
  category: finops
---

# Azure WAF Cost Optimization Review

The Azure Well-Architected Framework Cost Optimization pillar focuses on designing workloads that maximize the value delivered while minimizing unnecessary Azure spending.

## 5 Cost Optimization Design Principles

1. **Develop a cost model** — model costs before deployment; estimate spend based on requirements; track cost KPIs
2. **Design with a cost-efficiency mindset** — favor managed services, consumption-based pricing, and right-sized resources
3. **Design for usage optimization** — enable auto-scaling; schedule workloads; use spot/low-priority VMs for fault-tolerant jobs
4. **Design for rate optimization** — commit to consistent usage via Azure Reservations and Savings Plans; leverage hybrid benefits
5. **Monitor and optimize over time** — track spend continuously; act on cost recommendations; retire underutilized resources

## Azure Cost Tools

- **Microsoft Cost Management + Billing** — cost analysis, budgets, cost alerts, export to storage for Parquet/Athena analysis
- **Azure Advisor** — cost recommendations: resize/shutdown VMs, buy reservations, delete unused disks
- **Azure Monitor + cost alerts** — alert on budget thresholds
- **Azure Pricing Calculator** — pre-deployment cost estimation
- **Azure Cost Management Exports** — raw billing data to Azure Storage for Power BI / Excel analysis
- **Azure Carbon Optimization** (preview) — sustainability tracking for carbon footprint reporting

## Cost Optimization Areas

### Compute
- Right-size VMs using Azure Advisor; use Burstable VMs (B-series) for dev/test
- Spot VMs — up to 90% off list price for interruptible workloads (CI/CD, ML training, batch)
- Azure Batch Spot nodes — bursty HPC at spot pricing
- Auto-shutdown dev VMs outside business hours via Azure DevTest Labs or scheduled stop

### Reservations and Savings Plans
- **1yr/3yr Azure Reservations** — VMs, SQL, Cosmos DB, App Service, AKS nodes; committed capacity discount
- **Azure Savings Plans for Compute** — covers VMs, VMSS, AKS, App Service, Functions across regions, SKU families, and sizes; no instance flexibility restrictions

### Azure Hybrid Benefit
- **Windows Server** — up to 40% off VM pricing using existing Software Assurance licenses
- **SQL Server** — up to 55% off Azure SQL Database and SQL Managed Instance
- **Dev/Test pricing** — reduced rates for Visual Studio subscribers on eligible services

### Storage
- **Azure Blob lifecycle management** — tiered transition: Hot → Cool → Cold → Archive based on last access time
- **Storage Reserved Capacity** — commit to blob storage capacity for up to 3 years
- LRS vs GRS vs GZRS selection — match redundancy to actual DR requirements; avoid over-replication

### Networking
- Azure Front Door vs Application Gateway vs Load Balancer cost tradeoffs
- ExpressRoute vs VPN Gateway pricing model differences
- Egress data transfer costs — VNet peering is free; egress to internet is billed per GB

### Governance
- Management Group + Subscription tagging policy — enforce required tags via Azure Policy deny
- Cost allocation tags — chargeback/showback reporting by team, application, environment, cost-center
- Azure Policy to enforce allowed VM SKUs — prevent accidental high-cost SKU deployment

## Assessment Questions

- How do you model and predict cloud costs for your workload?
- How do you ensure resources are correctly sized for the workload?
- How do you use Azure pricing models (reservations, savings plans, spot) to reduce costs?
- How do you monitor and act on cost anomalies?
- How do you retire unused or underutilized resources?
- How do you allocate costs to teams or business units?
- How do you evaluate the cost impact of architectural decisions?

## Validation Checklist

- [ ] Azure Cost Management budgets configured per subscription with email alerts at 80% and 100%
- [ ] 100% of resources tagged with required tags (env, team, app, cost-center) via Azure Policy
- [ ] Azure Advisor cost recommendations reviewed and actioned monthly
- [ ] Azure Reservations or Savings Plans covering ≥70% of steady-state compute (VMs, AKS nodes, SQL)
- [ ] Azure Hybrid Benefit enabled for eligible Windows Server and SQL Server workloads
- [ ] Spot VMs or Azure Batch Spot nodes used for CI/CD, ML training, and dev/test workloads
- [ ] Blob Storage lifecycle policies active for all storage accounts with data older than 30 days
- [ ] Unused resources identified and decommissioned monthly: unattached disks, unused public IPs, empty resource groups, stopped VMs >7 days
- [ ] Dev/test VMs auto-shutdown scheduled outside business hours
- [ ] Cost allocation report generated monthly for chargeback/showback

## Response Shape

Cost visibility and tooling assessment → tagging compliance → reservation/savings plan coverage → rightsizing opportunities → hybrid benefit and spot adoption → storage lifecycle → idle resource inventory → prioritized savings actions

## Official Documentation

- https://learn.microsoft.com/azure/well-architected/cost-optimization/
- https://learn.microsoft.com/azure/cost-management-billing/

## Security Notes

Read-only advisory. Do not cancel Reservations, delete resources, or modify billing configurations without explicit approval and resource inventory confirmation.
