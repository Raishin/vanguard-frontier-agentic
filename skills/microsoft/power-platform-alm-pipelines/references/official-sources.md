# Official sources

Use this reference only when you need source grounding for Power Platform ALM, Pipelines, or solution lifecycle behavior.

## Microsoft Learn documentation

Use these as starting points, not as proof of the user's actual pipeline configuration, environment topology, or deployment history:

- https://learn.microsoft.com/power-platform/alm/implement-healthy-alm — Healthy ALM scenarios covering new projects, citizen development, moving from single environment, moving from unmanaged to managed solutions, DevOps automation, and team development. Supports ALM pattern assessment and anti-pattern detection steps.
- https://learn.microsoft.com/power-platform/alm/pipelines — Overview of Power Platform Pipelines: maker-run preconfigured deployments, pre-flight validation, automatic solution backup, sequential stage enforcement, and licensing requirements for Managed Environments. Supports pipeline configuration review steps.
- https://learn.microsoft.com/power-platform/alm/set-up-pipelines — Pipeline setup via platform host (maker-configured) and custom host (admin-governed). Supports environment topology and host configuration review.
- https://learn.microsoft.com/power-platform/alm/run-pipeline — Step-by-step pipeline run: stage selection, pre-flight checks, connection reference and environment variable injection, deployment scheduling, and notes. Supports deployment gate review steps.
- https://learn.microsoft.com/power-platform/alm/move-from-unmanaged-managed-alm — Scenario 3: migrating from unmanaged to managed solutions in production. Critical for assessing organizations that have not yet adopted managed solution posture.
- https://learn.microsoft.com/power-platform/alm/devops-build-tools — Power Platform Build Tools for Azure DevOps: export, import, solution checker, and environment management tasks for CI/CD pipelines. Supports pro-dev ALM review.
- https://learn.microsoft.com/power-platform/alm/devops-github-actions — GitHub Actions for Power Platform: ALM automation for developer and admin teams using GitHub platform.
- https://learn.microsoft.com/power-platform/developer/cli/reference/pipeline — pac pipeline CLI commands: pac pipeline list and pac pipeline deploy. Supports developer-driven deployment review.
- https://learn.microsoft.com/power-platform/alm/form-alm-recommendations — Recommendations for healthy model-driven app form ALM: full vs. differential FormXml, publisher ownership, and avoiding manual customizations.xml edits.
- https://learn.microsoft.com/power-platform/alm/git-integration/overview — Native Git integration for Dataverse solutions: source control, branch strategy, and commit discipline.

## Grounding rule

Official documentation explains Power Platform ALM and pipeline behavior. It does not prove the user's actual environment topology, pipeline configuration, solution posture, or deployment history. Prefer exported solution analysis reports, pipeline run logs, or sanitized user-provided summaries for current-state claims. Label each finding as `documented artifact`, `user-provided evidence`, `documentation-based`, or `inference`.
