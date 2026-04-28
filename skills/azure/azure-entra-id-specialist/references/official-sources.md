# Official sources

Use this reference only when you need the detailed source list or need to ground a specific claim.

## Azure MCP

- Azure MCP overview  
  https://learn.microsoft.com/en-us/azure/developer/azure-mcp-server/overview
- Azure MCP concepts  
  https://learn.microsoft.com/en-us/azure/developer/azure-mcp-server/concepts
- Azure MCP tools overview  
  https://learn.microsoft.com/en-us/azure/developer/azure-mcp-server/tools/

## Microsoft Entra ID

- What is Microsoft Entra?  
  https://learn.microsoft.com/en-us/entra/fundamentals/what-is-entra
- What is Microsoft Entra ID Governance?  
  https://learn.microsoft.com/en-us/entra/id-governance/identity-governance-overview
- What is Microsoft Entra Privileged Identity Management?  
  https://learn.microsoft.com/en-us/entra/id-governance/privileged-identity-management/pim-configure
- Protect security info registration with Conditional Access policy  
  https://learn.microsoft.com/en-us/entra/identity/conditional-access/policy-all-users-security-info-registration
- Conditional Access: Users, groups, agents, and workload identities  
  https://learn.microsoft.com/en-us/entra/identity/conditional-access/concept-conditional-access-users-groups
- What are workload identities?  
  https://learn.microsoft.com/en-us/entra/workload-id/workload-identities-overview
- Securing workload identities  
  https://learn.microsoft.com/en-us/entra/id-protection/concept-workload-identity-risk
- Conditional Access for Agent ID (Preview)  
  https://learn.microsoft.com/en-us/entra/identity/conditional-access/agent-id
- Manage agent identities in your organization  
  https://learn.microsoft.com/en-us/entra/agent-id/manage-agent-identities-admin
- Microsoft Entra security for AI overview  
  https://learn.microsoft.com/en-us/entra/agent-id/security-for-ai-overview
- Microsoft Entra ID Governance licensing fundamentals  
  https://learn.microsoft.com/en-us/entra/id-governance/licensing-fundamentals
- Microsoft Entra licensing  
  https://learn.microsoft.com/en-us/entra/fundamentals/licensing
- What is Conditional Access?  
  https://learn.microsoft.com/en-us/entra/identity/conditional-access/overview
- Features and licenses for Microsoft Entra multifactor authentication  
  https://learn.microsoft.com/en-us/entra/identity/authentication/concept-mfa-licensing
- What is Microsoft Entra ID Protection?  
  https://learn.microsoft.com/en-us/entra/id-protection/overview-identity-protection
- Microsoft Entra service description  
  https://learn.microsoft.com/en-us/office365/servicedescriptions/azure-active-directory

## Cross-service licensing examples

- Azure integration with Microsoft 365  
  https://learn.microsoft.com/en-us/microsoft-365/enterprise/azure-integration
- Understand Microsoft Fabric licenses  
  https://learn.microsoft.com/en-us/fabric/enterprise/licenses
- Buy a Microsoft Fabric subscription  
  https://learn.microsoft.com/en-us/fabric/enterprise/buy-subscription
- Learn about Conditional Access and Intune  
  https://learn.microsoft.com/en-us/intune/device-security/conditional-access-integration/overview
- Require device compliance with Conditional Access  
  https://learn.microsoft.com/en-us/entra/identity/conditional-access/policy-all-users-device-compliance
- Microsoft Defender service description  
  https://learn.microsoft.com/en-us/office365/servicedescriptions/microsoft-365-service-descriptions/microsoft-365-tenantlevel-services-licensing-guidance/microsoft-defender-service-description
- Microsoft Entra External ID overview  
  https://learn.microsoft.com/en-us/entra/external-id/external-identities-overview
- What are workload identities?  
  https://learn.microsoft.com/en-us/entra/workload-id/workload-identities-overview
- What is Microsoft Entra?  
  https://learn.microsoft.com/en-us/entra/fundamentals/what-is-entra

## Grounded insights used by this skill

- Existing identity governance controls do not prove broader Entra ID posture is sound.
- Conditional Access and registration controls can create lockout risk if exclusions and emergency access are careless.
- Workload identities and app registrations need the same least-privilege scrutiny as human admins.
- Tenant identity, Microsoft 365 bundle rights, and Fabric capacity rights are related but not interchangeable.
- Adjacent Microsoft services can share the same tenant while still having separate licensing, capacity, or premium-feature gates.
- Device-compliance Conditional Access can depend on Intune setup and compliance policy existence, not just Entra policy authoring rights.
- Some ID Protection detections and AI-agent protections depend on adjacent Microsoft services or preview-specific capabilities, not just base Entra branding.
