# Official sources

Use this reference only when you need source grounding for Power Automate sharing, DLP, error handling, or monitoring behavior.

## Microsoft Learn documentation

Use these as starting points, not as proof of the user's live flow inventory or tenant DLP configuration:

- https://learn.microsoft.com/power-automate/guide-to-cloud-flow-sharing-permissions — Mitigating sharing risk: environment security groups, limiting co-ownership, run-only sharing, DLP as a safety net, maker-vs-run-only segmentation, periodic auditing. Supports the ownership/sharing and security-segmentation steps.
- https://learn.microsoft.com/power-automate/create-team-flows — Sharing a cloud flow and best practices: security roles to segment duties, run-only access over co-ownership, DLP enforcement, regular auditing (Power Platform admin center, CoE Starter Kit). Supports sharing and auditing steps.
- https://learn.microsoft.com/power-automate/guidance/coding-guidelines/error-handling — Robust error handling: run-after configuration, Terminate action, error logging and notifications, Application Insights. Supports the resilience and monitoring steps.
- https://learn.microsoft.com/power-automate/guidance/planning/reducing-risk — Reducing risk: custom failure notifications, assigning multiple owners, clustering for business-critical automation. Supports the continuity and resilience steps.
- https://learn.microsoft.com/power-automate/error-reference — Cloud flow error code reference: 401/403 (auth, DLP block), 429 throttling, ConnectionAuthorizationFailed (expired tokens), retry guidance. Supports diagnosing connector/DLP and credential-lifecycle risks.

## Grounding rule

Official documentation explains Power Automate behavior. It does not prove the user's actual flow inventory, sharing posture, DLP policy set, or monitoring coverage. Prefer Power Platform admin center exports, CoE Starter Kit dashboards, or sanitized user-provided evidence for current-state claims. DLP policy changes take effect immediately and can block flows without warning — treat DLP as tenant-specific and verify before asserting.
