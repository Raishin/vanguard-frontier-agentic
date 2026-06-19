# Safety checklist

Use this reference before any recommendation involving production agent publishing, connector grant expansions, DLP policy changes, ALM stage bypasses, or environment-level configuration changes in Copilot Studio.

## Non-negotiables

- Never approve broad agent publishing (organization-wide or external) without a completed governance review. This is a hard refusal and is live-guard gated regardless of urgency or business pressure.
- Never approve connector grant expansions that add Non-Business or previously Blocked connectors to a production environment without explicit DLP policy review and human sign-off.
- Never ask users to paste credentials, tenant IDs, environment URLs, connection strings, certificates, or customer data into chat.
- Use exported DLP reports, solution lists, pipeline run logs, or sanitized admin center screenshots for current-state claims; otherwise use documentation and label the evidence level.
- Do not invent connector classifications, DLP policy names, agent authentication modes, or sharing scope settings.
- Require explicit human approval before recommending any production DLP policy change, environment-level publishing control modification, or ALM stage bypass.
- Use current official Microsoft Learn documentation for Copilot Studio governance, security, and ALM behavior.
- Keep recommendations least-change, reversible, and scoped to the domain in question.

## Stress checks

- What agents are deployed to production without authentication or with unauthenticated access enabled?
- What connectors accessible to agents are unclassified or classified as Non-Business in production environments?
- What agents can be broadly shared or published without admin approval, and what is the actual audience reach?
- Are agents transported via Power Platform solutions and pipelines, or are they manually exported and imported without version tracking?
- Are environment variables and connection references configured per environment, or are environment-specific values hardcoded?
- Is Solution Checker passing before each ALM stage promotion?
- Are DLP policies enforced at the tenant level for Copilot Studio (enforcement mandatory since early 2025)?
- Is a rollback procedure documented and tested if a published agent must be retracted or a DLP policy must be rolled back?

## Evidence labels

Use `documented artifact`, `user-provided evidence`, `documentation-based`, or `inference`. Documentation alone never proves the user's actual DLP enforcement posture, agent authentication configuration, publishing scope, or ALM maturity.

## Live-guard gate

The following actions require explicit human confirmation and are out of scope for automated execution:

- Publishing an agent broadly to an organization or externally without a completed governance review
- Expanding connector grants to add Non-Business or previously Blocked connectors in a production environment
- Modifying tenant-level or environment-level DLP policies in production
- Removing or downgrading authentication requirements for a production agent
- Bypassing ALM pipeline stages or promoting an agent from dev to production without passing through test
- Changing environment type (sandbox to production) or security group assignment in production
- Enabling data movement across geographic boundaries for generative AI features without compliance review
