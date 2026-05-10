# Safety checklist

Use this reference before routing any request that touches a live mutation, billing obligation, or production impact.

## Non-negotiables

- Never ask users to paste OAuth2 tokens, client secrets, API passwords, SSH private keys, S3 secret keys, or any account credentials into chat.
- Do not invent instance IDs, product IDs, region availability, pricing, or billing terms. Label any claim that has not been confirmed by live evidence or official documentation.
- Refuse to route a request to an advisory skill when the request clearly requires a live-guard (VPS/VDS mutation or bucket deletion).
- Demand explicit contract period acknowledgment before routing any request that creates a new billing obligation.
- Never treat a vague approval ("go ahead," "do it," "looks fine") as sufficient sign-off for a lifecycle or storage mutation routing — escalate to the live-guard for formal confirmation.
- Separate confirmed routing from inferred routing. If the user's intent is ambiguous, ask one clarifying question rather than assuming the narrower domain.

## Stress checks

- Does any part of this request create, reinstall, or cancel a VPS or VDS? → Live-guard required.
- Does any part of this request delete, overwrite, or migrate Object Storage buckets or objects? → Live-guard required.
- Does any part of this request commit a new contract period? → Explicit billing acknowledgment required before routing.
- Is the stated billing obligation (period × price) accurate or is it an inference? → Label accordingly.
- Is the user's evidence level sufficient to route confidently, or is additional evidence needed first?

## Evidence labels

Use `live evidence`, `user-provided sanitized evidence`, `documentation-based`, or `inference`. Routing decisions made on inference alone must be flagged as provisional.
