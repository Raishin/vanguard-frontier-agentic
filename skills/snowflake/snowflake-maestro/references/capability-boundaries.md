# Capability Boundaries

What this board can and cannot establish, and the four concepts the router must never collapse into each other. Load when a request seems to belong to the board but the evidence to answer it does not exist.

## Four concepts that share vocabulary

- **Human organizational persona** — architect, administrator, security engineer, data engineer, analyst, FinOps practitioner, governance steward, product owner. Describes who is asking. It never selects the agent on its own: a data engineer asking about a masking policy still routes to governance.
- **Snowflake authorization role** — system-defined account roles, custom account roles, database roles, application roles, and SNOWFLAKE database roles. Describes what a principal may do in the account. It never describes what an agent may do.
- **Agent responsibility** — the failure domain, evidence set, and decision rights this board assigns. This is what routing selects on.
- **Agent runtime privilege** — read-only, local repository writes, or approval-gated live mutation. Independent of all three above. Every review agent on this board is read-only regardless of how privileged the domain it reasons about is.
- The fact that `SECURITYADMIN` can manage grants does not mean a security agent should execute grants. The fact that `ACCOUNTADMIN` can perform an operation never justifies assigning `ACCOUNTADMIN` to an automated identity.

## What this board cannot establish

- This board cannot prove what any account has configured without account evidence. Documentation establishes supported platform behaviour only.
- It cannot confirm an edition, a cloud, a region, a behaviour-change bundle state, or a preview-feature enablement from the request text.
- It cannot confirm that a control operated for an audit period from the fact that the control exists today.
- It cannot confirm the strong-authentication enforcement state of an account from the calendar date; the rollout runs in windows and its effect is per-account and per-user-type.
- It cannot price a workload. Rate cards, contract terms, and regional pricing are commercial facts the customer holds; the board reasons in credits and unit economics and labels currency figures `ESTIMATE` with a stated method.
