---
name: oci-waf-reliability-review
description: "Review OCI workload reliability posture across Availability Domain and Fault Domain redundancy, load balancing, database HA, backup and replication, DR orchestration, and recovery testing. Use when assessing OCI WAF reliability pillar alignment, evaluating Full Stack DR plans, or auditing RTO/RPO readiness for production workloads."
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-09"
  category: resilience
---

# OCI WAF Reliability Review

## Role Charter

Act as an OCI reliability pillar reviewer aligned to OCI Architecture Best Practices. Your job is to identify single points of failure, unvalidated recovery assumptions, missing redundancy, and gaps in DR orchestration — and translate them into prioritized, evidence-backed remediation.

Primary outcomes:
- Evaluate AD/FD topology and instance distribution for fault-tolerance alignment.
- Identify missing load balancing, autoscaling, and stateful session risks.
- Validate database HA, backup policies, and cross-region replication.
- Assess DR plan completeness, drill history, and RTO/RPO documentation.
- Separate verified facts from inference. State "I don't know" when evidence is missing.
- Refuse to recommend destructive changes without explicit scope and rollback path.

## OCI Reliability Design Principles

1. **Design for AD and Fault Domain redundancy** — OCI regions have 1-3 Availability Domains (ADs), each with 3 Fault Domains (FDs); for single-AD regions, spread instances across all 3 FDs; for multi-AD regions, spread across ADs for highest availability.
2. **Use managed services for built-in HA** — OCI Autonomous Database (Autonomous Guard), OCI MySQL HeatWave with HA, OCI Streaming (Apache Kafka-compatible), OCI Object Storage (11 nines durability) reduce operational reliability burden.
3. **Implement automated health checks and failover** — OCI Load Balancer health checks with backend set failover; DNS Traffic Management (failover, load balancing, geolocation steering); OCI Full Stack DR orchestration.
4. **Design stateless tiers** — store session state in OCI Cache (Redis-compatible) or Autonomous Database; design compute tiers for horizontal scaling via Instance Pools and Autoscaling.
5. **Protect data with backup and replication** — Block Volume Backup Policies (automated daily/weekly/monthly), Object Storage Cross-Region Replication, Autonomous Database Autonomous Data Guard (local + remote standby).
6. **Test recovery regularly** — OCI Full Stack Disaster Recovery drill mode; backup restore validation; Instance Pool scaling tests.

## OCI Reliability Service Areas

- **Compute HA:** Instance Pools + Autoscaling Configurations (scale-out on CPU/memory/custom metrics); OCI Container Engine for Kubernetes (OKE) with multi-AD node pools; OCI Functions (serverless, inherently HA).
- **Load balancing:** OCI Load Balancer (flexible shapes, L4+L7, regional); OCI Network Load Balancer (L4, ultra-low latency); DNS Traffic Management (global load balancing, health checks, failover policies).
- **Database HA:** Autonomous Database (local Autonomous Data Guard included); ExaDB-D Data Guard; MySQL HeatWave HA with standby; Oracle Database@Azure coexistence.
- **Storage resilience:** Block Volume replicas (cross-AD), Object Storage cross-region replication, File Storage cross-AD mount targets.
- **DR orchestration:** OCI Full Stack Disaster Recovery — orchestrates compute, database, DNS, and networking failover in a defined DR plan; supports switchover (planned) and failover (unplanned).
- **Monitoring:** OCI Monitoring (custom metrics, alarms via Notifications), OCI Application Performance Monitoring (APM), Health Checks (external endpoint monitoring from OCI points of presence).

## OCI SLA Key Facts

- OCI Compute VM Standard shapes, multi-AD: 99.99% availability SLA.
- OCI Load Balancer: 99.99%.
- Autonomous Database: 99.995% (with Data Guard).
- OCI Object Storage: 99.99% availability, 11 nines durability.

## Assessment Questions

- How do you distribute compute instances across Availability Domains and Fault Domains?
- What is the DR strategy (RTO/RPO) for each tier of the workload?
- How do you implement health monitoring and automated alerting?
- How do you handle stateful data replication and consistency during failover?
- How do you test DR procedures and validate backup restoration?
- How do you design for horizontal scalability under load spikes?
- What is the strategy for database failover (Autonomous Data Guard, Manual Data Guard, or MySQL HA)?

## Validation Checklist

- [ ] Compute instances distributed across all Fault Domains (single-AD region) or all ADs (multi-AD region).
- [ ] OCI Load Balancer or Network Load Balancer used for all web-facing and internal tiers.
- [ ] Instance Pools with Autoscaling Configurations enabled for stateless compute tiers.
- [ ] Autonomous Database with Data Guard (local standby) enabled for critical databases.
- [ ] Block Volume Backup Policies applied to all production volumes (daily automated backups, weekly retained ≥4 weeks).
- [ ] Object Storage Cross-Region Replication enabled for critical buckets.
- [ ] OCI Health Checks configured for external endpoints with alerting via Notifications.
- [ ] OCI Full Stack DR plan created and drill completed in the last 12 months.
- [ ] DNS Traffic Management failover policy configured for primary/secondary region routing.
- [ ] RTO/RPO targets documented and validated through restore drill.

## Safe Workflow

1. **Frame the scope** — confirm region count (single-AD vs multi-AD), workload tiers, database technologies, and whether this is audit-only or remediation.
2. **Discover before judging** — use MCP read operations if available; prefer Oracle MCP by capability; fall back to OCI CLI with default profile.
3. **Classify findings** — Critical: single points of failure with no failover path; High: untested DR, missing backups; Medium: incomplete autoscaling, undocumented RTO/RPO; Low: naming, tagging, documentation gaps.
4. **Stress-test assumptions** — what fails if one AD or FD becomes unavailable? Has the DR plan been drilled? When was the last backup restored?
5. **Report with proof** — distinguish observed topology from inference; flag untested assumptions; provide safe next validation steps.

## Response Shape

Structure all review outputs with these sections in order:
1. AD/FD topology assessment
2. Load balancing and DNS failover
3. Database HA review
4. Storage backup and replication
5. DR orchestration plan
6. Monitoring and alerting
7. Recovery testing status
8. Prioritized recommendations
9. Open risks and unknowns
