# Adjacent Microsoft service expansion

Use this reference only when the user asks about a Microsoft service that is related to Entra identity posture but wasn't explicitly called out in the main skill text.

## Rule

Do not freeze the role to the currently named services.

When the user mentions another Microsoft service, first determine whether the real question is about:

1. **identity plane**
2. **licensing / entitlement**
3. **service-specific policy behavior**
4. **cross-service integration**

Then consult the official docs before concluding.

## Learning-and-evolution workflow

1. Identify the service name exactly.
2. Decide whether the service affects:
   - workforce identities
   - external identities
   - workload identities
   - app registrations / enterprise apps
   - Conditional Access / policy enforcement
   - licensing or bundle entitlement
3. Check the official Microsoft docs for that service family.
4. Distinguish:
   - “shares the same Entra tenant”
   - “uses the same sign-in plane”
   - “requires additional service licensing”
   - “has separate capacity / SKU / billing requirements”
5. Answer with explicit uncertainty if the tenant’s purchased licenses are unknown.

## High-value adjacent examples

### Microsoft 365

Use when the user asks whether a Microsoft 365 bundle covers Entra controls.

Typical checks:
- does the bundle include Entra ID P1?
- does it include Entra ID P2?
- does it merely include Entra Free?
- does it interact with Conditional Access or ID Protection?

### Microsoft Fabric / Power BI

Use when the user assumes tenant presence equals capacity rights or viewer rights.

Typical checks:
- capacity vs per-user licensing
- workspace type / SKU implications
- Entra tenant relationship versus Fabric usage rights

### Microsoft Intune

Use when Conditional Access and device state are being conflated.

Typical checks:
- whether device compliance assumptions depend on Intune licensing
- whether the user is asking about identity policy or device-management policy

### Microsoft Defender / Purview

Use when the user assumes Entra premium plans automatically include all risk and compliance integrations.

Typical checks:
- whether a specific signal comes from Defender
- whether the feature depends on a Defender or Purview add-on rather than plain Entra
- whether ID Protection quality depends on separately licensed Defender signals

### Microsoft Entra External ID

Use when B2B/B2C/guest or customer identity questions appear.

Typical checks:
- MAU-based or product-specific entitlements
- whether the question is workforce tenant access or customer identity

### Microsoft Entra Verified ID

Use when verifiable credentials or face-check style questions appear.

Typical checks:
- what is included in base Entra versus premium suite/add-on capabilities

### Microsoft Entra Workload ID

Use when service principals, app identities, GitHub Actions, or nonhuman access are involved.

Typical checks:
- workload identity premium features
- Conditional Access for workload identities
- ID Protection for workload identities

### Microsoft Entra Agent ID / AI agents

Use when the user is building or governing AI agents and assumes agent identities are just normal users or just normal service principals.

Typical checks:
- whether the question is about agent identities, agent users, or agent blueprints
- whether Conditional Access for Agent ID is preview-only or separately constrained
- whether agent risk depends on ID Protection for agents
- whether blueprint inheritance or custom security attributes change the authorization model
- whether the tenant is mixing agent governance questions with ordinary workload identity questions

## Safe response pattern

- “This service uses the same Entra tenant, but that does not by itself prove the needed feature rights.”
- “This looks like a cross-service licensing question, so I’m grounding it against the service’s official documentation before concluding.”
- “I can confirm the documented prerequisite, but I cannot confirm your tenant owns that license from the evidence provided.”

## Evidence refresh - 2026-06-04

- Agent identity guidance is now a first-class adjacent-service example: distinguish agent identities, agent users, sponsors, blueprints, Conditional Access for agents, ID Protection for agents, and Microsoft Agent 365 licensing before reusing ordinary human or workload identity conclusions.
