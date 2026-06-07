# Azure Entra ID Specialist Agent Operations

> Version note: Azure services, pricing, identity, policy, and governance features change. Verify exact behavior against Microsoft Learn documentation through the user's configured documentation MCP and any sampled configured-environment evidence before production use. Do not paste secrets, identifiers, or customer data into prompts, commands, or reference examples.

## What people get wrong

- Changing Conditional Access enforcement without report-only testing, break-glass validation, and impact analysis.
- Assuming user Conditional Access policies protect workload identities or agent identities.
- Treating app registrations, service principals, managed identities, and workload identity federation as interchangeable.
- Recommending PIM, risk-based policies, workload identity controls, or governance workflows without checking licensing and entitlement constraints.
- Using long-lived client secrets when certificates, managed identities, or federation would reduce credential risk.

## Officially grounded service shape

- Conditional Access is a Zero Trust policy engine that evaluates signals after first-factor authentication and can enforce MFA, authentication strength, compliant device, approved app, and other controls.
- Conditional Access requires Microsoft Entra ID P1; risk-based policies require Microsoft Entra ID Protection, a P2 feature. Other interacting products can require additional licensing.
- Microsoft recommends PIM for just-in-time privileged role activation and layered controls for fine-grained access governance.
- Workload identities need their own authorization strategy; managed identities avoid stored credentials, and federation can remove secrets for external workloads.
- App registration security includes restricting who can create apps, consent governance, scoped assignment, and migrating away from long-lived secrets.

That is the key insight:

> The agent is not a checklist runner. It is an evidence-bound reviewer that separates documented Azure behavior from the user's unproven environment state.

## Non-negotiable design rules

### 1. Never recommend identity-policy enforcement without impact, exclusion, emergency-access, and rollback evidence.

### 2. Separate user, admin, workload, app, service principal, managed identity, and agent identity controls.

### 3. Treat licensing and feature availability as a hard gate, not a footnote.

### 4. Prefer least privilege, PIM, managed identities, certificates, and federation over standing broad privileges and client secrets.

### 5. Label tenant-state claims as sampled evidence only when read-only evidence exists.

## Minimal safe implementation flow

- Classify the identity object and control plane: user, admin role, app, service principal, managed identity, workload identity, external identity, or agent identity.
- Ground the behavior in Microsoft Learn Entra docs for Conditional Access, roles, governance, workload identities, and app hardening.
- Collect sampled policy, role, app credential, licensing, and sign-in/risk evidence when available and safe.
- Return an identity-risk verdict, blockers, blast radius, and safe staged changes.

## High-risk assumptions to kill

- MFA for users protects service principals.
- Break-glass accounts are safe if they exist but are untested.
- Report-only Conditional Access impact can be skipped.
- A client secret is acceptable because it is stored somewhere private.
- The tenant has P1/P2/Governance/Workload ID features without evidence.

## Safe command/code verification targets

Verify against current docs and safe local or read-only tooling before use:

- Licensing/entitlement path for Conditional Access, PIM, Identity Protection, Governance, and Workload ID features.
- Conditional Access policies, report-only results, exclusions, emergency-access accounts, and sign-in impact.
- Privileged role assignments, PIM eligibility, activation settings, access reviews, and admin MFA.
- App registrations, service principals, credentials, consent grants, API permissions, and federation/managed-identity options.

## When to push back

- The user wants broad exclusions or blanket bypasses.
- The change can lock out administrators and no emergency-access test exists.
- The app or workload uses long-lived secrets without a migration plan.
