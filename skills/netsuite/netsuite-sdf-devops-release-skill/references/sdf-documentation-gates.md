# Sdf Documentation Gates

Required documentation artifact standards: README, ARCHITECTURE, CHANGELOG completeness criteria

Scope: SDF project structure correctness, deployment configuration review, and environment promotion governance. Validates manifest.xml completeness, deploy.xml ordering, customrole permission XML against the SDF permission catalog, and pre/post-deployment documentation requirements (README, ARCHITECTURE, CHANGELOG). Flags SuiteScript 1.0 unconverted code as a deployment blocker.

- SDF project structure: validate standard directory layout (FileCabinet/, Objects/, SuiteScripts/, Templates/), manifest.xml completeness, and object XML well-formedness
- Deployment configuration review: validate deploy.xml ordering, dependency declarations, and customdeploy tag correctness for the target environment
- Permission XML validation in deployment objects: cross-reference customrole permkey/permlevel against the 684-code SDF permission catalog (upstream dependency netsuite-sdf-roles-and-permissions)
- Environment promotion governance: confirm sandbox → staging → production promotion path is documented; flag direct-to-production deployments without sandbox evidence
- Documentation gate: verify required artifacts (README.md, ARCHITECTURE.md, CHANGELOG.md) exist and are not stale; confirm secrets and PII are redacted from generated docs
- SuiteScript version gate: flag SuiteScript 1.0 code in the project as a deployment blocker (migration urgency per upgrade path conventions)
- Audit evidence artifacts: confirm deployment records include change ticket reference, approver, rollback plan, and target environment documentation
