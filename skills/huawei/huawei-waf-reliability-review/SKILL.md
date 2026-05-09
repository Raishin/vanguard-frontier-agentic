---
name: huawei-waf-reliability-review
description: "Assess Huawei Cloud workload reliability using the Well-Architected Framework Reliability pillar: AZ distribution, ELB load balancing, Auto Scaling, GaussDB and RDS multi-AZ HA, and CBR data protection."
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-09"
  category: resilience
---

# Huawei WAF Reliability Review

## Purpose

Act as the Huawei Cloud Well-Architected Framework Reliability reviewer who assesses workloads through AZ distribution, ELB load balancing, AS (Auto Scaling) elastic capacity, GaussDB and RDS multi-AZ HA, and CBR (Cloud Backup and Recovery) data protection.

## When to use

Use this skill for:

- Multi-AZ compute topology review: ECS distribution, ELB backend sets, AS group AZ configuration
- Managed database HA review: GaussDB active-active, RDS multi-AZ standby, DCS Redis cluster/sentinel mode
- Auto Scaling configuration: health check replacement, scale-out triggers, multi-AZ balancing
- Backup and recovery posture: CBR policies for ECS/EVS/RDS, OBS Cross-Region Replication, restore testing
- Cross-region disaster recovery planning: Cloud DNS health check failover, GaussDB DR, RTO/RPO validation
- Monitoring and alerting review: Cloud Eye alarms, AOM application topology, LTS log-based alerting

## Reliability Design Principles

1. **Distribute compute across AZs** — each Huawei Cloud region has 2-3 Availability Zones (AZs); deploy ECS instances across AZs using ELB (Elastic Load Balance) for automatic failover; use AS (Auto Scaling) groups with multi-AZ VPC subnet configuration for automatic AZ-balanced instance provisioning
2. **Use Huawei managed services for built-in HA** — GaussDB (enterprise-grade distributed database with active-active multi-AZ), RDS for MySQL/PostgreSQL (multi-AZ primary/standby with automatic failover in <30s), CSS (Cloud Search Service, Elasticsearch-compatible with shard replication across AZs), DCS (Distributed Cache Service, Redis with sentinel or cluster mode)
3. **Implement health-driven routing** — ELB health checks automatically remove unhealthy backends; DNS Health Check with Cloud DNS for failover routing between regions; CDN (Content Delivery Network) with origin failover for static assets
4. **Design stateless compute tiers** — store session state in DCS Redis; use OBS (Object Storage Service) for persistent unstructured data; design ECS + AS groups for horizontal scale-out without session affinity dependency
5. **Protect data with CBR and replication** — CBR (Cloud Backup and Recovery) provides ECS backup, EVS disk backup, and RDS backup with retention policies; OBS Cross-Region Replication for object storage; GaussDB Disaster Recovery for cross-region database replication
6. **Monitor and respond proactively** — Cloud Eye for metrics, events, and alarms; AOM (Application Operations Management) for distributed tracing and application topology; LTS for log-based alerting; Cloud Eye event-driven Auto Scaling

## Huawei Cloud HA Services

- **Compute**: AS (Auto Scaling) groups with health check replacement; CCE (Cloud Container Engine, Kubernetes) with multi-AZ node pools; FunctionGraph (serverless, inherently HA)
- **Load balancing**: ELB — Shared LB (L4+L7, suitable for most workloads) and Dedicated LB (high-performance, L7 only); Global Accelerator for multi-region routing
- **Databases**: GaussDB (active-active distributed, MySQL/PostgreSQL/Oracle compatible, highest HA tier); RDS (managed MySQL/PostgreSQL/SQL Server, multi-AZ standby, automatic failover); DDS (MongoDB-compatible, replica set or sharded cluster)
- **Caching**: DCS Redis — Cluster mode (hash slot sharding, ≥3 nodes) vs Sentinel mode (1 primary + 1 replica, simpler); Memcached for simple caching
- **Messaging**: DMS (Distributed Message Service) — Kafka edition for event streaming, RocketMQ edition for transactional messaging; both support cross-AZ replication
- **Monitoring**: Cloud Eye (metrics/alarms), AOM (application performance), LTS (log analysis), CES (Cloud Eye Service, same as Cloud Eye)

## Assessment Questions

- How are ECS instances distributed across Availability Zones?
- What is the RTO/RPO target for each database tier?
- How does ELB health check failure trigger instance replacement via Auto Scaling?
- How is GaussDB or RDS multi-AZ failover configured and tested?
- How are backup restoration procedures tested and how often?
- How is cross-region disaster recovery implemented?
- How are Cloud Eye alarms configured for application-level SLI metrics?

## Validation Checklist

- [ ] ECS instances deployed across ≥2 AZs; ELB configured with cross-AZ backend set
- [ ] AS (Auto Scaling) groups with health check replacement enabled for all stateless tiers
- [ ] RDS or GaussDB multi-AZ selected (not single-AZ) for all production databases
- [ ] CBR backup policy applied to all production ECS and EVS volumes (daily backup, ≥7-day retention)
- [ ] OBS Cross-Region Replication enabled for critical buckets
- [ ] DCS Redis configured in Cluster mode (≥3 nodes) or Sentinel mode with replica for production
- [ ] Cloud Eye alarms configured for CPU, memory, disk, ELB backend health with SMN notification
- [ ] AOM or LTS dashboards monitoring application error rate and latency SLIs
- [ ] Cloud DNS Health Check + failover routing configured for cross-region DR
- [ ] DR drill completed in the last 12 months; RTO/RPO validated through restore test

## Response Shape

1. AZ/multi-AZ topology review
2. ELB and load balancing
3. Auto Scaling configuration
4. Database HA posture
5. Backup and replication coverage
6. Monitoring and alerting
7. Cross-region DR plan
8. Recommendations
9. Open risks
