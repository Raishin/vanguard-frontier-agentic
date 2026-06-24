---
name: sap-cap-architecture-review
description: Review SAP Cloud Application Programming Model (CAP) applications for CDS data modeling quality, service layer design, authorization correctness (@requires/@restrict), multitenancy architecture, draft handling, and test coverage. Use when assessing CAP Node.js or Java projects for architectural compliance, security posture, and clean service design. Does not touch live systems.
allowed-tools: Read Grep Glob WebSearch WebFetch
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-06-19"
  category: architecture
  lifecycle: experimental
---

# SAP CAP Architecture Review

## Purpose

Assess the architecture, security posture, and operational quality of SAP Cloud Application Programming Model (CAP) applications. Review CDS data model design for normalization, association correctness, and annotation quality. Assess the service layer for separation of concerns, projection completeness, and anti-pattern avoidance. Review authorization configuration (`@requires`, `@restrict`) for correctness and enforcement. Evaluate multitenancy design for tenant isolation, extensibility readiness, and MTX service wiring. Assess draft handling for consistency, activation logic, and side-effect completeness. Review test coverage for CAP-specific test patterns. Does not connect to or mutate any live CAP application, BTP service, or database.

## When to use

Use this skill when the user asks to:

- review CDS entity and service definitions for modeling quality, association correctness, projection anti-patterns, or annotation completeness,
- assess CAP authorization annotations (`@requires`, `@restrict`) for correctness, missing access controls, or role design gaps,
- evaluate multitenancy architecture: MTX service wiring, tenant onboarding flow, extensibility activation, `cds.requires.multitenancy` configuration,
- review draft handling design: draft-enabled entities, activation hooks, side effects, draft lock expiry, and cancel behavior,
- audit CAP test coverage: `cds.test` unit patterns, integration test setup, mock authentication usage, and CDS test environment teardown,
- identify CAP anti-patterns: direct database access bypassing the service layer, missing `@readonly` on projection fields, incorrect `up_` association exposure, or unguarded actions,
- assess CAP application readiness for SAP BTP production deployment: profile configuration, feature toggles, and `package.json` `cds` block correctness.

## When not to use

- When the request is about ABAP-side RAP business objects or ABAP Cloud compliance — use `sap-abap-cloud-rap-review`.
- When the request is about Integration Suite iFlow or API Management configuration — use `sap-integration-suite-review`.
- When the request requires live CAP application inspection, database query execution, or BTP service key access — this skill accepts only user-provided code artifacts, CDS model files, `package.json`, or written descriptions.
- When the request is about SAP Fiori / UI5 frontend code rather than the CAP backend service layer.

## Does not touch live systems

This skill operates on user-provided CDS source files, `package.json` configuration, `srv/` handler files, test file excerpts, MTX configuration, or written descriptions of the CAP application architecture. It does not connect to any BTP subaccount, CAP runtime, HANA database, or deployment pipeline. All live inspection is out of scope.

## Lean operating rules

- Classify findings before recommending. Every finding must be assigned to a review domain (CDS Modeling / Service Layer / Authorization / Multitenancy / Draft / Testing / Deployment Config) before remediation is proposed.
- Authorization is non-negotiable. A CAP service or entity with no `@requires` or `@restrict` annotation is accessible to any authenticated — or unauthenticated — caller depending on CAP runtime version. An unguarded service exposed to external callers is a `critical` finding.
- `@restrict` is preferred over `@requires` for entity-level access control. `@requires` on a service grants blanket access; `@restrict` with `grant`/`to`/`where` clauses gives fine-grained operation-level control. A service using only service-level `@requires` with no entity-level `@restrict` is a `medium` finding when business data sensitivity is high.
- Direct database access in service handlers bypasses CAP authorization enforcement. Any `SELECT` from `db.run(SELECT.from(...))` inside a service handler that circumvents the CAP service layer authorization is a `high` finding.
- Multitenancy requires explicit MTX wiring. A CAP application configured with `cds.requires.multitenancy: true` but missing `@sap/cds-mtxs` dependency, subscriber passcode configuration, or tenant upgrade hooks is a `high` finding.
- Draft handling activation must be complete. A draft-enabled entity missing an `activate` action implementation, `draftActivate` side-effect hook, or proper `BeforeSave` validation is a `high` finding for transactional correctness.
- Test isolation is required. CAP integration tests that use a shared `cds.test` instance without proper `beforeAll`/`afterAll` teardown, or that test against a live HANA target rather than in-memory SQLite, are `medium` findings for test reliability.
- Evidence from official SAP CAP documentation and user-provided artifacts takes precedence over inference.
- Load only the reference needed for the component in scope.

## Evidence rules

Label all claims with one of:

- `documentation-based` — grounded in official SAP CAP documentation (cap.cloud.sap, SAP Help Portal)
- `user-provided evidence` — CDS model files, service handler code, `package.json`, test files, or written descriptions provided by the user
- `context7-supplementary` — grounded in CAP framework docs fetched from Context7 (supplementary to official SAP CAP docs; applies for CDS modeling, authorization, multitenancy, and draft patterns)
- `inference` — derived reasoning not directly confirmed by official docs or user evidence

## Live-environment rules

**This skill does not touch live systems.** There is no CAP runtime API call, BTP service binding access, HANA database connection, or BTP cockpit access in this skill's execution path. Users must supply CDS source files, handler code excerpts, configuration files, or written descriptions of their CAP application for this skill to review.

## References

Load only when needed:

- [Workflow and output contract](references/workflow-and-output.md) — review domain taxonomy, severity classification, output format.
- [Safety checklist](references/safety-checklist.md) — non-negotiables, common CAP mistakes, when to push back.
- [Official sources](references/official-sources.md) — SAP CAP docs, CDS language reference, authorization guides, multitenancy, draft handling.
- [Context7 framework docs](references/context7-framework-docs.md) — CAP authorization patterns, CDS service modeling, multitenancy, and draft handling (supplementary; strongly applies for this skill).

## Response minimum

Return, at minimum:

- **Problem classification**: review domain(s) in scope and specific finding(s) per domain.
- **Evidence used**: documentation-based / user-provided evidence / context7-supplementary / inference.
- **Risk level**: critical (security or data integrity risk) / high (correctness or operational reliability risk) / medium (governance or maintainability gap) / low (best practice deviation).
- **Recommended action**: specific remediation per finding (add `@restrict` annotation, wire MTX service, implement `draftActivate` hook, add test teardown, etc.).
- **Refusal / escalation triggers**: if live CAP runtime access, BTP service binding, or database inspection is required to complete the review, state that clearly and do not proceed.
- **Business impact**: security exposure, data integrity risk, multitenancy isolation failure, or test reliability gap.
- **Next verification step**: validate recommended changes against the current CAP runtime version and BTP deployment target before deploying.
