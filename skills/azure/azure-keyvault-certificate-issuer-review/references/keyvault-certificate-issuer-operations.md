# Azure Key Vault Certificate Issuer Operations

Use this reference for current, source-grounded service behavior and the hard review gates that the lean `SKILL.md` intentionally does not carry.

## What people get wrong

- Assigning management-plane contributor access when only certificate data-plane lifecycle operations are needed.
- Ignoring that a certificate also creates backing key and secret objects.
- Allowing exportable private keys for cluster-internal mTLS without a specific need.
- Letting cert-manager and Key Vault renewal policies race without a clear owner of renewal timing.
- Treating integrated CA setup as safe without checking requester credential scope and contacts.

## Officially grounded service shape

Microsoft Learn evidence says a Key Vault certificate creates addressable key and secret objects, has a policy that controls issuer, key properties, exportability, lifetime actions, and renewal behavior, and can use integrated issuers such as DigiCert and GlobalSign. Exportability controls whether private key material can be retrieved from the backing secret. Certificate contacts and Event Grid support lifecycle notification, and RBAC should separate certificate lifecycle permissions from broader vault administration.

- A Key Vault certificate policy defines subject/SANs, key properties, exportability, secret content type, lifetime actions, issuer, and validation type.
- Integrated issuers can automate renewal for supported CAs; nonintegrated CAs require different renewal automation or manual process.
- Certificate lifecycle events need contacts or event routing to accountable responders.
- Private endpoint and DNS posture determine whether AKS workloads can reach a locked-down vault.
- RBAC decisions must distinguish control plane, data plane, certificate, secret, and purge/recover operations.

## Non-negotiable design rules

- Prefer the least data-plane certificate role required; do not grant broad vault administration to cert-manager by default.
- Flag exportable certificates when private key extraction is unnecessary for the workload.
- Validate issuer object, certificate policy, lifetime action, contacts, and CA credential scope together.
- Check AKS network path and private DNS before declaring a private vault usable.
- Never request private keys, PFX content, CA passwords, or requester credentials in chat.

## Minimal safe implementation flow

- Scope the certificate issuer, Key Vault, AKS cluster, managed identity, namespaces, and certificate consumers.
- Review policy fields: issuer, key type/size, exportable, reuse key on renewal, SANs, lifetime action, enabled state, and tags.
- Review RBAC and network evidence without exposing credentials or private key material.
- Compare cert-manager renewBefore behavior against Key Vault lifetime action and owner expectations.
- Return severity-labeled findings with source labels and safe remediation path.

## High-risk assumptions to kill

- Certificate data-plane work does not require broad Key Vault or resource-group management-plane access by default.
- A Key Vault certificate is also backed by key and secret objects; certificate review must include private-key retrieval and backing-secret implications.
- `exportable` certificates are dangerous for mTLS and internal trust unless private-key extraction is explicitly required and audited.
- Integrated CA renewal does not remove the need for contacts, lifecycle events, owner response, and failed-renewal handling.
- Private endpoint enabled on the vault is insufficient unless AKS DNS, firewall, and egress paths are proven for the issuer workflow.

## Safe command/code verification targets

- Inspect certificate policy JSON or IaC for issuer, subject/SANs, key type/size, exportable, reuse-key-on-renewal, lifetime actions, secret content type, and enabled state.
- Review role assignments for certificate lifecycle permissions separately from secret, key, purge, and management-plane permissions.
- Check cert-manager issuer manifests or automation for managed identity binding, namespace scope, renewal timing, and absence of embedded CA credentials.
- Verify Key Vault network definitions include private endpoint, private DNS zone links, firewall posture, and AKS egress compatibility.
- Confirm monitoring covers certificate near-expiry, expiry, renewal success/failure, export operations, delete/recover/purge events, and accountable responders.

## Safe verification targets

- Managed identity has only required certificate operations, not broad vault delete or purge authority.
- Certificate policies align with organizational issuer, key, exportability, and validity standards.
- Renewal contacts/events exist and route to an accountable owner.
- Private endpoint, firewall, and DNS path match the AKS connectivity model.
- Rollback plan exists for failed renewal, wrong issuer, or bad private DNS change.

## When to push back

- The request asks to export private keys without a documented break-glass need.
- The identity has broad Contributor or Administrator posture and the user wants to accept it as fine.
- Issuer credentials are account-wide or unmanaged.
- No owner can explain whether Key Vault or cert-manager owns the next renewal event.
