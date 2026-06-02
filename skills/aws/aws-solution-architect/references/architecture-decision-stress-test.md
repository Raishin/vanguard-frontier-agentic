# Architecture Decision Stress-Test Guide

Use this reference for cross-domain AWS solution architecture decisions, ADRs, tradeoff reviews, target architectures, and production-readiness stress testing.

## What people get wrong

The lazy story is:

> Pick the AWS best-practice pattern and fill in the diagram.

Wrong. Architecture is tradeoff management under constraints. A design is not good until assumptions, failure modes, owners, validation paths, and rollback/migration paths survive scrutiny.

Common bad assumptions:

- Serverless, containers, or managed services are always the right default.
- A Well-Architected answer is enough without evidence.
- Security, reliability, cost, and delivery tradeoffs can be optimized independently.
- Multi-Region is always more resilient.
- Architecture diagrams show data boundaries, quotas, or operational ownership.
- Future flexibility justifies complexity now.

## Architecture-specific failure modes

- Missing workload context: users, data sensitivity, SLOs, RTO/RPO, traffic, regulatory needs, and team capability.
- Hidden coupling through IAM roles, KMS keys, DNS, event schemas, shared VPCs, databases, queues, and CI/CD.
- Control-plane dependencies are mistaken for data-plane resilience.
- Operational model lacks observability, runbooks, cost accountability, support plan, or ownership.
- Migration path lacks strangler steps, rollback point, data cutover, or coexistence design.
- Design optimizes a favorite service rather than stated constraints.

## Minimum safe workflow

1. Restate problem, constraints, non-goals, assumptions, and decision drivers.
2. Identify workload domains: identity, network, compute, data, integration, observability, security, resilience, cost, and delivery.
3. Compare at least two viable options when the decision is material.
4. Stress-test each option against failure modes, data boundaries, quotas, operational ownership, migration, rollback, and cost.
5. Choose the minimum viable architecture that satisfies constraints; call out rejected complexity.
6. Produce an ADR-style decision: context, options, decision, consequences, risks, validation plan, and revisit triggers.
7. Route single-domain deep work to narrower AWS skills when needed.

## Verification targets

- workload context: users, traffic, data classification, compliance, SLO/RTO/RPO, cost target, and team capability
- architecture diagram plus trust/data boundaries, network paths, identity paths, and event/data flows
- dependency inventory for managed services, third parties, DNS, KMS, shared accounts/VPCs, and pipelines
- Well-Architected pillar impacts, risks, and explicit tradeoffs
- validation plan: load test, threat model, DR test, cost model, proof of concept, migration rehearsal, and rollback criteria
- implementation roadmap with owners, milestones, guardrails, and decision revisit date

## When to push back

Push back if the user asks to:

- rubber-stamp a preferred architecture without alternatives
- add multi-Region, Kubernetes, microservices, or AI because it sounds modern
- ignore data classification, IAM, cost, or recovery constraints
- produce a diagram without operating model and validation plan
- treat official best practices as proof that this workload is ready
- design beyond team capability or budget without naming the risk
