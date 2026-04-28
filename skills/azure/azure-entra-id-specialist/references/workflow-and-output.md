# Workflow and output contract

Use this reference only when you are performing the full Entra review.

## Workflow

1. **Scope the target**
   - Confirm whether the question is about users, admins, workload identities, app registrations, external identities, or mixed.
   - Confirm whether the real problem is sign-in control, privileged access, workload access, or governance.
   - Confirm whether the issue is tenant-wide, one role family, one app, or one policy set.

2. **Establish evidence level**
   - Use live Azure MCP evidence when available.
   - Otherwise use official docs plus sanitized user evidence.
   - Explicitly label unknowns.

3. **Check licensing and service entitlements when relevant**
   - Determine whether the user is asking about feature rights, not only technical configuration.
   - Distinguish Azure baseline, Microsoft 365 bundle inclusion, Entra premium plans, workload identity premium features, and Fabric capacity/per-user rights.
   - Distinguish Entra tenant identity from adjacent service entitlements such as Intune compliance dependencies, Microsoft Defender signal prerequisites, Purview or Fabric service rights, External ID billing, Verified ID premium add-ons, and agent identity preview capabilities.
   - If tenant licensing is unproven, mark the answer as licensing-conditional instead of assuming entitlement.

4. **Learn before concluding on adjacent services**
   - If the user mentions another Microsoft service, do not answer from brand association alone.
   - Check whether the service merely shares the Entra tenant, depends on Intune/Defender/Purview/Fabric-specific licenses, or introduces a separate identity primitive such as agent identities.
   - Prefer official Microsoft documentation over memory for cross-service claims.

5. **Stress-check the identity control posture**
   - Conditional Access scope, exclusions, and lockout safety
   - MFA/SSPR/authentication-method registration and abuse resistance
   - risky-user / risky-sign-in handling and identity protection posture
   - app-registration, enterprise-app, and service-principal ownership and privilege shape
   - workload identity and managed-identity control boundaries
   - agent identity, agent user, and blueprint control boundaries when AI agents are in scope
   - break-glass safety and recovery paths

6. **Check adjacent roles the user may be missing**
   - **Azure Identity Governance Review** when the problem narrows specifically to PIM, access reviews, entitlement management, and standing-versus-eligible access.
   - **Azure RBAC Review** when the dominant issue is Azure resource authorization scope rather than Entra tenant identity controls.
   - **Azure Security Posture Hardening** when the identity question becomes part of a broader Azure security program review.

## Output contract

Use this structure:

1. **Verdict**
2. **Evidence level**
3. **Key findings**
4. **Safest next actions**
5. **Open questions**
