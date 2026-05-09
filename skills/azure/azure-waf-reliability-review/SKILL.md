---
name: azure-waf-reliability-review
description: "Review Azure workload reliability against the Well-Architected Framework Reliability pillar: availability targets, AZ/region topology, health monitoring, data resilience, deployment safety, and chaos testing."
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-09"
  category: resilience
---

# Azure WAF Reliability Review

The Azure Well-Architected Framework Reliability pillar covers designing, building, and operating workloads that are resilient to failures, meet availability targets, and recover gracefully from disruptions.

## 5 Reliability Design Principles

1. **Design for business requirements** — translate business requirements into measurable reliability targets; define RTO, RPO, availability SLOs
2. **Design for resilience** — ensure the workload survives fault conditions without complete outage; design for partial failure
3. **Design for recovery** — assume failures will happen; implement automated recovery; test recovery procedures regularly
4. **Design for operations** — build operations into the design; implement health modeling; use automated deployments
5. **Keep it simple** — avoid design complexity that introduces reliability risks; use managed services to reduce operational burden

## Azure Reliability Service Areas

### Availability Zones
- Zone-redundant services: ZRS Storage, Zone-redundant Azure SQL, AKS across zones
- Zonal deployments: pin to specific zone for latency-sensitive workloads
- Regional services: single-zone, not HA — use only for non-critical or dev/test workloads

### Fault Tolerance
- Azure Load Balancer (L4) — regional traffic distribution
- Azure Application Gateway (L7) — zone-redundant HTTP/S load balancing with WAF
- Azure Traffic Manager (DNS) — cross-region failover and weighted routing
- Azure Front Door — global anycast with automatic failover

### Scalability
- VMSS (Virtual Machine Scale Sets) — VM-based horizontal scaling
- Azure Kubernetes Service (KEDA, HPA) — pod and node autoscaling
- Azure App Service autoscale — rule-based and metric-based scaling
- Azure Functions consumption plan — event-driven serverless scaling

### Data Resilience
- Azure SQL Business Critical — 2 readable secondaries, zone-redundant by default
- Cosmos DB — multi-region writes, configurable consistency (strong/bounded-staleness)
- Azure Storage GRS/GZRS — geo-redundant and geo-zone-redundant replication
- Azure Backup — VM, SQL, and file share backup with retention policies
- Azure Site Recovery (ASR) — VM replication and orchestrated failover

### Health Modeling
- Azure Monitor — metrics, logs, and alerting platform
- Application Insights — distributed tracing, availability tests, failure analysis
- Azure Service Health alerts — planned maintenance, outage notifications, advisories
- Resource Health API — per-resource health signal for automated remediation

### Chaos Engineering
- Azure Chaos Studio — inject faults: VM availability zone failure, AKS pod failure, network latency, dependency unavailability

### Deployment Safety
- App Service deployment slots — blue/green with slot swap and auto-swap
- Azure Deployment Environments — consistent, governed environment templates
- Traffic Manager weighted routing — canary deployment with percentage-based traffic split

## Azure SLA Key Facts

- Availability Zones: **99.99%** SLA for VMs when using ≥2 VMs across ≥2 AZs
- Azure SQL Business Critical + zones: **99.995%**
- Cosmos DB multi-region writes: **99.999%**
- Single VM with Premium SSD: **99.9%** — NOT suitable for production HA
- Traffic Manager: **99.99%**

## Assessment Questions

- What are the availability and reliability targets for the workload?
- How do you ensure the workload can operate during a zone or region outage?
- How do you implement health monitoring and proactive alerting?
- How do you manage dependencies and limit blast radius of failures?
- How do you test resilience and validate recovery procedures?
- How do you design for graceful degradation under load or partial failure?
- How do you implement safe deployment practices to minimize rollback risk?

## Validation Checklist

- [ ] SLOs defined with measurable SLIs monitored via Azure Monitor
- [ ] All stateful services (VMs, databases) deployed across ≥2 Availability Zones
- [ ] Azure Service Health alerts configured for all services in use
- [ ] Autoscaling enabled for stateless compute (VMSS, App Service, AKS node pools)
- [ ] Azure SQL or Cosmos DB multi-region replication configured for mission-critical data
- [ ] Azure Backup or ASR configured for VMs, SQL databases, and critical storage
- [ ] Deployment slots or Traffic Manager canary routing used for production deployments
- [ ] Azure Chaos Studio experiments conducted to validate AZ failure recovery
- [ ] Circuit breaker and retry patterns implemented for external service calls
- [ ] RTO/RPO targets documented and validated via DR drill in the last 12 months

## Response Shape

Availability targets review → AZ/region topology → health monitoring → failure tolerance → data resilience → deployment safety → chaos testing status → recommendations → open risks

## Official Documentation

- https://learn.microsoft.com/azure/well-architected/reliability/
- https://learn.microsoft.com/azure/reliability/availability-zones-overview

## Security Notes

Read-only advisory. Do not modify autoscaling policies, backup schedules, or Azure Site Recovery configurations without explicit approval.
