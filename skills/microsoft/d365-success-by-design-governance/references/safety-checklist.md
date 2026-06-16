# Safety checklist

Use this reference before any recommendation involving go-live approval, phase gate waiver, production deployment authorization, or compliance-impacting implementation decisions in Dynamics 365 projects following the Success by Design framework.

## Non-negotiables

- Never ask users to paste credentials, tenant IDs, environment URLs, client secrets, certificates, or customer personally identifiable information into chat.
- Use documented project artifacts or sanitized user-provided evidence for current-state claims; otherwise use documentation and label the evidence level.
- Do not invent phase gate artifact states, SBR finding counts, fit-gap analysis results, customization counts, or project timelines.
- Require explicit human approval before recommending any production deployment, go-live authorization, or phase gate waiver.
- Use current official Microsoft Learn documentation for Success by Design framework behavior and FastTrack guidance.
- Keep recommendations least-change, reversible, and scoped to the domain in question.
- Production deployment and go/no-go decisions are live-guard gated. Always escalate to the project sponsor and implementation lead with environment access before execution.

## Stress checks

- What Success by Design phase gates have been skipped or have no documented artifacts?
- What Solution Blueprint Review findings remain open or without a triggered Implementation Review?
- What customizations or extensions lack a documented fit-gap justification, cost estimate, or roadmap alignment review?
- What go-live readiness checklist items are incomplete or missing stakeholder sign-off?
- What rollback plan exists if the production go-live fails within the cutover window?
- What audit evidence is missing that the project steering committee or FastTrack team would expect?
- What hypercare and production support plan is in place for post-go-live stabilization?

## Evidence labels

Use `documented artifact`, `user-provided evidence`, `documentation-based`, or `inference`. Documentation alone never proves the user's actual project phase, SBR completion status, fit-gap discipline, or go-live readiness.

## Live-guard gate

The following actions require explicit human confirmation and are out of scope for automated execution:

- Authorizing production go-live or issuing a formal go/no-go decision
- Waiving a mandatory Success by Design phase gate or SBR workshop
- Approving a customization or extension without documented fit-gap analysis and architectural review
- Bypassing a FastTrack Implementation Review finding without documented mitigation
- Authorizing production environment provisioning or deployable package promotion to production
- Signing off on UAT, performance testing, or data migration dry-run results on behalf of business stakeholders
