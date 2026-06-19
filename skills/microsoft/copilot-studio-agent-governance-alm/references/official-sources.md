# Official sources

Use this reference only when you need source grounding for Microsoft Copilot Studio governance, security, DLP, or ALM behavior.

## Microsoft Learn documentation

Use these as starting points, not as proof of the user's actual agent configuration, DLP posture, or ALM maturity:

- https://learn.microsoft.com/microsoft-copilot-studio/security-and-governance — Key concepts in Copilot Studio security and governance: geographic data residency, DLP controls, environment routing, standards certifications, generative AI publishing controls, Customer Lockbox. Core reference for governance posture assessment.
- https://learn.microsoft.com/microsoft-copilot-studio/admin-data-loss-prevention — Configure data policies (DLP) for Copilot Studio agents: connector classification, blocking unauthenticated usage, channel restrictions, knowledge source controls, and connector-level enforcement. DLP enforcement is in effect for all tenants since early 2025.
- https://learn.microsoft.com/microsoft-copilot-studio/guidance/sec-gov-intro — Manage Copilot Studio projects overview: links to the full governance and security series covering requirements capture, zoned governance, securing projects, testing strategy, ALM deployment, and compliance monitoring.
- https://learn.microsoft.com/microsoft-copilot-studio/guidance/sec-gov-phase1 — Capture governance requirements: stakeholder alignment, compliance review (GDPR, HIPAA), data protection and risk assessment, and restricting data sources.
- https://learn.microsoft.com/microsoft-copilot-studio/guidance/sec-gov-phase2 — Implement a zoned governance strategy: tenant-, environment-, and agent-level feature controls; maker access controls; Managed Environment requirements; DLP scoping per environment.
- https://learn.microsoft.com/microsoft-copilot-studio/guidance/sec-gov-phase3 — Secure Copilot Studio projects: virtual networks, IP firewall, continuous access evaluation, sharing rules, data residency, and enabling data movement restrictions across geographies.
- https://learn.microsoft.com/microsoft-copilot-studio/guidance/alm — Establish an ALM strategy: environment strategy, ALM golden rules, solution-based agent transport, environment variables, CI/CD options (Azure DevOps, GitHub Actions, Power Platform Pipelines), testing strategy, and Copilot Studio-specific ALM items that require post-deployment steps.
- https://learn.microsoft.com/microsoft-copilot-studio/authoring-solutions-overview — Create and manage solutions in Copilot Studio: solution-based agent creation, preferred solution configuration, pipeline deployment from Copilot Studio, and ring-deployment methodologies.
- https://learn.microsoft.com/microsoft-copilot-studio/guidance/sec-gov-phase5 — Monitor operations, compliance, and capacity: built-in analytics, transcript reviews, feedback tools, and iterative improvements for agent quality and safety.

## Grounding rule

Official documentation explains Copilot Studio governance and ALM behavior. It does not prove the user's actual DLP configuration, agent authentication posture, publishing scope, or ALM maturity. Prefer exported policy reports, sanitized admin center screenshots, or user-provided governance summaries for current-state claims. Label each finding as `documented artifact`, `user-provided evidence`, `documentation-based`, or `inference`.
