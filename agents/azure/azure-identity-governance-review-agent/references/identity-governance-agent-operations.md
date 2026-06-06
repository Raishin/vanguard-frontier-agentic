# Azure Identity Governance Review Agent Operations

> Version note: Azure service behavior, API surfaces, permissions, and operational safety guidance change. Verify exact behavior against Microsoft Learn documentation through the user's configured documentation MCP and any sampled configured-environment evidence before production use. Do not paste credentials, tokens, tenant identifiers, subscription identifiers, connection strings, certificates, private keys, kubeconfigs, or customer data into prompts, commands, or reference examples.

## What people get wrong

- Treating PIM as proof of least privilege while permanent assignments, stale eligible roles, or weak activation controls remain.
- Running access reviews without checking reviewer ownership, recurrence, auto-apply behavior, decision helpers, and removal results.
- Equating entitlement management access packages with safe access when lifecycle expiration, approval, and connected organization boundaries are vague.
- Ignoring license and role prerequisites, then assuming a governance control exists in the user environment.
- Reviewing privileged access without separating Microsoft Entra roles, Azure resource roles, groups, applications, and guests.

## Officially grounded service shape

- Microsoft Learn frames least privilege around required resources, RBAC, just-in-time privilege, regular auditing, and default deny.
- PIM provides just-in-time access for privileged Microsoft Entra and Azure resource roles, but activation settings and reviews determine control quality.
- Access reviews help verify continued need for group memberships, application access, and role assignments; review outcome handling must be explicit.
- Entitlement management automates access request workflows, assignments, reviews, and expiration, but it is not a substitute for owner accountability.
- Least privileged administration differs by feature; Identity Governance Administrator, User Administrator, and Privileged Role Administrator have different scopes.

That is the key insight:

> The agent must prove that privilege is time-bound, reviewed, owned, and removed when no longer needed; tool presence or policy existence is not governance.

## Non-negotiable design rules

### 1. Do not call access least-privileged until role scope, assignment type, activation policy, owner, and review cadence are evidenced.

### 2. Treat permanent privileged assignments as blockers unless justified, owned, monitored, and reviewed.

### 3. Require review outcome behavior, auto-apply or manual follow-up, and removal evidence before trusting access reviews.

### 4. Separate documentation-based capability claims from sampled configured-environment evidence.

### 5. Never request raw tenant, user, group, guest, or membership dumps; require sanitized evidence.

## Minimal safe implementation flow

- Classify the review path: PIM, access reviews, entitlement management, lifecycle workflows, privileged roles, guests, or app access.
- Ground the control in Microsoft Learn identity governance and least-privilege guidance.
- Use read-only configured-environment evidence when available for assignments, eligibility, activation settings, review cadence, owners, and outcomes.
- Map each finding to standing access, missing owner, weak cadence, incomplete removal, license prerequisite, or unsupported inference.
- Return verdict, evidence level, blockers, safe next actions, and open questions.

## High-risk assumptions to kill

- Eligible access is safe even if activation does not require approval or strong authentication.
- An access review exists, so stale access is removed.
- Entitlement packages automatically solve joiner-mover-leaver risk.
- Global or privileged roles are acceptable because only a few admins have them.
- Documentation proves licensing, tenant configuration, role assignments, or review outcomes.

## Safe command/code verification targets

Verify against current docs and safe local or read-only tooling before use:

- Privileged role assignments: active versus eligible, permanent versus time-bound, activation requirements, and approval settings.
- Access reviews: scope, reviewers, recurrence, decision helpers, auto-apply behavior, outcome history, and nonresponse handling.
- Entitlement management: access package owners, approval stages, assignment expiration, connected organizations, and review settings.
- Lifecycle workflows and group governance: owner coverage, orphaned groups, guest lifecycle, and removal automation evidence.
- Audit and reporting: sampled logs, alert routing, stale privileged access reports, and sanitized evidence boundaries.

## When to push back

- The user wants broad privileged assignments without JIT, approval, MFA, owner, or review evidence.
- Review results are claimed but removal or follow-up evidence is missing.
- The task requires tenant-specific proof but only documentation is available.
- Evidence includes raw users, guests, groups, or customer data that should be redacted.
