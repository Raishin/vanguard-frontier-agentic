# Workspace Segmentation And The 50–100 Guidance

Databricks guidance on workspace counts, legitimate segmentation drivers, and when to consolidate under catalogs and schemas instead.

- Databricks recommends not exceeding roughly 50–100 workspaces per account without strong justification. The organisation should consolidate under catalogs and schemas rather than create a workspace per team or project.
- Legitimate reasons to segment into separate workspaces are: environment isolation (dev/staging/prod), regulated-data isolation (PII/financial in separate workspace), complete business-unit isolation, data residency (GDPR/sovereign-cloud requirements), and feature capability (serverless in one region, classic in another).
- Workspace segmentation decisions should map explicitly to one or more of these drivers and should be documented as part of the account architecture.
- Catalog and schema organisation is the default pattern for team, project, and data-domain separation within a single workspace; segmentation into multiple workspaces is the exception, not the rule.
- Serverless and classic compute can coexist in the same workspace (via separate clusters) or across workspaces; the choice depends on data classification and cost allocation intent.
