# Adr Template

Structured architecture decision record template with rationale, alternatives, tradeoffs, and risk-rating fields

Scope: Evaluate NetSuite architectural decisions against Oracle best practices, zero-trust boundaries, least-privilege design, and the SOAP-to-REST migration timeline. Produce opinionated architecture assessments with risk-rated findings and safe next actions for large-scale implementations spanning multiple subsidiaries, integration suites, and development lifecycle stages.

- SuiteCloud platform architecture: SuiteScript 2.1 script-type selection and governance, SDF project structure, Suitelet/RESTlet/portlet design patterns
- Integration topology: REST web services vs. RESTlet vs. SuiteAnalytics Connect selection; OAuth 2.0 vs. TBA authentication posture; SOAP migration roadmap planning aligned to 2026.1/2027.1/2028.2 milestones
- OneWorld multi-subsidiary design: intercompany transactions, consolidated reporting topology, subsidiary-scoped role and permission architecture
- Customization strategy: custom records, custom fields, SuiteBuilder configuration vs. SuiteScript code decisions, technical debt assessment
- SDF project organization: bundle dependencies, object deployment ordering, environment promotion pipelines, sandbox-to-production architecture
- AI Connector MCP integration architecture: tool selection (Reports vs. Saved Searches vs. Record Ops vs. Custom SuiteQL), scope boundaries, permission posture
- Architecture decision record (ADR) production: rationale, alternatives, risk tradeoffs, and review date
- Cross-domain conflict arbitration when multiple specialist agents disagree on design approach
