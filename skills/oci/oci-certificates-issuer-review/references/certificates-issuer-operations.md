# OCI Certificates Issuer Review Operations

> Version note: OCI service behavior and tooling change over time. Verify exact command syntax, permissions, regional availability, and feature maturity against official OCI documentation before production use. Do not paste secrets or sensitive identifiers into commands, files, or chat.

Use this reference for current, source-grounded service behavior and the hard review gates that the lean `SKILL.md` intentionally does not carry.

## What people get wrong

- Using a root CA directly for workload certificate issuance.
- Granting broad certificate-authority management permissions to cert-manager or automation.
- Assuming instance-level identity is equivalent to pod/service-account scoped workload identity.
- Skipping OCSP/revocation reachability because most clients soft-fail.

## Officially grounded service shape

- Official OCI documentation describes the service behavior and lifecycle concepts for this domain, but it does not prove the user's tenancy, compartments, IAM policies, limits, deployed resources, or production readiness.
- OCI API evidence through the user’s configured read-only OCI MCP shows certificate and certificate-authority list operations expose compartment, lifecycle-state, name, issuer CA, certificate ID, and sorting filters. Treat this as API shape evidence, not proof of CA safety.
- Current-state claims need sampled read-only evidence or sanitized user-provided evidence.

Documentation evidence proves documented OCI service behavior. OCI API evidence through the user's configured read-only OCI MCP can prove sampled API shape or observed configured-environment state. Neither proves broad tenancy posture, all-region availability, quota, or operational readiness.

## Non-negotiable design rules

- Keep root CAs out of routine workload issuance.
- Use subordinate CA and issuance rules for cert-manager-style automation.
- Use least-privilege IAM scoped to issuance needs, not CA deletion/update authority.
- Prefer workload identity for pod-scoped issuance over broad node-level authority.
- Never expose private keys, CA material, certificate bundles with secrets, or customer identifiers in chat.

## Minimal safe implementation flow

- Identify issuer, CA hierarchy, OKE/authentication model, and namespace/service-account boundary.
- Review official Certificates, IAM policy, and OKE workload identity guidance.
- Collect sampled read-only API evidence for CAs/certificates where available.
- Classify findings by CA hierarchy, issuance constraints, IAM, OCSP, and lifecycle.
- Return severity, evidence label, remediation, approval needs, and safe validation.

## High-risk assumptions to kill

- “A certificate was issued, so the issuer is safe.”
- “A root CA in the service is harmless if access is “limited.””
- “Node/instance authority is close enough for cert-manager.”
- “Broad manage permissions are needed for certificate requests.”
- “Revocation checking can be ignored because clients usually soft-fail.”

Those are lazy assumptions.

## Safe command/code verification targets

- List CA and certificate metadata in the confirmed compartment only.
- Check CA type, issuer chain, lifecycle state, rule constraints, expiration, and certificate-version count.
- Inspect IAM policy text for request-specific conditions and reject broad CA management unless explicitly justified.
- Verify OKE workload identity binding and service account scope for issuer automation.
- Check OCSP/revocation path reachability and documented fallback behavior before calling trust posture ready.

## Safe verification targets

- Scope is confirmed without exposing sensitive identifiers in chat or committed docs.
- Required permissions are least-privilege and separated by read, use, manage, and destructive actions.
- Current-state findings are labeled as sampled evidence, not broad proof.
- Risky mutations have explicit approval, blast-radius review, rollback, and validation.
- Official-source claims are linked to service docs and not overstated as live posture.

## When to push back

- The user asks for a write/delete/start/stop/update/remediate action before scope and owner are clear.
- The answer would depend on live infrastructure state but only documentation evidence exists.
- The proposed access is broader than the task requires.
- The plan has no rollback, owner, or validation step.
