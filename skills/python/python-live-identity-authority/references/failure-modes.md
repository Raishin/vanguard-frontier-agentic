# Failure Modes This Role Prevents

The concrete production incidents this role's identity-and-authority verification is designed to prevent.

- A shared service account performs a sensitive action and no individual can be held accountable after an incident.
- A requester self-approves their own change, defeating separation of duties and hiding a conflict of interest.
- A standing administrative credential, once compromised, grants broad and indefinite access instead of a bounded JIT window.
- An expired or stale credential is accepted because no one checks credential age, extending an attacker's window.
- An approval granted for one target is reused against a different target, bypassing the scope the approver actually reviewed.
