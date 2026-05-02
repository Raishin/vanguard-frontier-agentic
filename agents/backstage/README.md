# 🎭 Backstage Agents

<p align="center">
  <span style="font-size:3.5em">🎭</span>
</p>

Backstage agent catalog for this marketplace.

## 🧱 Agent tiers

| Tier | Purpose | Default access | Live cluster mutation |
|---|---|---|---|
| Review agents | Audit Backstage Scaffolder templates, plugin configurations, RBAC policy, and catalog entity posture | read-only | not allowed |

## 📋 Scaffolder template review agents

| Agent | Primary use | Default live posture | Must refuse when |
|---|---|---|---|
| `backstage-scaffolder-template-review-agent` | Review Backstage Scaffolder software templates for action blast-radius, input parameter injection, RBAC gate coverage, integration secret scope, catalog entity poisoning via `catalog:register`, and output stanza exposure | read-only | — |

## 🛡️ Operating note

- Scaffolder templates without RBAC policy (`permission: {rule: 'HAS_TAG', params: {tag: 'platform-internal'}}`) allow any Backstage user to trigger provisioning actions including Terraform apply, Kubernetes RBAC changes, and cloud resource creation
- `fetch:template` with `cookiecutterCompat: true` and unvalidated `{{ cookiecutter.values }}` renders arbitrary user input — template injection risk
- `github:repo:create` action using a GitHub App integration grants Backstage the ability to create repos in the org; verify which organizations the App is installed on
- `catalog:register` with `optional: false` and user-controlled entity YAML path allows users to register arbitrary entities including those with `kubernetes.io/` annotations pointing to cluster resources

## 📦 Install

```bash
# Install Backstage Scaffolder template review agent
npx vfa-export-agents --platform claude-code --agents backstage-scaffolder-template-review-agent --repo .

# Install all Kubernetes developer platform agents
npx vfa-export-agents --platform claude-code --role kubernetes-developer-platform-engineer --repo .
```
