# Official sources

Use this reference only when you need source grounding for Dynamics 365 Success by Design, FastTrack implementation guidance, or fit-gap analysis behavior.

## Microsoft Learn documentation

Use these as starting points, not as proof of the user's live project state:

- https://learn.microsoft.com/dynamics365/guidance/implementation-guide/success-by-design
- https://learn.microsoft.com/dynamics365/guidance/implementation-guide/overview
- https://learn.microsoft.com/dynamics365/guidance/implementation-guide/process-focused-solution-fit-to-standard-fit-gap-analysis
- https://learn.microsoft.com/dynamics365/guidance/implementation-portal/conduct-solution-blueprint-review-workshop
- https://learn.microsoft.com/dynamics365/guidance/implementation-guide/prepare-to-go-live
- https://learn.microsoft.com/dynamics365/guidance/implementation-guide/prepare-go-live-checklist
- https://learn.microsoft.com/dynamics365/guidance/fasttrack/go-live-workshops
- https://learn.microsoft.com/dynamics365/guidance/fasttrack/implementation-workshops
- https://learn.microsoft.com/training/paths/use-success-design/
- https://learn.microsoft.com/dynamics365/guidance/implementation-guide/project-governance-conclusion

## Grounding rule

Official documentation explains Success by Design framework behavior and FastTrack program guidance. It does not prove the user's actual project phase, artifact completeness, SBR status, or go-live readiness posture. Prefer documented project artifacts (SBR outputs, fit-gap logs, implementation review findings, go-live checklist sign-offs) over inference.

## Service facts (verified 2026-06-16)

Success by Design phases:
- The Success by Design framework organizes the Dynamics 365 implementation lifecycle into five methodology-agnostic phases: **Strategize**, **Initiate**, **Implement**, **Prepare**, and **Operate**.
- **Strategize**: Discovery mode — gather and validate business requirements, finalize the high-level solution approach, define environment strategy and organizational strategy.
- **Initiate**: Define all in-scope workstreams, update the project plan. The **Solution Blueprint Review (SBR)** is the mandatory starting review conducted at or before the end of this phase.
- **Implement**: Build the solution per the agreed design and scope. **Implementation Reviews** address findings from the SBR; they cover data model, security, integration, ALM, and testing strategy.
- **Prepare**: Solution built and tested. Final UAT, training, cutover plan, mock go-live, support model, deployment runbook. The **Go-live Readiness Review** is the mandatory final gate.
- **Operate**: Solution is live. Goal is stabilization and handoff toward the next phase of digital transformation.

Solution Blueprint Review (SBR):
- The SBR is mandatory and serves as the starting point for all Success by Design reviews.
- It covers: program strategy, test strategy, business process strategy, application strategy, data strategy, integration strategy, intelligence strategy, security strategy, ALM strategy, and environment and capacity strategy.
- Findings from the SBR trigger Implementation Reviews for deeper dives into specific risk areas.

Fit-to-standard and fit-gap:
- **Fit-to-standard analysis**: Compare current processes against standard Dynamics 365 processes in the Business Process Catalog. Adopt standard configuration wherever possible.
- **Fit-gap analysis**: Identify requirements that standard processes cannot fulfill. Evaluate complexity, cost, maintenance burden, and product roadmap alignment before approving any customization or extension.
- Customizations that recreate legacy system behavior without justification are the leading cause of implementation cost overruns and upgrade risk.

Go-live readiness review:
- The Go-live Readiness Review is mandatory and conducted during the Prepare phase.
- Required evidence: SIT completion and sign-off, UAT completion and sign-off, performance testing, data migration plan and dry-run results, cutover plan with go/no-go criteria, user training completion, license count confirmation, production support plan.
- The review produces a go/no-go decision that must be documented and signed by project stakeholders and the implementation lead.

Review implications:
- Do not approve go-live from intent alone. Require SBR outputs, fit-gap analysis, implementation review findings with resolutions, go-live checklist completion, and explicit business owner sign-off.
- Documentation cannot prove the user's actual project phase, artifact state, or implementation review history.
