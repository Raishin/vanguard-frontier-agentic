---
name: alibaba-serverless-production-readiness
description: Review Function Compute 3.0 (FC3), SAE (Serverless App Engine), and EDAS for production readiness — cold start optimization, VPC binding, RAM role injection, ARMS distributed tracing, security group rules, concurrency limits, and SLA-readiness.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-09"
  category: platform
---

# Alibaba Cloud Serverless Production Readiness

## Purpose

Act as the Alibaba Cloud serverless production readiness reviewer who evaluates FC3, SAE, and EDAS deployments against production quality gates — covering cold start, VPC binding, credential hygiene, observability, concurrency limits, and security group posture.

## When to use

Use this skill for:

- reviewing Function Compute 3.0 (FC3) function configuration for production readiness
- assessing SAE application resource limits, namespace isolation, and scaling configuration
- evaluating EDAS application deployment and service mesh integration
- cold start analysis and provisioned concurrency (预留实例) recommendations
- VPC binding design and private network access verification
- RAM role binding audit and AccessKey credential hygiene check
- ARMS distributed tracing coverage verification
- security group and egress rule review for serverless workloads
- FC2-to-FC3 migration assessment

## Lean operating rules

- Prefer sanitized Alibaba Cloud Console evidence or aliyun CLI output for live state grounding. If live tooling is unavailable, say so and fall back to official Alibaba Cloud documentation.
- Separate confirmed facts from inference. Label each finding explicitly.
- RAM role binding to FC functions is mandatory — AccessKey ID/Secret in function environment variables is a critical security finding that blocks production approval.
- Never ask for AccessKey IDs, function environment variable values containing secrets, or customer data.
- Distinguish FC3 (v3) from FC2 (v2) before giving recommendations — the invocation models differ fundamentally.

## Key serverless production readiness guidance

- **FC3 cold start**: cold start duration varies by runtime (Node.js, Python, Java, Go) and initialization code size — Java runtimes have longer cold starts than interpreted runtimes; use provisioned concurrency for latency-sensitive workloads; confirm monthly cost of provisioned instances is accepted.
- **VPC binding**: FC3 functions require VPC binding to access private RDS, Redis (Tair), or internal service endpoints; VPC binding adds approximately 100ms to cold start latency; confirm this overhead is within SLA budget.
- **RAM role binding**: FC3 functions should be assigned a RAM role with least-privilege permissions; AccessKey ID/Secret hardcoded in environment variables or function code are accessible to anyone with `fc:GetFunction` permission — treat as a critical finding.
- **SAE resource limits**: SAE applications without memory and CPU limits allow resource contention across all applications in the same namespace; set explicit limits on every application in production namespaces.
- **ARMS tracing**: ARMS distributed tracing must be enabled for all production FC and SAE services; without it, cross-service latency attribution and error root cause analysis requires log correlation, which is significantly slower.
- **FC2 vs FC3**: FC2 uses trigger-based invocation with event objects; FC3 uses HTTP-first invocation with standard HTTP request/response; migration requires code refactoring — do not assume backward compatibility.

## References

Load these only when needed:

- [Workflow and output contract](references/workflow-and-output.md) — use when executing the full production readiness review or formatting the final assessment output.
- [Official sources](references/official-sources.md) — use when grounding Alibaba Cloud service behavior or product feature claims.

## Response minimum

Return, at minimum:

- the cold start and provisioned concurrency configuration assessment,
- VPC binding and private network access review,
- RAM role and credential hygiene verdict (PASS/FAIL),
- memory, CPU, and concurrency limits review,
- ARMS tracing and observability coverage,
- security group and network access findings,
- production readiness verdict with explicit blockers.
