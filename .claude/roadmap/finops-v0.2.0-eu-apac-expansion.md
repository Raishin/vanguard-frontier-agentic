# FinOps Cloud Price Advisor v0.2.0 — Multi-Cloud Pricing Expansion

**Status:** Planning
**Target Release:** Q3 2026
**Owner:** FinOps Agent Team
**Scope:** Extend pricing coverage from 3 providers (AWS, Azure, OCI) to 11 providers (bind to all agents with pricing in agents/ folder)

---

## Feature Goals

- **Bind pricing to existing agent ecosystem** — add support for all providers that have agents in agents/ folder
- **Reduce coverage gaps** — match price advisor support (11 providers) to existing agent portfolio (40+ AWS, 35+ Azure, 40+ GCP, 25+ OCI, 40+ Alibaba, 40+ Huawei, 6+ Scaleway/Contabo/Hetzner/IONOS, 6+ OVHCloud)
- **Single-agent routing** — maestro routes cloud-specific questions to finops-cloud-price-advisor-agent; agent handles provider detection
- **Minimal harness changes** — same 6 harnesses (codex, copilot, claude-code, cursor, gemini, kiro); provider integration in skill layer
- **Live-price with fallback** — fetch live prices at runtime; fallback to documented pricing when API unavailable
- **GCP reconsideration** — re-add GCP support (was removed v0.1.1 per Codex feedback, but GCP agents exist; evaluate for v0.2.0)

---

## Implementation Phases

### Phase 1: API Research & Integration Spec (Week 1)

| Provider | API Endpoint | Auth | Rate Limit | Response Format | Agents in Repo | Status |
|----------|---|---|---|---|---|---|
| **AWS** | https://pricing.aws.amazon.com/pricing | Public | Unlimited | JSON | 40+ | ✅ v0.1.1 |
| **Azure** | https://api.microsoft.com/pricing/azure | Public | Unlimited | JSON | 35+ | ✅ v0.1.1 |
| **GCP** | https://www.googleapis.com/compute/v1/projects/*/machineTypes | Public | Unlimited | JSON | 40+ | ❌ Removed v0.1.1 |
| **OCI** | https://pricing.oracle.com/v2 | Public | Unlimited | JSON | 25+ | ✅ v0.1.1 |
| **Alibaba Cloud** | https://www.alibabacloud.com/cloud-computing/pricing | Scrape/API | Variable | HTML/JSON | 40+ | 🟠 v0.2.0 TBD |
| **Huawei Cloud** | https://www.huaweicloud.com/intl/en-us/price.html | Scrape/API | Variable | HTML/JSON | 40+ | 🟠 v0.2.0 TBD |
| **Scaleway** | https://pricing.scaleway.com/api/v1/products | Public | 1000 req/h | JSON | 6+ | 🟠 v0.2.0 TBD |
| **Contabo** | https://contabo.com/en/pricing/ | Scrape | Variable | HTML | 5+ | 🟠 v0.2.0 TBD |
| **Hetzner** | https://www.hetzner.cloud/pricing | Scrape/GraphQL | Variable | HTML/JSON | 6+ | 🟠 v0.2.0 TBD |
| **IONOS** | https://www.ionos.com/en-us/hosting/cloud/vps | Scrape | Variable | HTML | 6+ | 🟠 v0.2.0 TBD |
| **OVHCloud** | https://www.ovh.com/world/public-cloud/pricing/ | Public API | 100 req/s | JSON | 6+ | 🟠 v0.2.0 TBD |

**Deliverable:** `docs/pricing-api-research/v0.2.0-provider-analysis.md` with:
- Endpoint discovery (official vs. reverse-engineered)
- Auth requirement mapping
- Response schema examples
- Rate-limit & cost implications
- Fallback strategy (cached docs link if API unavailable)

### Phase 2: Skill Extension (Week 2)

Extend `skills/finops/finops-cloud-price-advisor/SKILL.md`:
- Add 4 new provider sections (Scaleway, Gandi, Alibaba, Tencent)
- Each section specifies:
  - Supported resource types (compute, storage, networking, database)
  - Currency handling (SGD, CNY, EUR vs. USD conversion)
  - Region mapping (Scaleway: eu-fr, eu-nl; Alibaba: cn-*, ap-*; Tencent: ap-*)
  - Pricing components (on-demand, sustained-use discounts where available)
- Update required input schema: add `provider` field with enum [aws, azure, oci, scaleway, gandi, alibaba, tencent]
- Update response shape: add provider-specific footnotes (e.g., Alibaba RMB pricing with USD conversion rate + timestamp)

**Deliverable:** Updated SKILL.md with 7 provider paths, test fixtures for 3 new providers

### Phase 3: Agent Metadata & Routing (Week 2)

Update `agents/finops/finops-cloud-price-advisor-agent/`:
- **metadata.json**:
  - Bump version to `0.2.0`
  - Add official_docs links for Scaleway, Gandi, Alibaba, Tencent pricing pages
  - Add `provider_coverage: ["aws", "azure", "oci", "scaleway", "gandi", "alibaba", "tencent"]` field
- **AGENT.md**: Update Focus section to mention all 7 providers
- **Maestro taxonomy.json**:
  - Add keywords for EU region pricing (scaleway, gandi, eu-fr, eu-nl, european pricing, paris, amsterdam)
  - Add keywords for APAC region pricing (alibaba, tencent, cn-, ap-, singapore, tokyo, sydney, chinese pricing, renminbi)
  - Ensure cloud-price-advisor domain matches all 7 providers
- **harnesses/**: No changes required — agent detects provider from user input and skill handles integration

**Deliverable:** Updated metadata, AGENT.md, maestro taxonomy.json with full provider keyword coverage

### Phase 4: Integration Testing (Week 3)

Create `tests/fixtures/finops-cloud-price-advisor/`:
- **Input fixtures** (10 tests):
  - 2 Scaleway: eu-fr instance pricing, storage pricing
  - 2 Gandi: VPS, CDN pricing
  - 2 Alibaba: ECS (Elastic Compute), RDS pricing in CNY
  - 2 Tencent: CVM, TencentDB pricing in CNY
  - 2 comparative: AWS vs. Scaleway EU regions; Azure vs. Alibaba APAC regions
- **Expected outputs**: Pricing tables with correct currency, region, API response timestamp label
- **Grader**: Validate that:
  - Each provider's API is called (or fallback triggered)
  - Currency conversions are labeled (live-price / documentation-based)
  - Regions are correctly mapped to provider availability
  - No secrets (API keys) appear in output even if user provided them

**Deliverable:** 10 passing fixtures + grader validation script

### Phase 5: Eval Harness & Sign-Off (Week 3)

Run formal eval-harness with:
- **Schema validation**: All 7 providers present in metadata, official_docs, provider_coverage
- **API integration tests**: Live-fetch success rate for each provider (allow 1 timeout fallback per provider)
- **Routing tests**: 4 maestro fixtures (1 per new provider region) route to finops-cloud-price-advisor-agent
- **Security posture**: No secrets, no cloud credentials accepted; API keys for Gandi only accepted if user explicitly provides them
- **Catalog alignment**: agent/skill versions bumped to 0.2.0; all references updated

**Target:** 48/48 eval checks passing (100% pass@1)

---

## Provider-Specific Notes

### Scaleway (EU)
- **Official API:** https://pricing.scaleway.com/api/v1/products
- **Regions:** Paris (eu-fr-par), Amsterdam (nl-ams)
- **Currency:** EUR
- **Agents in repo:** 6 (cost-optimizer, iam-policy-review, kapsule-platform-operator, live-kapsule-rollout-guard, maestro, network-architect)
- **Unique:** Transparent, machine-readable JSON API — lowest integration effort
- **Fallback:** Official pricing page https://www.scaleway.com/en/pricing/

### Alibaba Cloud (Asia)
- **API:** Reverse-engineered or HTML scrape — no official public unauthenticated API
- **Regions:** cn-* (mainland China), ap-* (Singapore, Tokyo, Sydney)
- **Currency:** CNY (Renminbi) + USD equivalent (live conversion required)
- **Agents in repo:** 40+ (ack-platform, actiontrail, analyticdb, certificate-manager, compliance, cost-finops, devops, ecs, function-serverless, iac-review, kms, landing-zone, live-guards ×5, maestro, migration, mse, network, observability, oss-storage, polardb, ram-iam, registry, resilience, security-center, serverless, solution-architect, support, ticket-triage, waf ×3)
- **Unique:** Pricing region-specific; RMB to USD conversion requires live exchange rate (use XE or ECB)
- **Fallback:** Alibaba Cost Calculator or cached pricing table

### Huawei Cloud (Asia)
- **API:** Reverse-engineered or HTML scrape — no official public unauthenticated API
- **Regions:** cn-* (China), ap-* (Asia-Pacific), eu-* (Europe)
- **Currency:** CNY + USD equivalent (live conversion required)
- **Agents in repo:** 40+ (cce-platform, certificate-manager, codearts, compliance-sovereignty, cost-finops, daily-ops, dew-kms, drs-replication, dws-dli, ecs, event-driven, functiongraph, gaussdb, iac-review, iam, ief-edge, landing-zone, live-guards ×5, load-balancer, maestro, modelarts, network, obs-storage, observability, registry, resilience, secmaster, serverless, solution-architect, support, ticket-triage, waf ×3)
- **Unique:** Pricing model similar to Alibaba; regional variants; requires scrape or undocumented API
- **Fallback:** Huawei Cost Calculator or cached pricing table

### Contabo (Europe)
- **API:** No official API; pricing via web scrape only
- **Regions:** Germany (eu-de), USA (us-west), Singapore (ap-sg)
- **Currency:** EUR, USD
- **Agents in repo:** 5 (capacity-planner, cost-optimization-analyst, live-instance-lifecycle-guard, live-storage-operations-guard, maestro, security-hardening)
- **Unique:** Fixed pricing model; no per-second billing; ideal for long-running workloads
- **Fallback:** Contabo pricing page https://contabo.com/en/pricing/

### Hetzner (Europe)
- **API:** GraphQL + web scrape hybrid; no official public REST API
- **Regions:** eu-de, eu-fi (Finland), us-west
- **Currency:** EUR, USD
- **Agents in repo:** 6 (capacity-planner, cost-optimization-analyst, infrastructure-reviewer, live-firewall-rule-guard, live-server-lifecycle-guard, maestro)
- **Unique:** Transparent, predictable pricing; strong EU market presence; fixed server pricing
- **Fallback:** Hetzner Cloud pricing page https://www.hetzner.cloud/pricing

### IONOS (Europe)
- **API:** No official API; pricing via web scrape only
- **Regions:** eu-* (multiple EU datacenters)
- **Currency:** EUR, USD
- **Agents in repo:** 6 (cost-optimization-analyst, datacenter-designer-reviewer, kubernetes-platform-operator, live-database-lifecycle-guard, maestro, security-compliance-reviewer)
- **Unique:** VPS-centric pricing; strong EU presence; managed services integration
- **Fallback:** IONOS pricing page https://www.ionos.com/en-us/hosting/cloud/vps

### OVHCloud (Europe)
- **API:** Official public API (limited pricing data); web scrape supplementary
- **Regions:** eu-* (multiple EU regions), ca-*, ap-*
- **Currency:** EUR, USD
- **Agents in repo:** 6 (cost-finops-analyst, iam-policy-review, kubernetes-platform-operator, live-kms-key-destruction-guard, maestro, network-architect)
- **Unique:** Transparent API-first approach; strong EU compliance footprint
- **Fallback:** OVH Public Cloud pricing page https://www.ovh.com/world/public-cloud/pricing/

### GCP (Multi-Region) — Reconsideration for v0.2.0
- **API:** https://www.googleapis.com/compute/v1/projects/*/machineTypes (Compute pricing) + Cloud Billing API
- **Regions:** us-*, eu-*, asia-*
- **Currency:** USD, EUR, GBP (currency conversion API required)
- **Agents in repo:** 40+ (alloydb-ai-developer, anthos-multicloud, apigee-platform, bigquery-analyst, cloud-auth, cloud-run-functions, cloudbuild-cicd, compute-engine, cost-finops, daily-ops, data-pipeline, event-driven, firebase-developer, gcs-data-perimeter, gemini-api, gke-platform, iac-review, iam, landing-zone, live-guards ×5, load-balancer, maestro, migration, network, observability, registry, resilience, resource-inventory, secret-kms, security-hardening, serverless, solution-architect, spanner-architect, support, ticket-triage, vertex-ai, vpc-service-controls, waf ×3)
- **Status in v0.1.1:** Intentionally removed per Codex review feedback (agent supports read-only operations; GCP not core FinOps focus)
- **v0.2.0 decision:** Evaluate trade-off of supporting 40+ existing GCP agents vs. narrower AWS/Azure/OCI focus for v0.2.0 release; may defer to v0.3.0

---

## Risk Mitigation

| Risk | Mitigation |
|------|-----------|
| Alibaba/Tencent API instability | Implement HTML scrape as fallback; document fallback via `documentation-based` label |
| Currency conversion drift | Use live exchange rate API (XE, Fixer, or OpenExchangeRates); fallback to ECB/PBoC rates cached in docs |
| Regional pricing variance | Create region-specific fixtures; test eu-fr vs. eu-nl, cn-beijing vs. cn-shanghai |
| Gandi API key management | Require explicit user request; never store/cache keys; label output as `user-provided` credential input |
| Rate limiting on scrape APIs | Add exponential backoff; if Alibaba/Tencent APIs unavailable, emit clear "pricing unavailable — using cached rate" message |

---

## Success Criteria

✅ All 11 providers supported by agent (AWS, Azure, OCI, Alibaba, Huawei, Scaleway, Contabo, Hetzner, IONOS, OVHCloud, ±GCP)
✅ 60/60 eval checks passing (schema, routing, API integration, security, provider-specific scenarios)
✅ Maestro routes provider-specific pricing questions to finops-cloud-price-advisor-agent
✅ No new agents required (same agent handles all providers)
✅ No secrets in output or stored in code (user-supplied API keys accepted only for Gandi-like APIs, never stored)
✅ All prices labeled with provenance (live-price / documentation-based / assumed / excluded)
✅ Currency handling explicit (USD, EUR, CNY, CAD with conversion rate + timestamp where applicable)
✅ 2+ fixtures per new provider (resource types × region variants)
✅ Scrape APIs have exponential backoff + documented fallback behavior

---

## Rollback Plan

If any provider integration fails eval:
- Mark provider as `lifecycle: "beta"` in metadata
- Move provider from `provider_coverage` to `provider_coverage_beta`
- Document known issue and ETA for fix
- Revert agent version to 0.1.1 if >2 providers fail; release 0.2.0-rc.1 instead

---

## Metrics & Observability

Post-release:
- Track pricing API success rate per provider (target: 95% weekly uptime)
- Monitor maestro routing for EU/APAC keywords (target: <100ms response time)
- Flag if any cached pricing is stale (>30 days) and re-verify
- Monthly: validate that no new payment method or rate limit has been imposed by any provider

