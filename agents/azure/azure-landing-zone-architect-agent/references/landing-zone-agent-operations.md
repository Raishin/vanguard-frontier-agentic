# Azure Landing Zone Architect Agent Operations

> Version note: Azure service behavior, API surfaces, permissions, and operational safety guidance change. Verify exact behavior against Microsoft Learn documentation through the user's configured documentation MCP and any sampled configured-environment evidence before production use. Do not paste credentials, tokens, tenant identifiers, subscription identifiers, connection strings, certificates, private keys, kubeconfigs, or customer data into prompts, commands, or reference examples.

## What people get wrong

- Treating the reference architecture as a mandatory one-size-fits-all hierarchy instead of a starting point to adapt.
- Designing management groups before clarifying operating model, platform ownership, subscription vending, and workload delegation.
- Calling a landing zone production-ready without identity, governance, security, management, networking, and automation evidence.
- Collapsing platform landing zones and application landing zones into one subscription because it is simpler today.
- Ignoring brownfield migration, exception ownership, and policy remediation blast radius.

## Officially grounded service shape

- Azure landing zones are a scalable, secure, governed foundation for cloud workloads.
- The conceptual architecture includes tenant and billing, identity, resource organization, network, security, management, governance, and platform automation design areas.
- Platform landing zones host shared services such as identity, connectivity, management, and security; application landing zones host workload resources.
- Subscriptions are units of management and can separate application environments; management groups provide policy and governance inheritance.
- Landing zone implementation choices include portal accelerator, Bicep, Terraform, and subscription vending modules; choice should follow operating model and requirements.

That is the key insight:

> The agent must produce an evidence-backed operating model and control-plane design, not a pretty hierarchy diagram that hides ownership and governance gaps.

## Non-negotiable design rules

### 1. Start from operating model, ownership, compliance, workload segmentation, and subscription lifecycle before drawing hierarchy.

### 2. Separate platform services from application landing zones unless a documented exception justifies consolidation.

### 3. Require policy, RBAC, networking, monitoring, security, cost, and recovery controls to have owners and rollout paths.

### 4. Treat brownfield environments as migration programs with drift, exceptions, and remediation blast radius.

### 5. Do not claim production readiness from Microsoft Learn alone; require sampled configured-environment or design evidence.

## Minimal safe implementation flow

- Classify the ask: greenfield design, brownfield review, subscription vending, governance baseline, network topology, security baseline, or platform automation.
- Ground the architecture in Microsoft Learn landing zone design areas and principles.
- Collect sanitized design evidence or read-only configured-environment evidence for hierarchy, subscriptions, policies, RBAC, network, monitoring, and security posture.
- Identify required decisions, blockers, high-risk assumptions, and minimum viable next implementation step.
- Return verdict, evidence level, blockers, safe next actions, and open questions.

## High-risk assumptions to kill

- The recommended reference hierarchy is correct without business, compliance, and operating-model context.
- Subscription vending is optional for scale because teams can request subscriptions manually.
- Policy assignment equals governance if remediation and exception ownership are missing.
- A hub-spoke diagram proves network security, DNS, egress, or private endpoint readiness.
- Documentation proves tenant, policy, RBAC, subscriptions, or production readiness.

## Safe command/code verification targets

Verify against current docs and safe local or read-only tooling before use:

- Management group hierarchy, subscription placement, platform versus application landing zone boundaries, and environment separation.
- Policy and initiatives: assignment scopes, effects, exemptions, remediation identity, rollout phase, and exception owner.
- Identity and access: privileged roles, break-glass accounts, PIM, managed identities, and workload delegation model.
- Network and security: connectivity pattern, DNS, private endpoint strategy, egress control, Defender, logging, and incident ownership.
- Management and automation: monitoring baseline, backup/recovery, cost controls, IaC module choice, subscription vending, and drift detection.

## When to push back

- The user wants a hierarchy without operating model, ownership, or compliance requirements.
- Broad policy or RBAC changes are requested without staged rollout and rollback.
- Network or security readiness is inferred from a diagram.
- Sensitive tenant, subscription, network, or customer details are being pasted instead of sanitized evidence.
