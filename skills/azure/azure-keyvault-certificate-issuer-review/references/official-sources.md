# Official Sources

Use these sources to ground the skill. Microsoft Learn documentation proves documented Azure behavior; it does not prove the user's tenant, RBAC, quotas, deployed resources, or production readiness.

## Primary Microsoft Learn sources

- https://learn.microsoft.com/azure/key-vault/certificates/about-certificates
- https://learn.microsoft.com/azure/key-vault/certificates/secure-certificates
- https://learn.microsoft.com/azure/key-vault/certificates/overview-renew-certificate
- https://learn.microsoft.com/azure/key-vault/certificates/tutorial-rotate-certificates
- https://learn.microsoft.com/azure/key-vault/certificates/how-to-integrate-certificate-authority
- https://learn.microsoft.com/azure/key-vault/certificates/how-to-export-certificate
- https://learn.microsoft.com/azure/key-vault/general/rbac-guide
- https://learn.microsoft.com/azure/key-vault/general/network-security

## Grounding notes

- Documentation-based claim: Microsoft Learn evidence says a Key Vault certificate creates addressable key and secret objects, has a policy that controls issuer, key properties, exportability, lifetime actions, and renewal behavior, and can use integrated issuers such as DigiCert and GlobalSign. Exportability controls whether private key material can be retrieved from the backing secret. Certificate contacts and Event Grid support lifecycle notification, and RBAC should separate certificate lifecycle permissions from broader vault administration.
- Current-state claim: requires sampled read-only Azure evidence or sanitized user-provided evidence.
- Inference: allowed only when labeled and tied to observed fields or documented behavior.
- Do not include sensitive internal identifiers or secret material in findings.

## Source use rules

- Prefer Microsoft Learn documentation through the user's configured documentation MCP for current Azure service behavior.
- Use sampled read-only Azure evidence only to validate current configured-environment observations.
- If documentation and sampled evidence appear to conflict, report both and stop short of a production-ready verdict.
- Re-check official sources before changing high-risk guidance, because cloud behavior and feature availability can change.
