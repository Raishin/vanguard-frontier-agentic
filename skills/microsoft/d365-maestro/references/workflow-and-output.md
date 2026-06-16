# Routing table and domain taxonomy

Use this reference when classifying a D365 task or selecting the right specialist.

## Domain taxonomy

| Domain | Keywords and signals |
|---|---|
| `d365-architecture` | solution architecture, solution blueprint, Success by Design, FastTrack, fit-gap, data model, integration design, dual-write, Dataverse, performance strategy |
| `d365-finance` | Dynamics 365 Finance, general ledger, accounts payable, accounts receivable, fixed assets, budgeting, financial reporting, chart of accounts, fiscal periods, posting configuration, tax |
| `d365-supply-chain` | Dynamics 365 Supply Chain Management, procurement, inventory, warehouse management, transportation, production, MRP, demand planning, source-to-pay, order management |
| `d365-business-central` | Dynamics 365 Business Central, SMB ERP, Business Central finance, Business Central supply chain, Business Central customization |
| `d365-customer-service` | Dynamics 365 Customer Service, case management, contact center, knowledge base, omnichannel, service agreements, SLA |
| `d365-field-service` | Dynamics 365 Field Service, work orders, scheduling, resource management, asset management, IoT integration, field operations |
| `d365-sales` | Dynamics 365 Sales, opportunity management, lead management, forecasting, territory management, revenue operations, sales process |
| `d365-customer-insights` | Dynamics 365 Customer Insights, Journeys, marketing automation, customer data platform, segments, real-time journeys |
| `d365-development` | FnO development, X++ extensions, D365 plugins, PCF controls, Power Platform extensions, SDK, customization patterns |
| `d365-integration` | Dual-write, Azure Logic Apps, OData, API integration, middleware, data entities, DIXF, electronic reporting, virtual entities |
| `d365-data-migration` | Data migration strategy, data entities, DIXF, cutover planning, data cleansing, migration testing, incremental migration |
| `d365-testing` | UAT, performance testing, regression testing, test scripts, go-live readiness, load testing |
| `d365-security` | D365 security roles, segregation of duties, role-based access, security model, audit logging, environment security |

## Full routing table

### Architecture and Solution Design

| Agent | Domain(s) | Use when… |
|---|---|---|
| `d365-business-applications-solution-architect` | d365-architecture | Designing or reviewing D365 solution architecture, Success by Design workshops, solution blueprint, integration design, or data model |

### Finance and Operations

| Agent | Domain(s) | Use when… |
|---|---|---|
| `d365-finance-functional-consultant-agent` | d365-finance | Configuring or troubleshooting D365 Finance: general ledger, AP/AR, fixed assets, budgeting, financial reporting, or posting configuration |
| `d365-supply-chain-functional-consultant-agent` | d365-supply-chain | Configuring or troubleshooting D365 Supply Chain Management: procurement, inventory, warehouse, transportation, or production |
| `d365-business-central-functional-consultant-agent` | d365-business-central | Implementing or supporting Dynamics 365 Business Central for SMB organizations |

### Customer Engagement

| Agent | Domain(s) | Use when… |
|---|---|---|
| `d365-customer-service-contact-center-consultant` | d365-customer-service | Designing or operating D365 Customer Service, omnichannel, case management, or contact center |
| `d365-field-service-operations-architect` | d365-field-service | Designing or operating D365 Field Service: work orders, scheduling, resource management, or asset management |
| `d365-sales-revenue-operations-architect` | d365-sales | Designing or operating D365 Sales: opportunity management, forecasting, territory management, or revenue operations |
| `d365-customer-insights-journeys-architect` | d365-customer-insights | Designing or operating Dynamics 365 Customer Insights, Journeys, or the real-time marketing module |

### Development and Integration

| Agent | Domain(s) | Use when… |
|---|---|---|
| `d365-fno-developer-extension-engineer` | d365-development | Writing X++ extensions, plugins, PCF controls, or custom D365 Finance & Operations development |
| `d365-integration-dual-write-architect` | d365-integration | Designing or troubleshooting D365 integrations: dual-write, Logic Apps, OData, DIXF, virtual entities, or electronic reporting |

### Implementation and Go-Live

| Agent | Domain(s) | Use when… |
|---|---|---|
| `d365-data-migration-cutover-lead` | d365-data-migration | Planning or executing D365 data migration strategy, DIXF data entities, cutover runbook, or incremental migration |
| `d365-test-performance-go-live-readiness-lead` | d365-testing | Planning or executing UAT, performance testing, regression testing, or go-live readiness assessment |

### Security

| Agent | Domain(s) | Use when… |
|---|---|---|
| `d365-security-segregation-of-duties-steward` | d365-security | Reviewing D365 security role design, segregation of duties conflicts, audit logging, or environment security posture |

## Live-guard gate protocol

Before routing to any live-guard agent (D365 production cutover, data migration to production, or posting-configuration changes), surface all three and wait for explicit written confirmation:

1. **Blast-radius assessment** — what data, environments, business processes, or users are affected if this goes wrong?
2. **Rollback path** — what is the tested rollback procedure and estimated recovery time?
3. **Explicit confirmation** — "I confirm I understand the blast radius and rollback path. Proceed."

If the user cannot supply a rollback path, recommend routing to `d365-business-applications-solution-architect` first to develop a cutover or rollback strategy.

## Success by Design gate check

Before routing to any implementation or go-live specialist, confirm:
- [ ] Solution blueprint completed and reviewed
- [ ] Data migration strategy defined and tested
- [ ] Security model and SoD conflicts reviewed (route to `d365-security-segregation-of-duties-steward` if not)
- [ ] Performance and load testing completed
- [ ] Cutover strategy and runbook reviewed

## Response shape

Every Maestro response begins with the routing header:
```
Route: <agent-name(s)>
Reason: <one sentence>
Mode: <single | parallel (N specialists) | live-guard-gate>
```
Followed by: dispatched specialist output (summarized), then recommended next actions.
