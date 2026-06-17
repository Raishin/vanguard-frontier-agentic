# Safety checklist

Use this reference before any recommendation involving production DLP policies, flow ownership/sharing, or connector configuration in Power Automate.

## Non-negotiables

- Never ask users to paste credentials, connection secrets, tenant IDs, environment URLs, or customer data into chat.
- Use Power Platform admin center exports, CoE Starter Kit dashboards, or sanitized user-provided evidence for current-state claims; otherwise use documentation and label the evidence level.
- Do not invent flow counts, sharing distributions, or DLP policy contents.
- Require explicit human approval before recommending any production DLP, flow-ownership, or connector change.
- Use current official Microsoft Learn documentation for Power Automate sharing, DLP, and error-handling behavior.
- Keep recommendations least-privilege and reversible: prefer run-only sharing over co-ownership; keep run-only users out of the Environment Maker role.

## Stress checks

- Which business-critical flows have a single owner (bus-factor risk)?
- Where is co-ownership broader than needed, or shared outside the environment's security group?
- Which connectors are unscoped by DLP, or use HTTP/custom connectors?
- Which flows lack error handling (run-after/Terminate), retry policies, or failure notifications?
- Which connections risk OAuth expiry or use unrotated service-account credentials?
- What rollback exists if a DLP or sharing change breaks a production flow (DLP changes take effect immediately)?

## Evidence labels

Use `documented artifact`, `user-provided evidence`, `documentation-based`, or `inference`. Documentation alone never proves the user's actual flow inventory, sharing posture, or DLP configuration.

## Live-guard gate

The following actions require explicit human confirmation and are out of scope for automated execution:

- Modifying production DLP policies or connector business/non-business classifications
- Changing production flow ownership, co-owners, or run-only sharing at scale
- Adding or removing connectors/custom connectors in production environments
- Bulk-disabling, deleting, or re-pointing production flows
- Rotating or re-authenticating production service-account connections
