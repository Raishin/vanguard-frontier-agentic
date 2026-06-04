---
name: azure-keyvault-certificate-issuer-review
description: Use this skill when reviewing Azure Key Vault certificate issuer configurations for cert-manager on AKS. Trigger on any request to audit Key Vault certificate policies, Managed Identity role assignments, exportability settings, private endpoint connectivity, integrated CA credentials, or rotation policy alignment.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: 0.1.2
  updated: "2026-06-04"
  category: security
---

# Azure Key Vault Certificate Issuer Review

## Purpose

Review Azure Key Vault configurations used as certificate issuers for cert-manager on AKS. Identify Managed Identity role assignment gaps (data plane vs management plane confusion), certificate policy misalignment, exportability risks, network connectivity issues, integrated CA credential over-scoping, and rotation race conditions between cert-manager and Key Vault auto-rotation. Output severity-labeled findings with evidence and remediation steps.

## Lean operating rules

- Check the Managed Identity (or Service Principal) role assignment on the Key Vault: the correct role is `Key Vault Certificate Officer` (data plane). Flag `Key Vault Contributor` as HIGH — it grants management plane access including vault deletion. Flag `Key Vault Administrator` as HIGH (full data plane + management).
- Verify whether Key Vault RBAC mode is enabled (`enableRbacAuthorization: true`). If legacy access policies are used instead of RBAC, flag as MEDIUM (harder to audit, no Azure AD Conditional Access integration).
- Review `exportable` in the Key Vault certificate policy. Flag `exportable: true` on certs used for cluster-internal mTLS as MEDIUM (private key unnecessarily extractable from Key Vault).
- Check Key Vault network access configuration: if `publicNetworkAccess: Disabled`, verify the AKS cluster has private endpoint access to the Key Vault and DNS resolution via private DNS zone. Flag missing private endpoint as MEDIUM.
- For integrated CAs (DigiCert, GlobalSign): verify the Key Vault has the CA integration configured and the credential secret is scoped to a minimum (single certificate profile, not account-wide).
- Review cert-manager `renewBefore` against the Key Vault certificate's auto-rotation policy to detect overlapping rotation windows. Flag simultaneous rotation triggers as MEDIUM.
- Label all findings as live evidence, documentation-based, or inference.

## References

Load these only when needed:

- [Azure Key Vault Certificate Issuer Operations](references/keyvault-certificate-issuer-operations.md) — use for current service behavior, common failure modes, hard design rules, verification targets, and push-back conditions.
- [Safety checklist](references/safety-checklist.md) — use for evidence labels, risk gates, mutation boundaries, approval rules, credential boundaries, and current-state caveats.
- [MCP and evidence path](references/mcp-and-evidence.md) — use when choosing documentation-based evidence, sampled read-only evidence, or sanitized user evidence.
- [Official sources](references/official-sources.md) — use when you need the detailed Microsoft documentation list or source notes.
- [Workflow and output contract](references/workflow-and-output.md) — execution flow and final response contract.

## Response minimum

- Severity-labeled findings list (CRITICAL / HIGH / MEDIUM / LOW)
- Evidence source for each finding
- Specific resource name or field that caused the finding
- Recommended remediation with example Azure CLI command or policy snippet
- Overall Key Vault certificate issuer posture verdict
