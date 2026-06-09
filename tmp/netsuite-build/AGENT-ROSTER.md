# NetSuite Agent Roster (Wave-1, 25) — finalized board

All IDs end `-agent`; companion skills end `-skill`. Board kept at 25 (full build) with
sharp, documented boundaries. `domain_key` feeds the maestro routing taxonomy.

## Layer 1 — Routing & Governance (Batch A)
| id | domain_key | companion skill | notes |
|---|---|---|---|
| netsuite-maestro-agent | (router) | — (companion_skills: []) | has README.md + routing taxonomy; NOT a routing domain |
| netsuite-live-org-mutation-guard-agent | (live_guard) | netsuite-live-operation-safety-skill | the `live_guards` entry; gate mode only, not a routing domain |
| netsuite-evidence-release-drift-agent | evidence-release-drift | netsuite-evidence-release-drift-skill | |
| netsuite-enterprise-architecture-agent | enterprise-architecture | netsuite-enterprise-architecture-skill | |
| netsuite-audit-controls-sox-agent | audit-controls-sox | netsuite-audit-controls-sox-skill | |

## Layer 2 — Cert-aligned specialists (Batches B, C)
Batch B:
| id | domain_key | companion skill | cert alignment | upstream reuse |
|---|---|---|---|---|
| netsuite-suitefoundation-agent | suitefoundation | netsuite-suitefoundation-skill | SuiteFoundation Specialist (available) | NO_ACTION |
| netsuite-administrator-agent | administrator | netsuite-administrator-skill | Administrator Professional (available) | NO_ACTION |
| netsuite-erp-consultant-agent | erp-consultant | netsuite-erp-consultant-skill | ERP Consultant Professional (available) | NO_ACTION |
| netsuite-financial-foundations-agent | financial-foundations | netsuite-financial-foundations-skill | Financial User / Accounting Professional (available) | NO_ACTION |
| netsuite-ai-foundations-agent | ai-foundations | netsuite-ai-foundations-skill | AI Foundations Associate (available); Specialist/Professional COMING SOON | NO_ACTION |

Batch C:
| id | domain_key | companion skill | cert alignment | upstream reuse |
|---|---|---|---|---|
| netsuite-bi-reporting-agent | bi-reporting | netsuite-bi-reporting-skill | BI & Reporting Associate/Specialist (available); BI&Reporting Professional COMING SOON | ADAPTED_WRAPPER netsuite-finance-analyst |
| netsuite-saved-searches-workbook-agent | saved-searches-workbook | netsuite-saved-searches-workbook-skill | BI & Saved Searches Professional (available) | NO_ACTION |
| netsuite-application-developer-agent | application-developer | netsuite-application-developer-skill | Application Developer Professional (available) | DEPENDENCY netsuite-suitescript-records-reference + netsuite-uif-spa-reference |
| netsuite-web-services-integration-agent | web-services-integration | netsuite-web-services-integration-skill | Web Services Developer Professional (available) | NO_ACTION |
| netsuite-suitecloud-developer-agent | suitecloud-developer | netsuite-suitecloud-developer-skill | SuiteCloud Developer Professional (available) | ADAPTED_WRAPPER netsuite-suitescript-upgrade |

## Layer 2 — Enterprise-role specialists (Batches D, E)
Batch D:
| id | domain_key | companion skill | upstream reuse |
|---|---|---|---|
| netsuite-identity-access-role-permission-agent | identity-access-role-permission | netsuite-identity-access-role-permission-skill | DEPENDENCY netsuite-sdf-roles-and-permissions |
| netsuite-sso-oauth-tba-agent | sso-oauth-tba | netsuite-sso-oauth-tba-skill | NO_ACTION |
| netsuite-sdf-devops-release-agent | sdf-devops-release | netsuite-sdf-devops-release-skill | ADAPTED_WRAPPER netsuite-sdf-project-documentation |
| netsuite-suitescript-secure-code-review-agent | suitescript-secure-code-review | netsuite-suitescript-secure-code-review-skill | ADAPTED_WRAPPER netsuite-owasp-secure-coding |
| netsuite-suiteflow-automation-agent | suiteflow-automation | netsuite-suiteflow-automation-skill | NO_ACTION |

Batch E:
| id | domain_key | companion skill | upstream reuse |
|---|---|---|---|
| netsuite-oneworld-multisubsidiary-agent | oneworld-multisubsidiary | netsuite-oneworld-multisubsidiary-skill | NO_ACTION |
| netsuite-data-governance-privacy-agent | data-governance-privacy | netsuite-data-governance-privacy-skill | NO_ACTION |
| netsuite-ai-connector-mcp-agent | ai-connector-mcp | netsuite-ai-connector-mcp-skill | DEPENDENCY netsuite-ai-connector-instructions |
| netsuite-integration-migration-agent | integration-migration | netsuite-integration-migration-skill | NO_ACTION (cross-ref REST/SOAP timeline) |
| netsuite-sandbox-nonproduction-governance-agent | sandbox-nonproduction-governance | netsuite-sandbox-nonproduction-governance-skill | NO_ACTION |

## Boundary notes (anti-overlap)
- **bi-reporting** = report/dashboard/KPI design & data-source semantics; **saved-searches-workbook**
  = saved search + SuiteAnalytics Workbook mechanics, criteria/results/PII-in-export.
- **web-services-integration** = SuiteTalk REST/SOAP record API design; **integration-migration**
  = end-to-end integration architecture + SOAP→REST migration program; **sso-oauth-tba**
  = auth/identity (OAuth2/TBA/SSO/SAML) only.
- **identity-access-role-permission** = NetSuite roles/permissions/SoD; **sso-oauth-tba** = the
  authentication mechanisms. Cross-escalate, don't duplicate.
