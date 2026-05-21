# Least-privilege Salesforce posture for Salesforce DevOps Release Agent

## Execution tier

**T0 — Static Review**

Rationale: `execution_tier: "static-review"` declared in `metadata.json`. This agent reviews
pipeline configurations, sandbox strategy documents, metadata deployment plans, scratch org
definitions, unlocked package version strategies, and release gate checklists from sanitized
artifacts. It never executes sf CLI commands against any org and never invokes the Metadata API.

## Identity model

No live identity required. This agent works from pasted sanitized excerpts only — SFDX
project.json files, pipeline YAML definitions (GitHub Actions, GitLab CI, Gearset, Copado
pipeline configurations), package.xml manifests, scratch org definition JSON, release plan
documents, and rollback procedure descriptions. It never initiates an OAuth flow and never
establishes a connection to any Salesforce org or CI/CD runtime.

## Run As account requirements

Not applicable. No Connected App, no service account, no OAuth client.

## MCP server binding

None. No MCP server is permitted for T0 agents.

## Blast-radius bound

This agent cannot execute deployments, trigger pipeline stages, create or refresh sandboxes,
publish unlocked package versions, modify DevOps Center work items, or approve release gates.
Even if an attacker fully controlled the agent's output, no deployment runs, no sandbox is
created or refreshed, and no pipeline stage is triggered as a direct result of this agent's
execution. All release findings are advisory; execution authority remains with a qualified human
release manager using a separately credentialed toolchain.

## Refusal triggers

- [ ] Any request to connect to a live Salesforce org, execute `sf project deploy start`, or
      invoke the Metadata API against any org
- [ ] Any request that includes or asks the agent to process org credentials, session tokens,
      Connected App client secrets, or pipeline service-account tokens
- [ ] Any request to approve, initiate, or execute a deployment or environment promotion
      without dry-run validation output and rollback plan
- [ ] Any release review that treats change sets as the default deployment pattern without a
      documented migration justification
- [ ] Any sandbox strategy that does not include data masking requirements for full or partial
      copy sandboxes
- [ ] Any request to approve a change freeze exception without documented emergency change
      authority evidence

## Escalation path

All requests to execute deployments, run validation-only deploys, create or refresh sandboxes,
or make any live-org release operation must be routed to **`salesforce-live-guard-agent`** with
a named human decision owner and a complete change envelope including dry-run output.

---

References: [Execution tiers](../../docs/execution-tiers.md) | [Salesforce agents README](../README.md)

## Validation checklist

Before submitting DevOps and release artifacts for review by this agent:

- [ ] Pipeline YAML or configuration files have been sanitized to remove service-account tokens, secrets, and environment-specific credentials
- [ ] SFDX project.json and package.json files identify dependencies and structure, not registry auth tokens
- [ ] Scratch org definition JSON is a generic template — no org-specific overrides or connection settings
- [ ] Release plan documents describe environments by type (production, full-copy sandbox, developer sandbox) rather than by org URL or ID
- [ ] Rollback procedure documentation identifies the responsible rollback owner by role, not by personal name with contact details

## Companion skill

`salesforce-release-readiness-skill` — use before invoking this agent to run the standard
release readiness checklist. The skill covers sandbox strategy requirements, deployment gate
criteria, rollback readiness verification, and change-freeze compliance that this agent
evaluates in submitted pipeline configurations and release plans.

## sf CLI example — login with minimum scopes

```bash
sf org login web \
  --instance-url https://login.salesforce.com \
  --scopes "api refresh_token" \
  --set-default
```

This example is shown for reference only. T0 agents never execute this command. If a
T1-or-above upgrade is evaluated for this agent, the Connected App must be created with
exactly these scopes and the org allowlist must be enforced before any CLI invocation.
