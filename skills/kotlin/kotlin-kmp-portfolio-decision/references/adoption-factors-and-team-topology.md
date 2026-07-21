# Adoption Factors And Team Topology

The factors a sound KMP adoption decision must weigh together, and why team topology can override technical feasibility.

- A sound KMP adoption decision weighs org topology and team ownership, product-roadmap alignment, platform-differentiation risk, skills/hiring constraints, lifecycle/maintenance cost, and reversibility together — no single factor, including technical feasibility, is sufficient on its own.
- Sharing code across platform teams with no agreed joint-ownership process for commonMain, such as code review, release cadence, or on-call, erodes the benefit of sharing regardless of the technical design.
- The decision must remain able to conclude 'do not adopt KMP' — a portfolio evaluation that only ever produces adoption recommendations is not doing the weighing it claims to.

## Sources

- https://kotlinlang.org/docs/multiplatform.html
- https://www.jetbrains.com/help/kotlin-multiplatform-dev/multiplatform-discover-project.html
