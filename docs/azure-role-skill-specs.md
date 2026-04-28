# Azure Role-Based Skill Specs

Last updated: **2026-04-27**

This file is the decision-complete spec backlog for the first Azure role-based skill wave.

## Working method

This backlog is intentionally shaped by the requested skill workflow:

1. **`eval-harness`**: every spec defines a minimum pass/fail gate before any skill draft exists.
2. **`skill-creator`**: each spec is detailed enough to draft a real `SKILL.md` without extra product decisions.
3. **`skill-comply`**: each spec includes a minimum expected behavioral sequence that can later be measured.

## Global portfolio rules

- Prefer official Microsoft Learn, Azure Architecture Center, and Azure MCP documentation over blogs.
- Prefer Azure MCP evidence when it reduces guesswork; fall back to docs when live access is absent.
- Distinguish control plane, data plane, and identity plane risks.
- Call out blast radius, least privilege, rollout risk, and rollback expectations for any production-affecting guidance.
- Do not create SDK tutorial clones unless the role skill cannot cover the need.

---

## P0 specs

### `azure-landing-zone-architect`

- **Classification:** role-synthesis
- **Trigger description:** Use when the user asks for Azure platform design, landing-zone review, management-group/subscription structure, shared services layout, or an architecture decision that spans identity, networking, governance, and operations.
- **Target role and scenarios:** platform architect, cloud platform owner, enterprise landing-zone lead; greenfield Azure platform setup, brownfield rationalization, multi-subscription operating model reviews.
- **In scope:** landing-zone design areas, platform vs application landing zones, management-group hierarchy, subscription placement, hub-spoke vs alternatives, baseline governance/security/management decisions, operating-model gaps.
- **Out of scope:** per-service deep implementation, detailed app workload design, writing production Bicep/Terraform.
- **Required evidence sources:** Azure landing-zone design areas, Azure landing-zone architecture, governance/security/identity design areas, Azure MCP tool inventory.
- **Preferred MCP/doc/tool path:** `cloudarchitect`, `wellarchitectedframework`, `policy`, `role`, `group`, `subscription`, plus Microsoft Learn architecture docs.
- **Output contract:** architecture verdict; target platform model; decision table by design area; unresolved risks; recommended next actions; explicit assumptions.
- **Safety constraints:** no blind recommendation of a single management-group hierarchy; no broad admin grants; no claim that landing zones are “done” without management, governance, and recovery posture.
- **Overlap notes:** umbrella skill; should route narrower RBAC, networking, observability, and governance questions to adjacent specialized skills when needed.
- **Minimum eval definition:** capability evals for (1) greenfield landing-zone review, (2) brownfield hierarchy critique, (3) platform/application boundary definition; fail if any design area is omitted.
- **Minimum compliance sequence expected by `skill-comply`:**
  1. Classify greenfield vs brownfield and who owns the platform.
  2. Map the ask to landing-zone design areas.
  3. Identify critical missing facts and explicit assumptions.
  4. Produce design-area-by-design-area recommendations.
  5. Surface blast radius, governance, and operations gaps before final advice.

### `azure-governance-policy-guardrails`

- **Classification:** role-synthesis
- **Trigger description:** Use for Azure Policy, guardrails, management-group inheritance, compliance enforcement, tag governance, allowed regions/SKUs, or governance automation reviews.
- **Target role and scenarios:** cloud governance lead, platform engineer, compliance engineer; baseline policy design, brownfield governance hardening, policy assignment review.
- **In scope:** policy definitions vs initiatives, assignment scope, inheritance, exclusions, remediation, tags, region/SKU restrictions, management baseline links to governance.
- **Out of scope:** full regulatory interpretation, SIEM operations, writing organization-specific policy JSON unless later requested.
- **Required evidence sources:** governance design area, Azure Policy overview, ALZ default policy assignments, Azure MCP policy/advisor/pricing docs.
- **Preferred MCP/doc/tool path:** `policy`, `advisor`, `pricing`, `group`, `subscription`; Microsoft Learn governance docs.
- **Output contract:** current-state governance summary; risky gaps; recommended guardrails; scope placement; exclusion strategy; staged rollout advice.
- **Safety constraints:** no root-scope policy sprawl without justification; no deny policies proposed without change-safety notes; always flag remediation/destructive implications.
- **Overlap notes:** complements `azure-landing-zone-architect`; should not absorb full identity or security-posture reviews.
- **Minimum eval definition:** capability evals for (1) tag/region guardrail design, (2) initiative placement review, (3) brownfield policy hardening with exclusions.
- **Minimum compliance sequence expected by `skill-comply`:**
  1. Identify governing scope and hierarchy.
  2. Separate audit-only, deny, deployIfNotExists, and remediation concerns.
  3. Recommend assignment level and exclusions.
  4. State rollout risk and rollback/exception handling.
  5. Return enforceable guardrails, not generic governance prose.

### `azure-role-selector`

- **Classification:** upstream-parity
- **Trigger description:** Use when the user asks which Azure role to assign, how to grant minimum access, whether built-in roles are enough, or when a custom role may be required.
- **Target role and scenarios:** IAM engineer, platform operator, app team onboarding; service-principal access, managed-identity permissions, narrow operator grants.
- **In scope:** built-in role matching, scope selection, custom-role fallback criteria, control-plane vs data-plane distinction, assignment examples.
- **Out of scope:** tenant-wide governance design, identity lifecycle governance, privileged workflow design.
- **Required evidence sources:** Azure RBAC overview/best practices, built-in role docs, Azure MCP RBAC tools, upstream `azure-role-selector` pattern.
- **Preferred MCP/doc/tool path:** `role`, `extension`, `bicepschema`, `get azure bestpractices get`; Microsoft Learn RBAC docs.
- **Output contract:** requested permission summary; recommended built-in role or custom-role rationale; scope recommendation; validation command/path; risks and assumptions.
- **Safety constraints:** prefer built-in roles first; do not jump to Owner/Contributor by habit; call out data-plane access separately.
- **Overlap notes:** sits next to `azure-rbac-review`; the latter critiques assignments, this one selects the minimum role shape.
- **Minimum eval definition:** capability evals for (1) storage read-only access, (2) App Service deploy-only access, (3) custom role fallback when no built-in role matches.
- **Minimum compliance sequence expected by `skill-comply`:**
  1. Parse the requested actions and target resource type.
  2. Distinguish control-plane and data-plane needs.
  3. Search for the narrowest built-in role.
  4. Only then consider custom role fallback.
  5. Return assignment scope and validation path.

### `azure-network-topology-review`

- **Classification:** role-synthesis
- **Trigger description:** Use when the user asks for Azure network architecture review, hub-spoke critique, routing/DNS segmentation, firewall placement, or landing-zone connectivity advice.
- **Target role and scenarios:** network architect, platform owner, AKS/App Service platform teams; multi-subscription connectivity, hybrid access, shared-services network reviews.
- **In scope:** hub-spoke topology, peering, connectivity boundaries, DNS dependencies, NSG/firewall placement, landing-zone connectivity considerations, platform/team ownership lines.
- **Out of scope:** packet-level troubleshooting, detailed firewall rule authoring, ExpressRoute implementation runbooks.
- **Required evidence sources:** hub-spoke reference architecture, landing-zone network design area, Private Link architecture guidance when relevant.
- **Preferred MCP/doc/tool path:** Azure docs first; if Azure MCP networking tools exist in the target client, use them for evidence. Otherwise use CLI/doc fallback.
- **Output contract:** topology summary; key risks; ownership model; recommended topology adjustments; validation checks; open questions.
- **Safety constraints:** no “flat network is fine” shortcuts; must call out DNS and route dependencies; must separate platform-shared and workload-local controls.
- **Overlap notes:** hand off Private Link placement to `azure-private-endpoint-adoption-planner` when private endpoint design is the main issue.
- **Minimum eval definition:** capability evals for (1) hub-spoke review, (2) multi-subscription shared-services critique, (3) hybrid routing concern review.
- **Minimum compliance sequence expected by `skill-comply`:**
  1. Classify topology and connectivity model.
  2. Identify shared services, trust boundaries, and route ownership.
  3. Check DNS, peering, security, and management implications.
  4. Surface bottlenecks and blast-radius issues.
  5. Return concrete architecture corrections.

### `azure-security-posture-hardening`

- **Classification:** role-synthesis
- **Trigger description:** Use for Azure security posture review, baseline hardening, managed identity adoption, Key Vault usage, Defender/Policy-driven hardening, or zero-trust cloud control reviews.
- **Target role and scenarios:** cloud security engineer, platform security lead, application security architect; landing-zone hardening, service exposure review, secret-handling posture review.
- **In scope:** least privilege, managed identities, private endpoints where justified, Key Vault adoption, logging/auditing expectations, policy-enforced controls, security baseline critique.
- **Out of scope:** full compliance audit, incident forensics, detailed SOC workflows.
- **Required evidence sources:** landing-zone security design area, Azure security best practices, Key Vault docs, Foundry security docs when AI scope exists.
- **Preferred MCP/doc/tool path:** `keyvault`, `role`, `policy`, `monitor`, `advisor`, `extension`; Microsoft Learn security docs.
- **Output contract:** security posture summary; high-risk findings; prioritized hardening recommendations; safe sequencing; evidence gaps.
- **Safety constraints:** do not expose secrets; do not recommend public endpoints by default for sensitive services; explicitly separate urgent from strategic controls.
- **Overlap notes:** broader than `azure-rbac-review`; narrower than a full compliance skill.
- **Minimum eval definition:** capability evals for (1) managed identity migration advice, (2) Key Vault/private endpoint hardening review, (3) broad access posture critique.
- **Minimum compliance sequence expected by `skill-comply`:**
  1. Identify identities, network exposure, and secret flows.
  2. Check baseline controls and missing telemetry.
  3. Prioritize high-impact hardening actions.
  4. Flag blast radius and rollout risks.
  5. Return staged recommendations with evidence basis.

### `azure-observability-investigator`

- **Classification:** role-synthesis
- **Trigger description:** Use for Azure Monitor, Log Analytics, Application Insights, alerting, workbooks, KQL triage, or operational-excellence investigations.
- **Target role and scenarios:** SRE, platform operator, incident responder, application owner; missing telemetry, noisy alerts, error triage, operational health reviews.
- **In scope:** metrics/logs/traces strategy, workspace and alert review, KQL-based investigation, action groups and alert-processing rules, workbook/reporting patterns.
- **Out of scope:** code instrumentation implementation unless explicitly requested, full app debugging, SIEM engineering.
- **Required evidence sources:** Azure Monitor best practices, alerts overview, Log Analytics operational guidance, upstream `azure-observability`.
- **Preferred MCP/doc/tool path:** `monitor`, `applicationinsights`, `kusto`, `workbooks`, `grafana` where available; docs fallback otherwise.
- **Output contract:** incident or posture summary; signals reviewed; likely failure domain; recommended alert/telemetry improvements; next diagnostic steps.
- **Safety constraints:** do not pretend logs prove what they do not; call out ingestion latency and missing telemetry; distinguish symptom from root cause.
- **Overlap notes:** incident-specific Resource Health workflows should defer to `azure-resource-health-incident-triage`.
- **Minimum eval definition:** capability evals for (1) noisy alert review, (2) recent failure investigation, (3) telemetry baseline gap assessment.
- **Minimum compliance sequence expected by `skill-comply`:**
  1. Classify issue as metrics, logs, traces, alerting, or configuration.
  2. Gather the highest-value signals first.
  3. Distinguish evidence, correlation, and inference.
  4. Recommend concrete query/alert/workbook improvements.
  5. End with operational next steps and residual blind spots.

### `azure-ai-foundry-ops-governor`

- **Classification:** role-synthesis
- **Trigger description:** Use when the user asks for Azure AI Foundry operations, RBAC, quotas, private networking, deployment governance, evaluations, or safe MCP-based Foundry operations.
- **Target role and scenarios:** AI platform engineer, AI security lead, Foundry platform owner; Foundry rollout, multi-team governance, quota planning, safe deployment operations.
- **In scope:** Foundry resource vs project boundaries, RBAC scope, quota checks, private link/network isolation considerations, diagnostics/logging, MCP execution safety.
- **Out of scope:** model prompt engineering, app-level agent logic, generic AI strategy.
- **Required evidence sources:** Foundry architecture, Foundry rollout planning, Foundry MCP security best practices, Foundry quotas/limits.
- **Preferred MCP/doc/tool path:** `foundry`, `quota`, `monitor`, `role`, `keyvault`; Microsoft Foundry docs and Foundry MCP best-practice docs.
- **Output contract:** governance summary; resource/project boundary decisions; RBAC and quota posture; network/security constraints; safe next actions.
- **Safety constraints:** must call out preview risk for Foundry MCP where relevant; verify write vs read operations; require nonproduction-first for mutating actions.
- **Overlap notes:** AI-specific specialization; do not absorb generic cost or network design unless tightly coupled to Foundry.
- **Minimum eval definition:** capability evals for (1) Foundry RBAC review, (2) quota-aware deployment planning, (3) private-networking governance critique.
- **Minimum compliance sequence expected by `skill-comply`:**
  1. Separate Foundry management scope from project scope.
  2. Check RBAC, quota, network isolation, and logging posture.
  3. Distinguish read-only discovery from write-risk operations.
  4. Recommend least-privilege and safe execution steps.
  5. Return rollout blockers and mitigation plan.

---

## P1 specs

### `azure-identity-governance-review`

- **Classification:** role-synthesis
- **Trigger description:** Use for Entra PIM, access reviews, entitlement management, recurring privilege review, or identity-governance posture checks for Azure operators.
- **Target role and scenarios:** IAM governance lead, security architect, platform owner.
- **In scope:** standing vs eligible access, access-review posture, entitlement-management use, privileged workflow risks, ownership/accountability gaps.
- **Out of scope:** low-level app authentication debugging, broad directory architecture redesign.
- **Required evidence sources:** identity-access design area, Entra governance docs, security design area references to PIM and access reviews.
- **Preferred MCP/doc/tool path:** docs-first; Azure role tools where role assignments need correlation.
- **Output contract:** current privilege model; governance gaps; recommended review cadence and control pattern; assumptions.
- **Safety constraints:** do not assume PIM solves poor scope design; always challenge standing privilege.
- **Overlap notes:** extends `azure-rbac-review`; does not replace `azure-role-selector`.
- **Minimum eval definition:** capability evals for PIM adoption, access review posture, and entitlement workflow critique.
- **Minimum compliance sequence expected by `skill-comply`:** identify privileged actors; separate assignment design from governance process; recommend review/eligibility model; surface operational ownership.

### `azure-private-endpoint-adoption-planner`

- **Classification:** role-synthesis
- **Trigger description:** Use for private endpoint placement, hub-vs-spoke design, DNS-zone linkage, and PaaS isolation trade-offs.
- **Target role and scenarios:** network architect, security engineer, platform owner.
- **In scope:** private endpoint placement, DNS integration, route implications, centralized vs workload-local endpoints, hub-spoke/Virtual WAN considerations.
- **Out of scope:** generic network topology reviews that do not hinge on Private Link.
- **Required evidence sources:** Private Link hub-spoke guidance, private endpoint DNS integration guidance, Azure Monitor private link design when observability is involved.
- **Preferred MCP/doc/tool path:** docs-first; networking MCP tools if available.
- **Output contract:** placement recommendation; DNS requirements; routing/security implications; rollout caveats.
- **Safety constraints:** never ignore DNS; explicitly call out `/32` route and access-control implications when relevant.
- **Overlap notes:** companion to `azure-network-topology-review`.
- **Minimum eval definition:** capability evals for hub placement, spoke placement, and DNS-linked multi-subscription design.
- **Minimum compliance sequence expected by `skill-comply`:** identify consumers and shared resources; choose hub vs spoke with rationale; map DNS and route impacts; return safe rollout steps.

### `azure-resource-health-incident-triage`

- **Classification:** repo-original
- **Trigger description:** Use when the user asks whether an Azure outage or degraded resource is the cause, or needs first-pass cloud-health triage.
- **Target role and scenarios:** incident commander, SRE, support engineer.
- **In scope:** resource health, service health, activity-log alerts, incident classification, immediate evidence collection, escalation framing.
- **Out of scope:** full RCA, long-term observability redesign, app-code fixes.
- **Required evidence sources:** Azure Resource Health docs, Azure Monitor alerts docs, Azure MCP tool inventory.
- **Preferred MCP/doc/tool path:** `resourcehealth`, `monitor`, `group`, `subscription`; docs fallback if tool access is absent.
- **Output contract:** current health finding; likely scope of impact; evidence collected; next triage actions; what remains unknown.
- **Safety constraints:** do not over-attribute platform health signals as root cause; distinguish Azure issue from workload issue.
- **Overlap notes:** narrower and faster than `azure-observability-investigator`.
- **Minimum eval definition:** capability evals for platform outage triage, degraded resource triage, and unknown-cause incident routing.
- **Minimum compliance sequence expected by `skill-comply`:** check health signals first; classify blast radius; separate provider incident from tenant misconfiguration; return immediate next actions.

### `azure-resilience-bcdr-review`

- **Classification:** role-synthesis
- **Trigger description:** Use for Azure resilience, HA/DR review, RTO/RPO mapping, runbook quality, recovery drill planning, or business-continuity critique.
- **Target role and scenarios:** solution architect, SRE lead, platform owner.
- **In scope:** RTO/RPO framing, failover/failback assumptions, shared-responsibility constraints, DR documentation/runbook quality, service-level recovery gaps.
- **Out of scope:** service-specific backup configuration steps unless later requested.
- **Required evidence sources:** Well-Architected reliability/disaster recovery docs, landing-zone BCDR design area references, service reliability guidance when relevant.
- **Preferred MCP/doc/tool path:** docs-first; `resourcehealth`, `monitor`, `advisor` can supplement live posture evidence.
- **Output contract:** reliability posture summary; target recovery model; missing controls; drill/test recommendations; unresolved dependencies.
- **Safety constraints:** no zero-RTO/RPO fantasies without cost/complexity challenge; must call out services that do not auto-failover cross-region.
- **Overlap notes:** broader than incident triage; can be referenced by landing-zone and workload-specific skills.
- **Minimum eval definition:** capability evals for RTO/RPO critique, multi-region DR plan review, and runbook/testability review.
- **Minimum compliance sequence expected by `skill-comply`:** identify business targets; map service realities; expose unsupported assumptions; recommend tested recovery posture.

### `azure-cost-optimization-governor`

- **Classification:** role-synthesis
- **Trigger description:** Use for Azure spend governance, budgets, alerts, exports, cost controls, pricing posture, and FinOps review.
- **Target role and scenarios:** FinOps lead, platform owner, engineering manager.
- **In scope:** budgets, alerts, cost analysis posture, tagging for cost, reservation/savings-plan awareness, governance-grade cost controls.
- **Out of scope:** procurement negotiation, invoice reconciliation details, one-off SKU pricing lookup unless later requested.
- **Required evidence sources:** cost planning docs, budgets tutorial, cost-management automation overview, governance design area.
- **Preferred MCP/doc/tool path:** `pricing`, `advisor`, `quota`, Microsoft Cost Management docs.
- **Output contract:** spend-control posture; immediate waste/risk findings; budget/alert/export recommendations; ownership model.
- **Safety constraints:** do not promise savings without utilization evidence; distinguish cost estimation from optimization governance.
- **Overlap notes:** broader than a simple pricing calculator helper.
- **Minimum eval definition:** capability evals for budget strategy, alerting design, and reservation/savings-plan decision framing.
- **Minimum compliance sequence expected by `skill-comply`:** inspect visibility and ownership first; recommend budgets/alerts/exports; call out cost-risk assumptions; return prioritized controls.

### `azure-platform-automation-devops`

- **Classification:** role-synthesis
- **Trigger description:** Use for Azure platform automation, landing-zone IaC delivery, Bicep/Terraform rollout workflow, CI/CD safety gates, and infrastructure/application deployment separation.
- **Target role and scenarios:** platform engineer, DevOps lead, cloud architect.
- **In scope:** IaC accelerator choices, Bicep/Terraform positioning, bootstrap/run phases, pipeline separation, secret handling, safe rollout patterns, validation gates.
- **Out of scope:** writing full pipelines on first pass, application release strategy unrelated to Azure platform automation.
- **Required evidence sources:** landing-zone implementation options, IaC accelerator guidance, App Service platform automation guidance, Azure MCP deploy/bicepschema docs.
- **Preferred MCP/doc/tool path:** `deploy`, `bicepschema`, `extension`, `advisor`; docs for accelerator patterns.
- **Output contract:** automation strategy; control points; pipeline split; validation gates; rollout sequencing.
- **Safety constraints:** no direct production-deploy advice without validation gates; no secret-in-repo patterns; separate infra and app flows.
- **Overlap notes:** execution-oriented companion to `azure-landing-zone-architect`.
- **Minimum eval definition:** capability evals for IaC approach selection, CI/CD control design, and secure deployment-flow review.
- **Minimum compliance sequence expected by `skill-comply`:** classify platform vs workload automation; choose IaC/control model; define gates and secret handling; return safe deployment workflow.

### `azure-aks-platform-operator`

- **Classification:** role-synthesis
- **Trigger description:** Use for AKS production-readiness review, cluster operating-model critique, upgrade safety, node-pool strategy, workload identity posture, network policy review, and cluster-operator readiness.
- **Target role and scenarios:** AKS platform operator, SRE lead, platform architect, cloud platform engineer.
- **In scope:** node-pool separation, cluster/network model, ingress/egress assumptions, workload identity, secret flow, autoscaling realism, subnet/IP headroom, PDB and drain risk, upgrade/rollback posture, operator observability.
- **Out of scope:** workload manifest authoring, application debugging, generic Kubernetes tutorials, full GitOps implementation.
- **Required evidence sources:** AKS baseline architecture, AKS upgrade guidance, workload identity docs, network policy guidance, Azure MCP AKS docs.
- **Preferred MCP/doc/tool path:** `aks` first for cluster evidence; `monitor`, `resourcehealth`, `applicationinsights`, `role`, and `policy` only when they materially support the review; Microsoft Learn fallback otherwise.
- **Output contract:** cluster verdict; ownership model; findings table; upgrade and rollback posture; security/identity posture; safe next actions; open questions.
- **Safety constraints:** do not bless AKS as “managed so safe”; must challenge upgrade assumptions, static secret use, flat traffic models, and missing rollback proof.
- **Overlap notes:** complements `azure-network-topology-review`, `azure-security-posture-hardening`, and `azure-observability-investigator`; should route app-runtime specifics to narrower skills.
- **Minimum eval definition:** capability evals for (1) production upgrade-readiness review, (2) workload identity and secret-flow critique, (3) cluster-operating-model assessment.
- **Minimum compliance sequence expected by `skill-comply`:**
  1. Classify environment, exposure model, and workload criticality.
  2. Map platform versus workload ownership.
  3. Check node-pool, network, identity, scaling, and observability posture.
  4. Stress-test upgrade, drain, surge, and rollback assumptions.
  5. Return a go/no-go style verdict with explicit evidence labels.

### `azure-app-service-production-readiness`

- **Classification:** role-synthesis
- **Trigger description:** Use for Azure App Service production-readiness review, deployment-slot safety, plan-tier fit, networking and private ingress posture, identity/secret handling, scaling, diagnostics, and rollout safety.
- **Target role and scenarios:** App Service platform operator, application platform engineer, SRE, cloud architect.
- **In scope:** plan and SKU fit, App Service Environment versus multitenant fit where relevant, deployment slots, VNet integration, private endpoint or access restrictions, managed identity, app settings and secret posture, autoscale, backup/restore expectations, diagnostics and safe release sequencing.
- **Out of scope:** application code profiling, framework-specific tuning, detailed CI/CD authoring unless separately requested.
- **Required evidence sources:** App Service best-practice docs, deployment-slot guidance, networking guidance, scaling guidance, diagnostic guidance, Azure MCP App Service docs.
- **Preferred MCP/doc/tool path:** `appservice` first where available; `monitor`, `applicationinsights`, `role`, `keyvault`, and `advisor` when they materially support the review; documentation fallback otherwise.
- **Output contract:** readiness verdict; platform fit summary; critical risks; rollout and rollback posture; diagnostics and configuration findings; safe next steps.
- **Safety constraints:** do not treat “it deploys” as production readiness; explicitly challenge slot misuse, public exposure assumptions, weak secret handling, and missing rollback/restore proof.
- **Overlap notes:** complements `azure-platform-automation-devops` and `azure-security-posture-hardening`; should not absorb full landing-zone or AKS concerns.
- **Minimum eval definition:** capability evals for (1) public web app production-readiness review, (2) slot-based rollout critique, (3) network-restricted or private-ingress posture review.
- **Minimum compliance sequence expected by `skill-comply`:**
  1. Classify hosting model, environment criticality, and traffic pattern.
  2. Check plan fit, deployment flow, identity/secret posture, and networking controls.
  3. Verify scaling, diagnostics, and recovery expectations.
  4. Distinguish evidence from inference and challenge weak production claims.
  5. Return a bounded hardening and rollout-safety plan.

### `azure-key-vault-secret-lifecycle-auditor`

- **Classification:** role-synthesis
- **Trigger description:** Use for Key Vault secret lifecycle audits, expiry posture, rotation realism, purge/recovery safety, lifecycle metadata hygiene, and RBAC review around secret operations.
- **Target role and scenarios:** cloud security engineer, platform security owner, secrets-management operator, compliance reviewer.
- **In scope:** soft delete, purge protection, expiration, tags/ownership metadata, RBAC versus legacy access policies, purge authority, Event Grid or alert posture, rotation and recovery readiness.
- **Out of scope:** secret value retrieval unless absolutely necessary, application-side secret-consumer code changes, key-management deep dives unrelated to lifecycle controls.
- **Required evidence sources:** Key Vault secret best practices, autorotation guidance, RBAC guide, soft-delete and recovery docs, Key Vault policy reference, Azure MCP Key Vault docs.
- **Preferred MCP/doc/tool path:** `keyvault` first for safe metadata-oriented evidence; `role`, `monitor`, and `policy` when needed; documentation fallback otherwise.
- **Output contract:** lifecycle verdict; findings table; lifecycle control review; safe next actions; open questions.
- **Safety constraints:** avoid retrieving secret values; treat missing purge protection, broad purge authority, unproven rotation, and false recovery confidence as blockers.
- **Overlap notes:** complements `azure-security-posture-hardening`; should stay focused on lifecycle operations rather than broad platform security.
- **Minimum eval definition:** capability evals for (1) expiry/rotation posture audit, (2) RBAC and purge-authority review, (3) recovery-readiness critique.
- **Minimum compliance sequence expected by `skill-comply`:**
  1. Scope vaults and dependent workloads.
  2. Check soft delete, purge protection, and permission model.
  3. Check expiration, metadata hygiene, rotation, and alerts.
  4. Stress-test recovery and purge assumptions.
  5. Return a go/no-go lifecycle verdict without exposing secrets.

### `azure-migrate-landing-zone-cutover`

- **Classification:** role-synthesis
- **Trigger description:** Use for Azure migration wave review, landing-zone cutover readiness, Azure Migrate assessment critique, dependency sequencing, and rollback/validation planning.
- **Target role and scenarios:** migration lead, cloud architect, platform owner, cutover manager, SRE lead.
- **In scope:** assessment quality, landing-zone readiness, connectivity and DNS dependency checks, wave grouping, migration permissions, validation gates, rollback posture, and post-cutover ownership.
- **Out of scope:** detailed per-tool migration runbooks, low-level replication tuning, or service-specific migration implementation unless separately requested.
- **Required evidence sources:** Azure Migrate assessment overview, assessment prerequisites, application assessment review, platform landing zone generation guidance, CAF landing-zone migration readiness guidance, Azure Migrate release notes.
- **Preferred MCP/doc/tool path:** target-environment evidence from available Azure namespaces such as `group`, `subscription`, `resourcehealth`, `monitor`, and deployment-related tooling when relevant; documentation fallback otherwise.
- **Output contract:** cutover verdict; findings table; readiness review matrix; safe next actions; open questions.
- **Safety constraints:** do not equate “Azure ready” with “cutover ready”; treat stale discovery data, weak dependency mapping, broad permissions, and missing rollback checkpoints as blockers.
- **Overlap notes:** complements `azure-landing-zone-architect` and workload-specific operator skills; remains focused on migration execution readiness.
- **Minimum eval definition:** capability evals for (1) migration wave critique, (2) landing-zone cutover review, (3) rollback and validation-plan challenge.
- **Minimum compliance sequence expected by `skill-comply`:**
  1. Classify migration scope and wave criticality.
  2. Check assessment freshness and discovery quality.
  3. Check landing-zone, dependency, and permission readiness.
  4. Stress-test cutover and rollback mechanics.
  5. Return a go/no-go cutover verdict with explicit missing evidence.

---

## First implementation-wave status

The first implementation wave is now drafted in the repo.

What remains is not basic drafting. It is:

1. `skill-comply` dry-runs against representative Azure skills,
2. `eval-harness` prompt packs across the role lanes,
3. overlap cleanup where nearby skills risk trigger collisions,
4. normalization of older first-wave Azure skills that are structurally weaker than the newer OCI-style drafts.
