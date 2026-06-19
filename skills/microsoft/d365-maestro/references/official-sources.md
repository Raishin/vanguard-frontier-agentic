# Official sources

Use this reference only when you need source grounding for D365 service behavior or the detailed source list.

## Dynamics 365 documentation

Use these as starting points, not as proof of the user's live D365 environment:
- https://learn.microsoft.com/dynamics365/guidance/implementation-guide/success-by-design
- https://learn.microsoft.com/dynamics365/guidance/overview
- https://learn.microsoft.com/dynamics365/guidance/implementation-guide/overview
- https://learn.microsoft.com/dynamics365/supply-chain/supply-chain-management-welcome
- https://learn.microsoft.com/azure/architecture/solutions/dynamics-365-scenarios

## Grounding rule

Official documentation explains Dynamics 365 service behavior. It does not prove the user's current environment configuration, data migration state, cutover readiness, or customization scope. Prefer read-only D365 admin center evidence, LCS evidence, or sanitized user-provided evidence for current-state claims.

## Current documentation refresh (2026-06-16)

Service facts from official docs:
- Success by Design is Microsoft's prescriptive framework for Dynamics 365 implementations, based on the FastTrack program. It defines five stages: Discover, Initiate, Implement, Prepare, Operate. Key workshops include: solution blueprint, data migration strategy, cutover strategy, security model review, integration design, and performance strategy.
- D365 Finance and Supply Chain Management support integration via Azure Logic Apps middleware, dual-write for near-real-time Customer Engagement to Finance & Operations sync, and Azure Synapse for data warehousing.
- D365 Supply Chain Management covers source-to-pay, warehouse management, Confirmed Purchase Orders Workspace, and Copilot capabilities for enhanced decision-making.
- The Dynamics 365 and Azure-powered manufacturing sales framework uses D365 Sales as core CRM, Dataverse as the data layer, Power BI for analytics, Power Pages for external interfaces, and Azure integration services (Data Lake, Functions, Logic Apps, Synapse, Service Bus).

Review implications:
- D365 Maestro routing should choose the narrowest specialist based on domain evidence: Finance, Supply Chain, Business Central, Customer Service, Field Service, Sales, Customer Insights, FnO development, integration, data migration, testing, or security/SoD.
- Always check for Success by Design stage gate requirements before routing to implementation or go-live specialists.
- Do not centralize decisions without citing the evidence source and routing rationale.
