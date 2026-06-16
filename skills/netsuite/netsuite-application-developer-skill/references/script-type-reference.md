# Script Type Reference

SuiteScript 2.x script type entry-point and governance limit quick reference

Scope: SuiteScript 2.x code quality, script-type correctness, SuiteFlow workflow logic, SuiteBuilder custom record and form design, and UIF SPA component architecture. Depends on netsuite-suitescript-records-reference and netsuite-uif-spa-reference as upstream reference skills (Oracle UPL-1.0).

- SuiteScript 2.x: script type selection (ClientScript, UserEventScript, MapReduceScript, ScheduledScript, Suitelet, RESTlet, MassUpdateScript, WorkflowActionScript), entry-point correctness, module usage
- Governance limit awareness: synchronous vs. asynchronous script limits, N/search usage limits, N/record load patterns
- SuiteFlow workflow design: trigger conditions, action correctness, approval routing logic, workflow action scripts
- SuiteBuilder customizations: custom record type design, custom field configuration, form layout, custom segments
- UIF SPA component review: @uif-js/core and @uif-js/component API correctness, state management patterns, DataGrid and Form component usage
- Script deployment configuration: record type binding, run-as configuration, deployment status
- Error handling and logging patterns in SuiteScript 2.x
- Script upgrade readiness: identifying SuiteScript 1.0 patterns requiring migration (escalate to netsuite-suitecloud-developer-agent for full SDF migration)
