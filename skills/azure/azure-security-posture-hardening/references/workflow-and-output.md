# Workflow and Output Contract

## Workflow

1. **Scope the review**
   - Identify tenant, management group, subscription, resource group, workload, or service scope.
   - Separate platform controls from workload-local controls.
   - Identify whether the ask is greenfield hardening, brownfield remediation, or exception review.

2. **Map identity and secret flows**
   - List human admins, workload identities, service principals, and managed identities.
   - Challenge stored credentials, connection strings in code/config, and over-scoped service principals.
   - Prefer system-assigned or user-assigned managed identities when they reduce secret handling and blast radius.
   - Check whether Key Vault access uses Azure RBAC and whether permissions are scoped narrowly.

3. **Check network exposure and private access posture**
   - Identify internet-reachable management or data paths.
   - Challenge public endpoints for sensitive data paths by default.
   - Recommend private endpoints, Private Link, or tighter network restrictions when justified by data sensitivity, lateral-movement risk, or zero-trust requirements.
   - Do not force private access blindly; call out DNS, routing, operational, and cost implications.

4. **Review Key Vault posture**
   - Check whether secrets, keys, and certificates are centralized appropriately instead of embedded in apps or pipelines.
   - Check RBAC model, network restrictions, diagnostic logging, and access patterns.
   - Prefer vault-per-application-per-environment patterns unless there is a justified shared-services design.
   - Flag broad vault administrator access, uncontrolled secret sprawl, and missing monitoring.

5. **Review policy-enforced controls**
   - Check for Azure Policy assignments or equivalent guardrails that enforce the intended baseline.
   - Distinguish audit-only, deny, deployIfNotExists, and remediation-driven controls.
   - Look for missing controls around public exposure, diagnostics, encryption expectations, approved regions/SKUs, and identity hygiene.

6. **Review detection, audit, and logging coverage**
   - Check whether critical services emit diagnostics to the intended destination.
   - Check whether Key Vault logging/monitoring is enabled and retained in an auditable destination.
   - Call out when security recommendations lack detection coverage, ownership, or alert routing.
   - Separate “control exists” from “control is monitored.”

7. **Prioritize remediation safely**
   - Classify findings into urgent, near-term, and strategic.
   - Sequence changes to avoid breaking apps, pipelines, or operators.
   - For brownfield environments, recommend validate-first rollout steps and rollback expectations.

## Output contract

Return all of the following:

- **Scope and evidence summary**
  - reviewed scope,
  - evidence sources used,
  - important unknowns.
- **Current posture summary**
  - identity model,
  - secret handling model,
  - network exposure posture,
  - policy and monitoring posture.
- **High-risk findings**
  - issue,
  - why it matters,
  - likely blast radius.
- **Prioritized hardening plan**
  - urgent actions,
  - near-term actions,
  - strategic improvements.
- **Safe sequencing**
  - dependency notes,
  - validation steps,
  - rollback cautions.
- **Assumptions and evidence gaps**
  - what was inferred,
  - what must be verified before change approval.

## Eval gate

The skill output is only acceptable if it:

1. identifies identities, secret flows, and network exposure explicitly,
2. separates control-plane, data-plane, and policy/monitoring concerns,
3. recommends managed identities and least privilege where applicable,
4. addresses Key Vault posture, not just generic “use a vault” advice,
5. includes logging or diagnostic expectations,
6. prioritizes actions by risk and rollout safety,
7. states assumptions and missing evidence when live posture is incomplete.

Fail the response if it defaults to broad access, treats public exposure as harmless, ignores telemetry, or gives generic compliance prose without an operator action path.

## Safety notes

- Do not request or expose secrets, tokens, certificates, keys, tenant secrets, or customer data.
- Do not recommend `Owner`, subscription-wide `Contributor`, or broad vault admin access unless the user has justified the blast radius.
- Do not recommend public endpoints by default for sensitive services or secret-bearing paths.
- Do not present private endpoints as free or automatic; call out DNS, routing, and operational dependencies.
- Do not assume Azure Policy enforcement exists just because a standard is documented.
- Do not treat diagnostic settings as optional for sensitive services.
- If the task becomes a deep RBAC redesign, route to `azure-rbac-review` or a role-selection skill.
- If the task becomes a broader governance initiative rollout, route to a governance/policy guardrail skill.
