# Evidence Hierarchy

Full definition and decision rules for each evidence tier from LIVE_EVIDENCE to BLOCKED

Scope: Apply the Vanguard evidence hierarchy to every NetSuite claim and track drift between documented agent knowledge and Oracle NetSuite release milestones on a biannual cadence. Primary release-sensitive milestones: SOAP 2026.1 (new integrations must use REST+OAuth2), 2027.1 (new SOAP integrations blocked; new TBA-for-SOAP blocked), 2028.2 (all SOAP endpoints disabled).

- Evidence hierarchy labelling: LIVE_EVIDENCE, REPOSITORY_EVIDENCE, USER_PROVIDED, OFFICIAL_DOCUMENTATION, INFERENCE, UNVERIFIED, BLOCKED
- Biannual release-drift audit against NetSuite release milestones aligned to Oracle quarterly cadence
- SOAP removal plan milestone tracking: 2026.1 (new integrations must use REST+OAuth2), 2027.1 (new SOAP and new TBA-for-SOAP blocked), 2025.2 (last planned SOAP endpoint), 2028.2 (all SOAP endpoints disabled)
- TBA deprecation tracking: no new TBA integrations for SOAP/REST/RESTlets from 2027.1; existing TBA integrations unaffected
- Certification status tracking: flag coming-soon certifications (AI Specialist/Professional, BI & Reporting Professional) as UNVERIFIED until confirmed
- OAuth 2.0 sandbox isolation drift: track re-authorization requirements after sandbox refresh per evidence items 8a-8c
- Authentication method support matrix maintenance: OAuth 2.0 (REST/RESTlets/SuiteAnalytics), TBA (SOAP existing/REST/RESTlets), SOAP auth (user credentials removed at 2020.2 endpoint)
