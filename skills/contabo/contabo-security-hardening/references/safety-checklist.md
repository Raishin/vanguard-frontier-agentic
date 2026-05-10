# Safety checklist

Use this reference before making hardening recommendations that touch SSH access paths, firewall rules, user accounts, or credential configuration on live Contabo instances.

## Non-negotiables

- Never ask users to paste OAuth2 tokens, client secrets, API passwords, SSH private keys, or any raw credentials into chat.
- SSH keys must always be referenced by Contabo secret IDs — never include raw private key material in recommendations, scripts, or API payloads.
- Do not recommend disabling SSH password login or root login unless an alternative access path (SSH key, console access) is confirmed to be working first.
- Do not recommend opening firewall rules broader than the minimum required port and source range.
- Do not invent firewall rule IDs, instance IDs, secret IDs, or current configuration state. Label any unconfirmed claim.
- Require explicit user acknowledgment before recommending changes to the only active SSH access path.
- OAuth2 token values must never appear in recommendation output, logs, or script echo statements.
- Cloud-Init userData must be reviewed for embedded secrets, curl-pipe-sh patterns, or commands that disable audit logging before inclusion in any API payload.

## Stress checks

- Does this change remove or weaken the only SSH or console access path? → Stop until alternative is confirmed.
- Does this firewall change open a management port (22, 3389, 443-admin) to 0.0.0.0/0? → Require justification and scope limitation.
- Does this Cloud-Init fragment contain raw credentials, hardcoded secrets, or unauthenticated remote execution? → Refuse to pass as userData.
- Are OAuth2 token refresh patterns logging token values? → Flag and replace with sanitized patterns.
- Is the current firewall or user configuration based on inference rather than evidence? → Label and request evidence before recommending changes.
- What is the blast radius if this hardening change locks out all administrative access?

## Evidence labels

Use `live evidence`, `user-provided sanitized evidence`, `documentation-based`, or `inference`. Hardening recommendations made on inference alone must be flagged as provisional and subject to evidence confirmation before implementation.
