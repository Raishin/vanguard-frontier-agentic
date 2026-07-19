# Decision Model and Cost Inputs

> Scope note. This reference defines the decision framework (modernize / rehost / replatform / retire / renew) and the cost-input taxonomy this agent requires before it will produce a dollar-denominated figure. It contains no vendor pricing, no subscription tiers, and no customer/tenant data — those must never be hardcoded here or anywhere in this bundle (CLAUDE.md operating stance: no secrets, credentials, tokens, tenant IDs, or customer data). Every dollar figure in an actual review comes from the user, labeled with its source and the date supplied.

## Why this decision matters

"Exit the app server" is a portfolio bet with a multi-year payback horizon, made once per component and expensive to reverse. Getting it wrong in either direction is costly: replatforming a component that would have been cheaper to retire wastes engineering effort; renewing a component that should be retired locks in license and support spend with no offsetting business value. The decision needs to be made per component, on explicit evidence, with the confidence level stated — not asserted as a single estate-wide verdict.

## The four exit options (plus the null option)

| Option | Definition | Typical signal |
|---|---|---|
| Retire | Decommission the component; its function is dead, duplicated elsewhere, or no longer needed | No active users/traffic evidence supplied, or the business function has migrated to a system of record elsewhere |
| Rehost | Move the same binary/runtime to different infrastructure (e.g. on-prem WebLogic to a cloud VM) without changing the app-server product or code | Low jakarta-namespace debt, low EJB/SOAP surface, but current infrastructure cost or datacenter exit is the driver |
| Replatform | Move off the proprietary app server to a different runtime (e.g. WebLogic to an open-source Jakarta EE-compatible runtime, or to a servlet container plus embedded framework), with code changes bounded by the namespace/API migration | Meaningful jakarta-namespace debt and/or EJB/JAX-WS/SOAP surface reported by the specialist inputs, but the component is actively maintained and worth the investment |
| Modernize in place | Migrate namespaces/APIs (javax→jakarta, EJB→CDI, JAX-WS→REST) while staying on the same commercial platform family, typically to reach a supported/current version | Vendor lifecycle tier is currently acceptable (per `vendor-lifecycle-sources.md`) but technical debt blocks a future move; the platform relationship itself is not the problem |
| Renew (null option) | Continue on the current platform and license/support tier as-is | Low technical debt, acceptable lifecycle tier, and/or missing evidence to justify any of the above — renew is also the correct answer when the evidence does not support a change |

Renew is a legitimate, explicitly-scored outcome — never treat "no clear signal" as a de facto license to modernize; absence of a driver is itself the finding.

## Required specialist inputs (consumed, not re-derived)

This agent does not perform the underlying technical analysis. Each per-component decision requires the following as INPUT evidence, and a missing input caps that component's decision confidence at low:

- **JDK lifecycle/support-boundary exposure** — from `java-jdk-lifecycle-and-upgrade-agent`'s output (vendor, version, support-tier finding, upgrade blockers).
- **Jakarta namespace debt** — the scope and complexity of `javax.*` → `jakarta.*` migration for the component (namespace specialist finding).
- **EJB/JAX-WS/SOAP inventory** — the count, coupling, and migration complexity of EJB, JAX-WS, and SOAP surface area (inventory specialist finding).
- **Container-readiness** — whether the component's runtime dependencies (filesystem state, clustering/session assumptions, JNDI/JMS wiring, native libraries) permit containerization (container-readiness specialist finding).

If a component's evidence set is incomplete, say explicitly which specialist finding is missing and what decision confidence that caps — do not fill the gap with an assumption presented as evidence.

## Required cost inputs (user-supplied only — never invented)

A payback period, ROI, or any dollar-denominated recommendation requires the user to supply, at minimum:

- **Current run-rate**: today's licence, support/subscription, and infrastructure cost for the component (annualized).
- **Target-state run-rate**: the annualized cost of the recommended end state (open-source runtime infra cost, new subscription tier if any, cloud infra cost).
- **One-time transition cost**: the migration/replatform labor estimate (person-time or contracted cost), tooling, and any parallel-run cost.
- **Hurdle/discount rate or payback threshold** (if the organization uses one) — optional but should be requested; without it, report simple (undiscounted) payback period only and say so.
- **Indirect/un-quantified costs the user is aware of** (retraining, downtime risk, tooling license changes) — collect these as named items even if the user cannot price them; never price them on the user's behalf.

None of these figures may be estimated, benchmarked from industry averages, or backfilled from vendor list pricing. If any of the first three is missing, the correct output is `insufficient-evidence` for the payback/ROI figure specifically — the per-component modernize/rehost/replatform/retire/renew decision can still be made on the specialist technical evidence alone, but it will carry no dollar figure and should say so plainly.

## Confidence scoring

Score each per-component decision's confidence from its evidence mix, not from how confident the recommendation "feels":

| Confidence | Criteria |
|---|---|
| High | All four specialist inputs present and confirmed (source provided); all required cost inputs supplied for a payback figure, or the decision does not depend on cost (e.g. clear retire signal on dead-component evidence). |
| Medium | At least one specialist input is inference-level (partial source) rather than confirmed, or cost inputs are partial (e.g. run-rate supplied but transition cost is a rough estimate the user flagged as such). |
| Low | A required specialist input is missing (assumption-level or absent) for the component, or the recommendation rests on cost figures the user has not supplied and this agent has therefore not priced. |

A component's overall confidence is the lowest confidence of any input it depends on — never averaged upward.

## Known uncertainty

- The boundary between "replatform" and "modernize in place" is a matter of degree (how much of the app-server-specific API surface remains after the change); when the specialist evidence is ambiguous, name both options and their relative cost/risk rather than forcing a single label.
- Indirect costs (retraining, organizational change cost, opportunity cost of delayed feature work) are real but structurally hard to price without organization-specific data the user must supply; this reference deliberately does not offer an estimation heuristic for them, to avoid smuggling an assumption in as a number.
- Wave sizing (how many components per wave, how much parallel capacity) is organization-specific; this reference gives sequencing principles (see `workflow-and-output.md`) rather than a fixed cadence.
