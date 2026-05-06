---
name: backstage-scaffolder-template-review
description: Use this skill when reviewing Backstage Scaffolder software templates. Trigger when the user asks whether a template is safe for developer self-service, whether template RBAC gates are in place, whether input parameters are validated, whether a step action has excessive blast radius, or whether template outputs expose secrets.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-05"
  category: delivery
---

# Backstage Scaffolder Template Review

## Purpose

Review Backstage Scaffolder `Template` kind resources for action blast-radius, input parameter injection risk, RBAC permission gate coverage, integration secret scope, catalog entity poisoning via `catalog:register`, and plaintext secret exposure in `output:` stanzas. Backstage Scaffolder gives developers a curated UI to trigger powerful backend actions — without RBAC gates and input validation, every authenticated developer effectively has write access to whatever the Scaffolder integration credentials can reach.

## Lean operating rules

- Prefer user-provided sanitized Template YAML as primary evidence; official Backstage docs are the authoritative fallback.
- Treat any `steps:` action that provisions real cloud infrastructure (Terraform, Crossplane CRD apply, CloudFormation deploy, `kubectl apply`) with no RBAC permission gate as a CRITICAL finding.
- Treat input parameters flowing unsanitized into `publish:github.repoUrl`, file-path actions, or shell-exec actions as a HIGH finding — path traversal and injection are realistic.
- Treat `publish:github` with `visibility: public` as the default or without an `allowedHosts` constraint as a HIGH finding.
- Treat `output:` stanzas exposing plaintext generated credentials, connection strings, or API keys in the Backstage UI as a HIGH finding.
- Treat the absence of `@backstage/plugin-permission-backend` policies for infrastructure-provisioning templates as a HIGH finding — any authenticated Backstage user can trigger them.
- Treat `catalog:register` accepting arbitrary user-supplied YAML without server-side entity schema validation as a MEDIUM finding — catalog poisoning overwrites ownership and lifecycle metadata.
- Keep the answer scoped: report what was reviewed, the evidence level, and exactly which steps or fields triggered each finding.

## References

Load these only when needed:
- [Workflow and output contract](references/workflow-and-output.md)

## Response minimum

- Scoped target (Template `metadata.name`) and evidence level
- Each `steps:` action type and its provisioning blast radius
- Input parameter validation gaps (missing `maxLength`, `pattern`, `enum`)
- RBAC permission gate verdict (present / absent / partial)
- Integration secret scope assessment
- `output:` stanza exposure assessment
- Safe next actions and open questions
