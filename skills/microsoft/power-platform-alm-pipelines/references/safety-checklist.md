# Safety checklist

Use this reference before any recommendation involving production pipeline configuration changes, deployment stage modifications, Managed Environment policy changes, or solution promotion in Power Platform.

## Non-negotiables

- Never approve unmanaged solutions in production environments. This is a hard refusal regardless of urgency, timeline pressure, or business justification.
- Never recommend bypassing a pipeline stage (e.g., skipping QA to deploy directly to production). Pipelines enforce sequential stage ordering; document any request to circumvent this and escalate.
- Never ask users to paste credentials, tenant IDs, environment URLs, connection strings, certificates, or customer data into chat.
- Use exported solution analysis, pipeline run logs, or sanitized user-provided summaries for current-state claims; otherwise use documentation and label the evidence level.
- Do not invent environment counts, solution version numbers, pipeline stage names, or deployment success rates.
- Require explicit human approval before recommending any production pipeline reconfiguration, Managed Environment policy change, or deployment stage removal.
- Use current official Microsoft Learn documentation for ALM, Pipelines, and solution behavior.
- Keep recommendations least-change, reversible, and scoped to the domain in question.

## Stress checks

- What solutions exist in an unmanaged state in test or production environments, and what is the migration path to managed?
- What pipeline stages are present, and is sequential stage ordering enforced?
- Are connection references and environment variables configured per environment, or are values hardcoded?
- Is Solution Checker running and passing before each stage promotion?
- Is a rollback procedure documented, tested, and accessible if a deployment breaks production?
- Are Managed Environments licensed and configured for all pipeline target environments?
- Is source control (Git integration or DevOps) tracking solution history, or are exports manual and untracked?
- Is the pipeline host (platform host or custom host) appropriate for the organization's governance requirements?

## Evidence labels

Use `documented artifact`, `user-provided evidence`, `documentation-based`, or `inference`. Documentation alone never proves the user's actual environment topology, pipeline configuration, solution posture, or deployment history.

## Live-guard gate

The following actions require explicit human confirmation and are out of scope for automated execution:

- Changing production pipeline stage configuration or removing deployment gates
- Modifying Managed Environment policies or environment type in production
- Promoting an unmanaged solution to a target environment
- Executing bulk solution imports or environment resets
- Removing or overwriting active customizations in a production environment
- Changing service principal credentials or pipeline host configuration in production
