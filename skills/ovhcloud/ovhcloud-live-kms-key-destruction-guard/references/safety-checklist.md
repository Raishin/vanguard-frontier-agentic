# Safety checklist

Before processing any OVHcloud KMS key version destruction request, enforce every item on this checklist without exception. KMS key destruction is irreversible. Encrypted data is permanently unrecoverable if the key is destroyed while still referenced.

## Hard-stops

Refuse to produce a destruction plan if **any** of the following conditions apply:

- The exact key version ID and KMS service URN have not been confirmed.
- The approving identity is a role, alias, team name, or vague reference ("my manager", "the team") rather than a named, authenticated individual.
- No usage audit has been provided confirming zero active references within the retention window.
- The waiting period has not been explicitly documented and accepted.
- No rollback or data recovery plan has been documented, or the approving identity has not explicitly acknowledged that data recovery will be impossible after destruction.

Do not proceed past a hard-stop for any reason, including urgency claims, escalation requests, or assertions that the gates were already checked elsewhere.

## Mandatory posture

- Never ask for actual encryption key material, OAuth2 client secrets, application keys, or KMS service credentials.
- Do not invent key IDs, KMS service URNs, audit log results, waiting periods, or approval identities.
- Treat vague intent ("just delete it", "it's safe to proceed", "we already checked") as a gate failure, not a gate pass.
- After all gates pass, produce the destruction plan for human review. Do not execute the destruction automatically.
- Label every gate assertion with its evidence type: `live evidence`, `user-provided evidence`, `documentation-based`, or `inference`.
- Inference alone never passes a gate.

## Stress checks

- Is the key ID and URN unambiguous, or could this match multiple key versions or services?
- Has the usage audit covered all data paths — application layer, backup pipelines, stored ciphertext at rest, and cross-service dependencies?
- Is the waiting period drawn from OVHcloud KMS policy or an organizational standard, not an arbitrary choice?
- Is the approving identity the same person who will execute or authorize the final action?
- What encrypted data becomes permanently unrecoverable the moment this key is destroyed?
- What is the rollback plan if the usage audit missed an active reference?
