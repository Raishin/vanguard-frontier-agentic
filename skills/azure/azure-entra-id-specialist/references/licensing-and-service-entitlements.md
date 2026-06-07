# Licensing and service entitlements

Use this reference only when the answer depends on whether the tenant is actually entitled to a Microsoft feature.

## Rule

Do not treat feature existence as proof of tenant entitlement.

When the user asks whether a control **can** be used, whether it is **included**, or whether one Microsoft product **covers** another, separate:

1. **technical capability**
2. **licensing prerequisite**
3. **service-specific entitlement**

If licensing is unknown, say so explicitly.

## High-value examples

### Example 1: Microsoft Azure / Entra baseline

- Microsoft Entra ID **Free** is included with Microsoft cloud subscriptions such as **Microsoft Azure** and **Microsoft 365**.
- That does **not** mean P1, P2, Identity Governance, or ID Protection are included.

Use this when the user assumes “we have Azure, so we have the Entra premium features.”

### Example 2: Conditional Access

- Microsoft documents **Conditional Access** as requiring **Microsoft Entra ID P1**.
- Microsoft also documents that **Microsoft 365 Business Premium** customers can use Conditional Access features.
- Risk-based policies require **Microsoft Entra ID Protection**, which is a **P2** feature.

Use this when the user assumes all Conditional Access features are equivalent.

### Example 3: PIM and identity governance

- Microsoft documents **Privileged Identity Management (PIM)** as a **P2 / Identity Governance / Entra Suite** capability.
- Identity Governance capabilities may have more specific licensing requirements than base Entra plans, including scenarios where “who can request” or “who is reviewed” affects licensing scope.

Use this when the user assumes PIM is included because they already have P1.

### Example 4: Workload identities

- Microsoft documents **Microsoft Entra Workload ID** separately.
- Some workload identity protections, such as risk reporting and Conditional Access for workload identities, have their own premium licensing constraints.

Use this when the user assumes service principals inherit all user-based Entra licensing rights.

### Example 5: Microsoft 365 bundle examples

- Microsoft documents **Entra ID P1** as included in **Microsoft 365 E3**, **F1**, **F3**, and **Business Premium**.
- Microsoft documents **Entra ID P2** as included in **Microsoft 365 E5** and certain defender/purview suites.

Use this when the user asks whether a Microsoft 365 bundle is enough for Entra controls.

### Example 6: Microsoft Fabric examples

- Microsoft Fabric runs in a **Microsoft Entra tenant**.
- Fabric collaboration depends on both **capacity** and **per-user licensing** in documented scenarios.
- Fabric examples are useful when the user assumes “tenant presence” equals “feature entitlement” across services.

Use this when the user is mixing Entra tenant identity, Power BI/Fabric capacities, and user-license assumptions.

### Example 7: External ID

- Microsoft documents **External ID** with its own service and pricing shape.
- External identity entitlement should not be inferred from workforce Entra premium licensing alone.

Use this when the user mixes workforce identity controls with customer or guest identity assumptions.

### Example 8: Verified ID and Entra Suite extras

- Microsoft documents that some Entra family capabilities sit in the broader **Entra Suite** or have premium add-on distinctions.
- Do not assume “it is part of Entra” means it is covered by the tenant’s current Entra ID plan.

Use this when the user assumes every Entra-branded capability is included with P1 or P2.

### Example 9: Intune-backed Conditional Access is not just "Entra"

- Microsoft documents that Conditional Access policies requiring compliant devices depend on **Intune compliance posture** and can fail if no compliance policy exists.
- A tenant can have Conditional Access rights without having the device-management setup or licensing needed for the device-compliance control to work as intended.

Use this when the user assumes device-compliance-based access control is purely an Entra switch.

### Example 10: ID Protection can depend on separate Defender signals

- Microsoft documents that some Microsoft Entra ID Protection detections rely on **Microsoft Defender** products and their licensing.
- Do not assume all risk detections are available just because the tenant has Entra ID P2.

Use this when the user assumes Entra P2 alone guarantees every risk signal or automated protection path.

### Example 11: Agent ID and AI agent controls have their own prerequisites

- Microsoft documents **Conditional Access for Agent ID (Preview)** and related agent-governance features separately from ordinary user and workload identity controls.
- Do not assume AI agents inherit the same control surface, object model, or licensing behavior as human users.

Use this when the user is designing AI agents and assumes ordinary Entra patterns automatically cover agent identities.

## Minimum licensing-check workflow

1. Identify the exact feature the user cares about.
2. Identify whether the question is about:
   - Entra tenant capability
   - Microsoft 365 bundle inclusion
   - Azure subscription baseline
   - Fabric capacity / per-user access model
   - Intune-backed device compliance dependency
   - Defender or Purview signal dependency
   - workload identity premium features
   - agent identity preview or AI-governance capability
   - external identity entitlement
   - Entra Suite or service-specific premium add-ons
3. Check official licensing docs.
4. State one of:
   - **confirmed licensed prerequisite from docs**
   - **confirmed not included from docs**
   - **licensing unknown from provided evidence**
5. Avoid “you can just use X” unless the prerequisite is proven.

## Safe phrasing examples

- “Documentation says this feature requires Entra ID P1, but I do not know whether your tenant has that license.”
- “Business Premium includes Conditional Access, but risk-based Conditional Access depends on P2-backed ID Protection.”
- “Fabric uses the same Entra tenant, but user rights and capacity rights are separate from Entra feature entitlements.”

## Evidence refresh - 2026-06-04

- Microsoft Learn documents separate prerequisites for agent security features: Conditional Access for agents requires Entra ID P1, ID Protection for agents requires Entra ID P2, ID Governance for agents requires Entra ID P1, and network controls for agents require Global Secure Access licensing. Treat these as documented prerequisites, not proof of tenant entitlement.
