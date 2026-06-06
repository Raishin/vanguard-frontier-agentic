# RDS and Aurora Performance Evidence Guide

Use this reference for Amazon RDS/Aurora latency, connection exhaustion, slow SQL, lock waits, replica lag, failover behavior, storage pressure, CPU/I/O saturation, Performance Insights, Enhanced Monitoring, and database capacity investigations.

## What people get wrong

The lazy story is:

> Resize the database; CPU or connections are high.

Wrong. Database incidents often come from query plans, locks, connection pools, I/O, replication, failover, storage, or application behavior. Resizing can hide root cause and increase cost without fixing the workload.

Common bad assumptions:

- High CPU is the root cause.
- More connections improves throughput.
- Replica lag means the reader is too small.
- Performance Insights top SQL is always the culprit.
- Failover success means no user impact.
- Storage autoscaling removes storage risk.

## RDS/Aurora failure modes

- Connection pool storms exhaust DB connections or memory.
- Lock waits/deadlocks make healthy CPU look misleading.
- Query plan regression or missing index increases DB load and I/O.
- Aurora replica lag, cluster cache behavior, or writer/read endpoint routing causes stale or slow reads.
- Storage, burst balance, IOPS, temp space, or transaction logs saturate before CPU.
- Maintenance, parameter changes, failover, or backups correlate with performance but are not inspected.

## Minimum safe workflow

1. Identify engine, deployment topology, instance classes, writer/readers, timeframe, symptoms, and customer impact.
2. Build evidence timeline from CloudWatch, Performance Insights, Enhanced Monitoring, RDS events, logs, and deployment changes.
3. Separate symptoms from hypotheses: CPU, connections, waits, locks, I/O, storage, SQL, replication, failover, and app traffic.
4. Inspect top SQL/waits, connection pool settings, recent schema/parameter changes, and slow query/error logs where available.
5. Recommend low-risk mitigations first: query/index review, pool tuning, throttling callers, read routing, alarm thresholds, or controlled scaling.
6. Require approval for failover, reboot, parameter apply, scale, index creation, or query kill actions.
7. State what cannot be proven without database logs, PI data, or workload context.

## Verification targets

- CloudWatch metrics: CPUUtilization, DatabaseConnections, FreeableMemory, Read/WriteIOPS, Read/WriteLatency, DiskQueueDepth, FreeStorageSpace, ReplicaLag, Deadlocks
- Performance Insights DB load, waits, top SQL, dimensions, and retention/Advanced mode availability
- Enhanced Monitoring OS process/thread evidence and RDS/Aurora events
- slow query/error logs, lock/deadlock evidence, query plans, indexes, transaction age, and connection pool config
- cluster topology, endpoints, failover history, parameter groups, maintenance window, backups, and storage settings
- recent deployments, migrations, traffic changes, batch jobs, and application error rates

## When to push back

Push back if the user asks to:

- resize before inspecting waits, SQL, and connections
- kill sessions or fail over without impact/rollback approval
- ignore application connection pool behavior
- create indexes in production without plan and lock/space analysis
- call top SQL root cause without workload and wait context
- treat replica lag as purely instance-size problem
