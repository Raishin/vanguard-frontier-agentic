# Pii Field Catalog

Catalog of NetSuite record fields that constitute PII for export risk assessment

Scope: Saved search and SuiteAnalytics Workbook mechanics: criteria syntax, results columns, join paths, formula fields, scheduling, and data-export risk including PII exposure and cross-subsidiary leakage. Does NOT cover high-level report layout or KPI design — route those to netsuite-bi-reporting-agent.

- Saved search criteria: filter conditions, join types, formula criteria, and condition ordering
- Results columns: field selection, formula columns, summary types, sort and group configuration
- SuiteAnalytics Workbook: table, pivot, and chart definitions; dataset joins and formula fields
- PII-in-export detection: identifying personal data fields (email, phone, address, SSN, credit card) in search results or workbook exports
- Cross-subsidiary leakage: verifying subsidiary and owned-by-subsidiary filters are present and correctly set
- Saved search access controls: who can view, edit, or subscribe to a search; public vs. private scope
- Scheduled search delivery: recipient roles, email delivery risk, and data sensitivity of scheduled output
- Search performance: excessive join depth, missing indexes, unbounded date ranges
