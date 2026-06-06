# Azure Key Vault Certificate Issuer Review Agent Operations

> Version note: Azure service behavior, API surfaces, permissions, and operational safety guidance change. Verify exact behavior against Microsoft Learn documentation through the user's configured documentation MCP and any sampled configured-environment evidence before production use. Do not paste credentials, tokens, tenant identifiers, subscription identifiers, connection strings, certificates, private keys, kubeconfigs, or customer data into prompts, commands, or reference examples.

## What people get wrong

- Granting management-plane Key Vault Contributor where certificate data-plane roles are enough.
- Ignoring that a Key Vault certificate creates addressable key and secret objects with the same name.
- Allowing exportable private keys when the workload only needs Key Vault-backed signing or TLS lifecycle management.
- Confusing public certificate download with private key export through the certificate addressable secret.
- Letting cert-manager, Key Vault lifetime actions, and CA renewal workflows race without a single owner.

## Officially grounded service shape

- Key Vault certificates manage X.509 certificates and create an addressable key plus secret alongside certificate metadata.
- Certificate policies define subject, SANs, key type, key length, exportability, reuse on renewal, secret content type, lifetime actions, and issuer parameters.
- Exportable certificate policies allow retrieval of private key material from the addressable secret; nonexportable policies exclude the private key from the secret value.
- Azure RBAC is recommended for certificate access; Certificate Officer can manage certificates without managing permissions.
- Partner issuers can support automatic renewal; nonpartner issuers may not. Publicly trusted certificates may involve CA and certificate transparency systems outside Azure.

That is the key insight:

> The agent must review certificate issuance as a private-key custody and renewal-control problem, not just as a certificate-expiry checklist.

## Non-negotiable design rules

### 1. Prefer data-plane certificate roles and managed identities over management-plane vault control.

### 2. Treat exportable private keys as high risk unless the workload and custody model require export.

### 3. Require policy evidence for key type, key size, SANs, key usage, lifetime actions, issuer, and renewal owner.

### 4. Separate certificate public metadata reads from secret-backed private key export.

### 5. Do not approve cert-manager and Key Vault auto-renewal overlap without an ownership and race-condition analysis.

## Minimal safe implementation flow

- Classify the issuer path: partner CA, nonpartner CA, self-signed, import, cert-manager integration, or application certificate binding.
- Ground the review in Microsoft Learn certificate composition, policy, access control, export, renewal, and network docs.
- Use read-only configured-environment evidence for policy, issuer, lifetime actions, RBAC, network access, diagnostics, and renewal events.
- Identify private-key custody risks, issuer credential risks, network reachability gaps, and renewal owner conflicts.
- Return severity-labeled findings with safe remediation and evidence gaps.

## High-risk assumptions to kill

- Key Vault Contributor is acceptable for certificate automation.
- Downloading a certificate and exporting a private key are the same risk.
- A certificate contact or lifetime action means renewal will succeed.
- Nonexportable is optional for internal mTLS or signing keys.
- Documentation proves issuer configuration, private endpoint reachability, or cert-manager behavior.

## Safe command/code verification targets

Verify against current docs and safe local or read-only tooling before use:

- Certificate policy: issuer, subject, SANs, key type, key size, exportable flag, reuse on renewal, lifetime action, and content type.
- RBAC and identity: certificate data-plane role, vault management roles, managed identity scope, and CA credential access.
- Network and access: Key Vault firewall, private endpoint, DNS, AKS or workload reachability, and public network exposure.
- Renewal and events: expiration, renewal attempts, failed operations, Event Grid or alerting, cert-manager reconciliation, and rollback.
- Private key custody: export events, secret reads, backup/download process, temporary files, and audit logs.

## When to push back

- Automation asks for management-plane vault roles or private key export without explicit justification.
- Issuer credentials or PFX material would need to be pasted into the conversation.
- Renewal is claimed safe without event, owner, and application binding evidence.
- cert-manager and Key Vault auto-renewal both manage the same certificate without clear ownership.
