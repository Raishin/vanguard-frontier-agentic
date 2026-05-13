# CI/CD Enforcement Pattern

> How to enforce VFA live-guard and review agents at the pipeline layer so guardrails run without developer opt-in.

## Why Pipeline Enforcement

Developer self-service installs work for exploration. They fail at Fortune 50 scale because:

- Engineers bypass tools they find inconvenient.
- Contractors and new hires never install anything.
- Audit evidence disappears when a developer's laptop is wiped.

Pipeline enforcement solves all three: the guardrail runs at the merge gate, regardless of who authored the change. The evidence artifact is stored in the pipeline run, not on a developer workstation.

---

## Enforcement Architecture

```
PR opened
    │
    ▼
[CI: vfa-review-gate]          ← BEFORE layer: review agents
    │
    ▼
Merge to main
    │
    ▼
[CD: vfa-live-guard-gate]      ← AT layer: live-guard agents
    │
    ▼
Deployment executes
    │
    ▼
[CD: vfa-verify-step]          ← AFTER layer: verification + evidence artifact
```

Each gate uses the same `vfa-export-agents` CLI to install the correct agent harness into the pipeline runner, then invokes it via the platform's native agent execution mechanism.

---

## GitHub Actions

### Prerequisites

```bash
npm install -g @raishin/vanguard-frontier-agentic
```

Or pin via `package.json`:

```json
{
  "devDependencies": {
    "@raishin/vanguard-frontier-agentic": "^1.8.0"
  }
}
```

### RBAC Review Gate (PR check — BEFORE layer)

```yaml
# .github/workflows/vfa-rbac-review.yml
name: VFA RBAC Review Gate

on:
  pull_request:
    paths:
      - 'k8s/**'
      - 'manifests/**'
      - 'terraform/**'
      - 'iam/**'

jobs:
  rbac-review:
    runs-on: ubuntu-latest
    permissions:
      contents: read
      pull-requests: write

    steps:
      - uses: actions/checkout@v4

      - name: Install VFA agents
        run: |
          npx --yes @raishin/vanguard-frontier-agentic \
            vfa-export-agents \
            --platform codex \
            --role cloud-security-engineer \
            --repo .

      - name: Run Kubernetes RBAC Review
        uses: anthropics/claude-code-action@beta
        with:
          agent: .codex/agents/kubernetes-rbac-review-agent.toml
          prompt: |
            Review all RBAC manifest changes in this PR.
            Output must include: verdict, evidence_level, blockers, safe_next_actions, open_questions.
            If verdict is 'blocked', exit non-zero.
        env:
          ANTHROPIC_API_KEY: ${{ secrets.ANTHROPIC_API_KEY }}

      - name: Upload evidence artifact
        if: always()
        uses: actions/upload-artifact@v4
        with:
          name: vfa-rbac-evidence-${{ github.run_id }}
          path: vfa-evidence.json
```

### Live RBAC Mutation Gate (deployment — AT layer)

```yaml
# .github/workflows/vfa-live-rbac-guard.yml
name: VFA Live RBAC Mutation Guard

on:
  push:
    branches: [main]
    paths:
      - 'k8s/rbac/**'

jobs:
  live-rbac-guard:
    runs-on: ubuntu-latest
    environment: production        # requires manual approval in GitHub Environments

    steps:
      - uses: actions/checkout@v4

      - name: Configure kubectl
        uses: azure/setup-kubectl@v3

      - name: Install VFA agents
        run: |
          npx --yes @raishin/vanguard-frontier-agentic \
            vfa-export-agents \
            --platform codex \
            --agents kubernetes-live-rbac-mutation-guard-agent \
            --repo .

      - name: Guard RBAC mutation
        uses: anthropics/claude-code-action@beta
        with:
          agent: .codex/agents/kubernetes-live-rbac-mutation-guard-agent.toml
          prompt: |
            Capture current RBAC state, assess the proposed change, and output
            verdict, evidence_level, blockers, safe_next_actions, open_questions.
            Block if escalate/bind/impersonate verbs present or cluster-admin binding.
        env:
          ANTHROPIC_API_KEY: ${{ secrets.ANTHROPIC_API_KEY }}
          KUBECONFIG: ${{ secrets.KUBECONFIG_PRODUCTION }}

      - name: Apply if approved
        run: |
          # Only runs if guard step exits 0 (verdict=approved)
          kubectl apply -f k8s/rbac/ --dry-run=server
          kubectl apply -f k8s/rbac/
```

### Role-Based Install (all security engineers)

```yaml
# .github/workflows/vfa-install-role.yml
name: Install VFA Role

on:
  workflow_dispatch:
    inputs:
      role:
        description: 'VFA role to install'
        required: true
        type: choice
        options:
          - cloud-security-engineer
          - cloud-platform-engineer
          - cloud-dba
          - cloud-finops-analyst
          - cloud-solutions-architect
          - cloud-devops-engineer
      platform:
        description: 'Target harness platform'
        required: true
        type: choice
        options:
          - claude-code
          - codex
          - copilot
          - cursor
          - gemini
          - kiro

jobs:
  install:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install VFA role agents
        run: |
          npx --yes @raishin/vanguard-frontier-agentic \
            vfa-export-agents \
            --platform ${{ inputs.platform }} \
            --role ${{ inputs.role }} \
            --repo . \
            --force

      - name: Commit installed agents
        run: |
          git config user.name "vfa-bot"
          git config user.email "vfa-bot@noreply"
          git add .
          git commit -m "chore: install vfa role ${{ inputs.role }} for ${{ inputs.platform }}"
          git push
```

---

## Azure DevOps

### Prerequisites

Add to your pipeline `pool` or as a script step:

```yaml
- script: |
    npm install -g @raishin/vanguard-frontier-agentic
  displayName: Install VFA CLI
```

### Azure RBAC Review Gate

```yaml
# azure-pipelines-vfa-entra-guard.yml
trigger: none
pr:
  paths:
    include:
      - iam/**
      - bicep/roleAssignments/**
      - terraform/azure/**

pool:
  vmImage: ubuntu-latest

stages:
  - stage: VFAEntraRoleReview
    displayName: VFA Entra Role Assignment Review
    jobs:
      - job: ReviewGate
        steps:
          - checkout: self

          - script: |
              npx --yes @raishin/vanguard-frontier-agentic \
                vfa-export-agents \
                --platform codex \
                --role cloud-security-engineer \
                --repo $(Build.SourcesDirectory)
            displayName: Install VFA security-engineer agents

          - task: AzureCLI@2
            displayName: Guard Entra role assignment
            inputs:
              azureSubscription: $(AZURE_SERVICE_CONNECTION)
              scriptType: bash
              scriptLocation: inlineScript
              inlineScript: |
                # Agent captures current assignments, classifies risk,
                # and outputs verdict JSON to vfa-evidence.json
                codex \
                  --agent .codex/agents/azure-live-entra-role-assignment-guard-agent.toml \
                  --output vfa-evidence.json

          - task: PublishBuildArtifacts@1
            condition: always()
            inputs:
              pathToPublish: vfa-evidence.json
              artifactName: vfa-entra-evidence
```

### OCI Network Security Gate

```yaml
# azure-pipelines-vfa-oci-network-guard.yml
trigger:
  branches:
    include: [main]
  paths:
    include:
      - oci/network/**
      - terraform/oci/security_lists/**

pool:
  vmImage: ubuntu-latest

stages:
  - stage: VFAOCINetworkGuard
    displayName: VFA OCI Network Security Rule Guard
    jobs:
      - job: LiveGuard
        environment: oci-production      # manual approval gate in ADO Environments
        steps:
          - checkout: self

          - script: |
              npx --yes @raishin/vanguard-frontier-agentic \
                vfa-export-agents \
                --platform codex \
                --agents oci-live-network-security-rule-guard-agent \
                --repo $(Build.SourcesDirectory)
            displayName: Install OCI network guard agent

          - task: OciInstallCli@0
            inputs:
              version: latest

          - script: |
              codex \
                --agent .codex/agents/oci-live-network-security-rule-guard-agent.toml \
                --output vfa-evidence.json
            displayName: Guard OCI security rule mutation
            env:
              OCI_CLI_AUTH: instance_principal

          - task: PublishBuildArtifacts@1
            condition: always()
            inputs:
              pathToPublish: vfa-evidence.json
              artifactName: vfa-oci-network-evidence
```

---

## OCI DevOps

### OCI Build Pipeline — RBAC Review (BEFORE)

```yaml
# oci-build-spec-vfa-rbac-review.yaml
version: 0.1
component: build
timeoutInSeconds: 600

steps:
  - type: Command
    name: Install VFA CLI
    command: |
      npm install -g @raishin/vanguard-frontier-agentic

  - type: Command
    name: Export cloud-security-engineer agents
    command: |
      vfa-export-agents \
        --platform codex \
        --role cloud-security-engineer \
        --repo ${OCI_PRIMARY_SOURCE_DIR}

  - type: Command
    name: Run Kubernetes RBAC review
    command: |
      codex \
        --agent ${OCI_PRIMARY_SOURCE_DIR}/.codex/agents/kubernetes-rbac-review-agent.toml \
        --output ${OCI_PRIMARY_SOURCE_DIR}/vfa-evidence.json
    onFailure:
      steps:
        - type: Command
          command: echo "RBAC review blocked — see vfa-evidence.json"

outputArtifacts:
  - name: vfa_rbac_evidence
    type: BINARY
    location: ${OCI_PRIMARY_SOURCE_DIR}/vfa-evidence.json
```

### OCI Deployment Pipeline — IAM Policy Guard (AT)

```yaml
# oci-deploy-spec-vfa-iam-guard.yaml
version: 0.1
component: deployment
timeoutInSeconds: 900

steps:
  - type: Command
    name: Install VFA CLI
    command: npm install -g @raishin/vanguard-frontier-agentic

  - type: Command
    name: Export OCI IAM guard agent
    command: |
      vfa-export-agents \
        --platform codex \
        --agents oci-live-iam-policy-compartment-guard-agent \
        --repo ${OCI_WORKING_DIRECTORY}

  - type: Command
    name: Guard IAM policy mutation
    command: |
      codex \
        --agent ${OCI_WORKING_DIRECTORY}/.codex/agents/oci-live-iam-policy-compartment-guard-agent.toml \
        --output ${OCI_WORKING_DIRECTORY}/vfa-iam-evidence.json

  - type: Command
    name: Apply if approved
    command: |
      verdict=$(python3 -c "import json; print(json.load(open('vfa-iam-evidence.json'))['verdict'])")
      if [ "$verdict" != "approved" ]; then
        echo "Blocked — verdict: $verdict"
        exit 1
      fi
      oci iam policy update ...
```

---

## Evidence Retention

All three platforms support artifact upload. Store `vfa-evidence.json` for:

| Compliance Framework | Minimum Retention |
|---|---|
| SOC 2 Type II | 12 months |
| PCI DSS v4 | 12 months |
| ISO 27001 | Aligned to ISMS review cycle (typically 12–36 months) |
| NIS 2 | Per member-state transposition (typically 24 months) |

Name artifacts with the change ID: `vfa-evidence-{run_id}-{change_id}.json` for traceability without a separate ticketing system.

---

## Extending to New Platforms

1. Add a new `platform` option to the `vfa-export-agents --platform` flag (see `scripts/export-marketplace-agents.mjs`).
2. Add a pipeline template section above following the same three-stage pattern (BEFORE/AT/AFTER).
3. Reference `docs/evidence-output-spec.md` for the required response fields the new integration must emit.
4. Do not skip the AFTER verification step — it is the audit trail layer, not a nice-to-have.
