# Azure Role-Based Skill Gap Analysis

Last updated: **2026-04-27**

## 1. Inventory baseline

This repository started Azure-thin, but this implementation wave materially changed that.

### Current repo state

| Scope | Count | Evidence |
| --- | ---: | --- |
| Azure skills in `skills/azure/` | 20 | role-based Azure skill folders now exist under `skills/azure/` |
| Azure entries in `catalog/skills.json` | 20 | catalog contains the same 20 Azure skill paths |
| Azure docs in `docs/` | 2 | `docs/azure-role-skill-gap-analysis.md`, `docs/azure-role-skill-specs.md` |

### Official Microsoft Azure skill baselines

| Source | Date verified | Count | Notes |
| --- | --- | ---: | --- |
| Repo snapshot in `skills/azure/README.md` | 2026-04-27 | 25 | Local snapshot of `microsoft/azure-skills` as recorded in this repo |
| `skills.sh/microsoft/azure-skills` | 2026-04-27 | 27 | Includes additional visible skills such as `azure-cost-optimization` and `azure-observability` |
| `skills.sh/microsoft/github-copilot-for-azure` | 2026-04-27 | 46 | Includes extra operational skills such as `azure-networking`, `azure-security`, `azure-role-selector`, `azure-quick-review`, and `azure-keyvault-expiration-audit` |

### Evidence-backed conclusion

The repo now has a **real first-wave Azure portfolio**, not just a single RBAC skill. But it is still not complete.

The remaining gap is narrower and more useful: the first-wave role lanes are now covered, and the remaining work is mostly quality hardening and future specialization.

The residual work areas are:

- stronger eval and compliance coverage,
- overlap reduction between adjacent skills,
- and possible future workload-specific operator lanes beyond the current first wave.

### Baseline upstream references

- Local Azure snapshot: `skills/azure/README.md`
- Azure landing zone design areas: <https://learn.microsoft.com/azure/cloud-adoption-framework/ready/landing-zone/design-areas>
- Azure governance design area: <https://learn.microsoft.com/azure/cloud-adoption-framework/ready/landing-zone/design-area/governance>
- Azure MCP tool inventory: <https://learn.microsoft.com/azure/developer/azure-mcp-server/tools/>
- Microsoft Azure skill directory on skills.sh: <https://skills.sh/microsoft/azure-skills>
- Microsoft GitHub Copilot for Azure directory on skills.sh: <https://skills.sh/microsoft/github-copilot-for-azure>

## 2. Role matrix

The portfolio should be organized by **role lane**, not by Azure service SKU.

| Role lane | Microsoft design areas / guidance | Upstream Microsoft skill anchors | Current repo coverage | Recommended local role skills | Verdict |
| --- | --- | --- | --- | --- | --- |
| Platform architect / landing-zone owner | Landing zone design areas: identity, resource organization, network, security, management, governance, automation | `azure-enterprise-infra-planner`, `azure-prepare`, `azure-validate`, `azure-deploy` | `azure-landing-zone-architect`, `azure-subscription-resource-organization`, `azure-governance-policy-guardrails` | retain and refine these role skills | **Covered (first wave)** |
| Identity / access / zero-trust | RBAC, Entra identity, access governance, PIM, entitlement management | `azure-rbac`, `azure-role-selector` | `azure-rbac-review`, `azure-role-selector`, `azure-identity-governance-review` | keep all three; narrow overlaps later if needed | **Covered (first wave)** |
| Network / connectivity | Hub-spoke, Private Link, DNS, network isolation, landing-zone connectivity | `azure-networking`, `azure-kubernetes` | `azure-network-topology-review`, `azure-private-endpoint-adoption-planner` | keep; later connect more tightly to AKS/App Service operator skills | **Covered (first wave)** |
| Security / posture | Security design area, Key Vault, Defender, policy guardrails, private access | `azure-security`, `azure-compliance`, `azure-aigateway` | `azure-security-posture-hardening`, `azure-key-vault-secret-lifecycle-auditor` | keep both; later clarify boundary between broad posture and vault-specific lifecycle audits | **Covered (first wave)** |
| SRE / operations / incident | Azure Monitor, Application Insights, Resource Health, alerting, operational excellence | `azure-observability`, `appinsights-instrumentation`, `azure-diagnostics` | `azure-observability-investigator`, `azure-resource-health-incident-triage`, `azure-resilience-bcdr-review` | keep and validate against realistic prompts | **Covered (first wave)** |
| FinOps / governance | Cost Management, budgets, pricing, Advisor, governance controls | `azure-cost`, `azure-cost-optimization`, `azure-quotas` | `azure-cost-optimization-governor`, `azure-cost-estimation-review` | keep; later add stronger evals around evidence thresholds | **Covered (first wave)** |
| Platform automation / delivery | IaC accelerator, Bicep/Terraform, CI/CD, deployment separation, safe rollout | `azure-prepare`, `azure-validate`, `azure-deploy`, `airunway-aks-setup` | `azure-platform-automation-devops`, `azure-app-service-production-readiness`, `azure-aks-platform-operator` | keep; this is now one of the stronger lanes | **Covered (first wave)** |
| Migration / modernization | Azure Migrate, landing-zone cutover, brownfield controls | `azure-cloud-migrate`, `azure-upgrade` | `azure-migrate-landing-zone-cutover` | keep and add eval scenarios for stale assessments and rollback failure modes | **Covered (first wave)** |
| AI platform / Foundry | Foundry RBAC, quotas, private networking, evaluations, change safety | `microsoft-foundry`, `azure-ai`, `azure-hosted-copilot-sdk` | `azure-ai-foundry-ops-governor` | keep; later add scenario-specific evals | **Covered (first wave)** |

## 3. Gap verdict

### A. Missing role coverage

The original structural gap is largely closed. The repo now has first-class Azure skills for most of the platform-owner roles that mattered most:

- landing zones,
- governance,
- network topology,
- management and monitoring,
- platform automation,
- cost control,
- DR and reliability operations,
- and Azure AI platform governance.

The remaining problem is **not absence** anymore. It is **portfolio completion and quality control**.

### B. Partial overlap already exists

The earlier identity gap is now addressed by:

- `azure-rbac-review`,
- `azure-role-selector`,
- `azure-identity-governance-review`.

That lane is now viable, but overlap should be watched so the three skills do not collapse into each other.

### C. Duplicate-risk areas

Avoid building shallow clones of Microsoft’s SDK-centric skill inventory. That would create:

- maintenance drag,
- catalog noise,
- poor trigger quality,
- and weak role differentiation.

Higher-risk duplicate zones:

- language-specific SDK skills,
- generic service overviews with no operator workflow,
- simple MCP tool wrappers with no decision logic or safety contract.

### D. Low-priority or optional gaps

These are still real, but should not outrank the remaining missing specialist skills:

- language-specific Azure SDK skills,
- single-product tutorials,
- one-command convenience skills,
- niche workload accelerators before the platform lanes exist.

## 4. Prioritized backlog

### P0 — remaining highest-priority quality work

| Skill | Why now | Source anchors |
| --- | --- | --- |
| `skill-comply` dry-runs across 5+ Azure skills | The portfolio now exists; the bigger risk is that some skills sound strong but do not reliably induce the intended workflow | internal compliance workflow |
| `eval-harness` prompt sets across each role lane | Trigger quality and regression safety are still unproven until evaluated systematically | internal eval workflow |
| overlap review across identity, security, networking, and platform-operation lanes | Growth increases duplicate-trigger risk and weak role boundaries if left unchecked | this artifact + `docs/azure-role-skill-specs.md` |

### P1 — next hardening wave

| Skill | Why next | Source anchors |
| --- | --- | --- |
| richer examples and sanitized evidence templates inside selected Azure skills | Some skills are structurally strong but still rely on the operator to invent evidence-gathering inputs | internal skill quality work |
| normalization of earlier Azure skills to the newer OCI-style structure | Newer skills are stronger and more explicit than some earlier first-wave drafts | local repo consistency work |
| scenario packs for high-risk lanes such as migration, AKS, App Service, and Key Vault | These lanes need more adversarial prompt testing than low-risk catalog helpers | internal eval workflow |

### P2 — useful, but lower urgency now

| Skill | Reason to defer |
| --- | --- |
| More workload-specific Azure operator skills | The portfolio already expanded fast; quality, evals, and overlap control matter more than raw count |
| SDK- or product-tutorial clones | They would bloat the catalog and dilute the role-first design |

## Decision rules for the next implementation phase

1. Prefer **operator-grade role skills** over service or SDK clones.
2. Require every Azure skill to define:
   - evidence sources,
   - preferred MCP/doc path,
   - safety gates,
   - explicit output contract,
   - minimum eval definition,
   - and minimum compliance sequence.
3. Do not create new Azure skill folders until the spec is decision-complete.
4. Before the next growth wave, run `skill-comply` and `eval-harness` against representative first-wave Azure skills.
5. When actual skills are added later, update:
   - `skills/azure/*`,
   - `catalog/skills.json`,
   - `catalog/skill-manifest.json`,
   - and run `npm run validate`.
