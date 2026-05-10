# Safety checklist

Before executing any Contabo VPS or VDS lifecycle mutation (create, reinstall, cancel), enforce every item on this checklist. Never proceed without all mandatory gates confirmed.

## Hard-stop gates — all required, no exceptions

Do not execute any mutation unless ALL of the following are confirmed in writing by the user:

1. **Target confirmed**: Instance ID (for reinstall or cancel) OR product ID + region (for create). Do not infer the target from prior conversation context alone.
2. **Contract period explicitly acknowledged**: The user has stated the contract period (1, 3, 6, or 12 months) and confirmed they understand the billing obligation. "Sure" or "go ahead" is not acknowledgment — require the period and billing figure to be stated.
3. **Rollback plan documented**: A concrete recovery path is on record. "We'll figure it out" is not a rollback plan. Minimum: where data is backed up, how to restore access, who to contact.
4. **Named approving identity on record**: The full name or authenticated account identifier of the authorizing person. A role title, team name, ticket number, or alias alone is not sufficient.
5. **OAuth2 token freshness confirmed**: The token has been refreshed within the last 5 minutes. Stale tokens cause silent failures or partial operations. Refresh immediately before the mutation call.

## Non-negotiables

- Do not execute direct mutations against an ambiguous instance ID or product. If the user's intent is unclear, ask exactly one clarifying question.
- Do not treat "reinstall" as reversible. Reinstallation wipes the current OS and all data not backed up externally. State this explicitly before the user confirms.
- Do not treat cancellation as reversible. Once cancelled, the instance and its data may be unrecoverable. Confirm remaining period and data backup before proceeding.
- Do not include raw SSH private key material in any API payload, script, or recommendation. Reference only Contabo secret IDs.
- Do not log, echo, or include OAuth2 token values in any output, log file, or script.
- Review all Cloud-Init userData for embedded secrets, curl-pipe-sh patterns, hardcoded credentials, and commands that disable audit logging before passing to the API. Refuse userData that fails this review.
- Include a fresh UUIDv4 `x-request-id` header in every mutation call.

## Mandatory posture

- Prefer read-only inventory first. Always call `GET /v1/compute/instances` before any mutation to confirm current state.
- Prefer the smallest mutation. If a reinstall can be avoided by a configuration change, recommend that first.
- Treat the absence of a rollback plan as a blocker, not a detail to resolve post-mutation.
- If any hard-stop gate is missing, stop completely and list exactly which gates remain open. Do not proceed partially.
- If live credentials are present but the target instance or account is ambiguous, stop and request explicit confirmation.

## Stress checks

- Is reinstallation genuinely required, or can the goal be achieved without wiping the instance?
- Is the cancellation period remaining understood, and is there a data export or migration path confirmed?
- Is the contract period the shortest option that meets the requirement, or is the user being locked into a longer period unnecessarily?
- Is the Cloud-Init userData idempotent and free of destructive or unauthenticated commands?
- What happens if the API call succeeds but the instance reaches an unexpected state (e.g., reinstall stuck, create with wrong image)?

## Evidence labels

Use `live evidence`, `user-provided sanitized evidence`, `documentation-based`, or `inference`. Never proceed with a mutation based on inference about the target instance state.
