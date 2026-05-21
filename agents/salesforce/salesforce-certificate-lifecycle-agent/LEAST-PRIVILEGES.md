# Least-privilege Salesforce posture for Salesforce Certificate Lifecycle Agent

## Execution tier

**T0 — Static Review**

Rationale: `execution_tier: "static-review"` declared in `metadata.json`. This agent reviews
certificate and key management configurations — self-signed certificates, CA-signed certificates,
JWT signing certs, SAML signing, Named Credential mTLS, and rotation procedures — from sanitized
excerpts. It never accesses live certificate stores and never connects to any org.

## Identity model

No live identity required. This agent works from pasted sanitized excerpts only — certificate
metadata exports (subject, issuer, validity period, key algorithm), Named Credential
configuration fragments, SAML metadata XML, and JWT signing certificate references. It never
receives private key material, never initiates an OAuth flow, and never establishes a connection
to a Salesforce org.

The agent must refuse any input that contains private key material, PEM-encoded private keys,
or PKCS#12 bundles even if presented as "test" or "sample" data.

## Run As account requirements

Not applicable. No Connected App, no service account, no OAuth client.

## MCP server binding

None. No MCP server is permitted for T0 agents.

## Blast-radius bound

This agent cannot upload, renew, or revoke certificates in any org, cannot modify Named
Credential mTLS bindings, cannot alter SAML assertion signing configuration, and cannot
trigger any certificate rotation. Even if an attacker fully controlled the agent's output, no
certificate lifecycle action, no key material, and no PKI configuration can change as a direct
result of this agent's execution. Private key material cannot be extracted because it is never
accepted as input.

## Refusal triggers

- [ ] Any request to connect to a live Salesforce org certificate store, a CA API, or any
      key management service
- [ ] Any input that includes or asks the agent to process private key material, PEM-encoded
      private keys, PKCS#12 bundles, or HSM access credentials
- [ ] Any request to approve, initiate, or execute a certificate rotation or renewal in a live
      org
- [ ] Any request to assess certificate trust chains without the certificate metadata export or
      equivalent sanitized documentation provided in the conversation
- [ ] Any request that treats an expired or near-expiry certificate as acceptable without a
      documented remediation plan and timeline
- [ ] Any request to confirm a Named Credential mTLS binding as secure without the certificate
      subject and expiry details provided

## Escalation path

All requests to upload new certificates, modify Named Credential configurations, rotate SAML
signing certificates, or make any live certificate lifecycle change must be routed to
**`salesforce-live-guard-agent`** with a named human decision owner, documented rollback plan,
and complete change envelope.

---

References: [Execution tiers](../../docs/execution-tiers.md) | [Salesforce agents README](../README.md)

## Validation checklist

Before submitting certificate configuration artifacts for review by this agent:

- [ ] Certificate exports contain only metadata fields (subject, issuer, serial number, validity period, key algorithm, key size) — no private key material
- [ ] Named Credential configuration excerpts describe the authentication type and certificate reference, not raw credential values
- [ ] SAML metadata XML is the public federation metadata, not an assertion or signed response containing private key usage
- [ ] JWT signing certificate references identify the certificate subject and thumbprint, not the private key
- [ ] All org IDs and environment-specific connection strings have been redacted before submission

## Companion skill

`salesforce-zero-trust-maturity-skill` — use before invoking this agent to establish the
current certificate and PKI maturity baseline. The skill's certificate rotation and mTLS
sections provide the evaluation criteria this agent applies when reviewing Named Credential
mTLS bindings and SAML assertion signing configurations.
