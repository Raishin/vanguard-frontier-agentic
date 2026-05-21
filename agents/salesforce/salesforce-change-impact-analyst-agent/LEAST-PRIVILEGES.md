# Least-privilege Salesforce posture for Salesforce Change Impact Analyst Agent

## Execution tier

**T0 — Static Review**

Rationale: `execution_tier: "static-review"` declared in `metadata.json`. This agent performs
pre-deployment change impact analysis from sanitized metadata manifests, package.xml files,
destructiveChanges.xml exports, and pipeline configuration excerpts. It never connects to any
org and never executes or validates a deployment.

## Identity model

No live identity required. This agent works from pasted sanitized excerpts only — SFDX project
structure descriptions, package.xml manifests, destructiveChanges.xml files, metadata dependency
graphs, API version references, and change-freeze calendar documentation. It never initiates an
OAuth flow and never establishes a connection to any Salesforce org or DevOps pipeline runtime.

## Run As account requirements

Not applicable. No Connected App, no service account, no OAuth client.

## MCP server binding

None. No MCP server is permitted for T0 agents.

## Blast-radius bound

This agent cannot initiate a deployment, execute a validation run against any org, modify
metadata in any environment, alter pipeline configurations, or approve any change request. Even
if an attacker fully controlled the agent's output, no deployment, no metadata change, and no
pipeline execution can be triggered as a direct result of this agent's execution. Impact
analysis findings are advisory; execution authority remains exclusively with a qualified human
operator.

## Refusal triggers

- [ ] Any request to connect to a live Salesforce org, a DevOps Center pipeline, or any CI/CD
      runtime to fetch live dependency data
- [ ] Any request that includes or asks the agent to process org credentials, session tokens,
      or API keys
- [ ] Any request to approve, initiate, or validate a deployment in any environment
- [ ] Any impact analysis request where the metadata manifest or destructiveChanges scope has
      not been provided in the conversation
- [ ] Any request that treats a change-freeze window violation as acceptable without documented
      emergency change authority evidence
- [ ] Any destructive change assessment that does not include a rollback plan and rollback owner

## Escalation path

All requests to execute a deployment, run a validation-only deploy, or make any live-org change
must be routed to **`salesforce-live-guard-agent`** with a named human decision owner, a
complete change envelope, and the impact analysis output from this agent as supporting evidence.

---

References: [Execution tiers](../../docs/execution-tiers.md) | [Salesforce agents README](../README.md)

## Validation checklist

Before submitting deployment artifacts for review by this agent:

- [ ] package.xml manifests identify metadata types and members, not org-specific record IDs
- [ ] destructiveChanges.xml scope is clearly documented and the rollback path for each destructive member is identified
- [ ] Pipeline YAML or CI/CD configuration excerpts have been sanitized to remove service-account tokens and environment secrets
- [ ] API version deprecation risks are scoped to the Salesforce release currently in the target org's API version
- [ ] Change-freeze calendar compliance is verified against the org's change management policy before submission

## Companion skill

`salesforce-devsecops-pipeline-skill` — use before invoking this agent to establish the
release pipeline security baseline. The skill covers destructive change risk categories,
API deprecation risk assessment, and change-freeze compliance patterns that this agent
applies when performing pre-deployment impact analysis.
