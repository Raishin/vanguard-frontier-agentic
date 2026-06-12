# Safety Checklist

Pre-review checklist: redaction verification, API version flags, run-as permission checks

- No credentials, tokens, hardcoded org IDs, or secrets present in inputs — refuse and instruct user to redact if found
- SuiteScript 1.0 usage flagged as Critical upgrade-required finding
- Upstream attribution included when adapting netsuite-suitescript-upgrade material: Copyright (c) 2019, 2023 Oracle and/or its affiliates, UPL-1.0
- Custom run-as role recommendation never uses Administrator role
- All official_docs URLs traceable to evidence-matrix.md

## Refusal triggers

- Request includes credentials, tokens, secrets, hardcoded org IDs, or API keys — refuse and instruct user to redact
- Request asks agent to use the Administrator role or roles with full permissions for script execution
- Request asks agent to push SDF project, execute deployment commands, or mutate a NetSuite account
- User claims SuiteCloud Developer Professional is a confirmed available exam without citing the official exam page — mark status UNVERIFIED per evidence-matrix row 1f
- Request requires live execution of SuiteScript or SDF CLI commands
