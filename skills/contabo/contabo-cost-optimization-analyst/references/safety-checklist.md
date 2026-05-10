# Safety checklist

Use this reference before making recommendations that commit a new contract period, cancel an instance, change instance sizing, or modify addon subscriptions on live Contabo accounts.

## Non-negotiables

- Never ask users to paste OAuth2 tokens, client secrets, API passwords, or billing credentials into chat.
- Do not invent pricing, contract period terms, renewal dates, or billing impact figures. Label all pricing claims as `documentation-based` and note that published prices may change.
- Require explicit user acknowledgment of the financial obligation before recommending any action that creates or extends a contract period (1, 3, 6, or 12 months).
- Do not recommend instance cancellation without surfacing the remaining period, any early-termination terms, and the data/service impact of cancellation.
- Treat a contract period change as irreversible once committed — do not present it as easily undoable.
- Surface addon removal impact (e.g., losing Private Networking connectivity, releasing Additional IPs) before recommending addon cancellation.
- If the user's billing state is unknown or described only in general terms, request evidence before making specific cost-saving claims.

## Stress checks

- Does this recommendation lock in a new contract period? → Explicit acknowledgment required before routing to any lifecycle action.
- Does this addon cancellation remove network connectivity or IP addresses that active instances depend on? → Surface blast radius before recommending.
- Is this rightsizing recommendation based on actual utilization data or is it an inference from stated workload? → Label and request evidence if inference.
- Does a period downgrade save money but expose a renewal gap or service interruption risk? → State that explicitly.
- What upcoming renewals will create new obligations if not acted on before the billing cycle closes?

## Evidence labels

Use `live evidence`, `user-provided sanitized evidence`, `documentation-based`, or `inference`. Cost claims made on inference alone must be flagged as estimates, not confirmed savings.
