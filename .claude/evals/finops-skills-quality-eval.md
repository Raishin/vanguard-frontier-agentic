[MODEL GRADER: finops-skills-quality]
Date: 2026-05-13

Skill: finops-maestro
  Clarity: 5/5 — Purpose (classify and dispatch only, never answer directly) unambiguous; routing table, dispatch examples, refused-mutation example, and handoff packet format all present
  Least-privilege: 5/5 — Agent Skill Read Grep Glob; WebFetch absent; dispatch limited to named catalog agent IDs only
  Security posture: 5/5 — Unconditional credential refusal; safety-checklist.md enumerates AKIA, Azure client secrets, GCP SA JSON, OCI API keys, bearer tokens; injection-attempt handling documented
  FOCUS compliance: N/A (routing layer; enforces provenance protocol on routed output)
  Provenance labeling: N/A
  Overall: PASS
  Notes: Routing table covers 3 agent IDs — must be kept current as catalog grows

Skill: fetch-foundation-model-pricing
  Clarity: 5/5 — Two modes, pricing dimensions table, response minimum section, and fallback behavior explicit
  FOCUS compliance: 5/5 — BilledCost, EffectiveCost, ChargeCategory (Usage), ServiceCategory (AI and Machine Learning), SkuId, SkuPriceId, ResourceId; token-economics.md extends to UsageType, UsageQuantity, ListCost
  Least-privilege: 4/5 — Read Grep Glob WebFetch appropriate; Glob is low-risk
  Security posture: 5/5 — Unconditional: "No credentials required or accepted... Never ask for API keys, billing account IDs, or tenant-specific data"
  Provenance labeling: 5/5 — Four-tier system (live-price, documentation-based, assumed, excluded) with mandatory source URL and ISO 8601 timestamp
  Overall: PASS

Skill: kubernetes-allocation-report
  Clarity: 5/5 — Six-step allocation methodology, required inputs enumerated, complete response shape with FOCUS column annotations; idle and total cluster rows required
  FOCUS compliance: 5/5 — ChargeCategory (Usage), ServiceCategory (Containers), ServiceName (Kubernetes), ResourceId, BilledCost; EffectiveCost gap documented with excluded label
  Least-privilege: 4/5 — Read Grep Glob WebFetch correct (live node pricing); no Bash/kubectl
  Security posture: 5/5 — "No kubeconfig, bearer token, service account JWT, or cloud IAM credential accepted"; "does not connect to the Kubernetes API server"
  Provenance labeling: 5/5 — All cost figures carry provenance labels
  Overall: PASS

Skill: rightsize-recommendation
  Clarity: 5/5 — Explicit formulas, templated response shape, five Karpenter eligibility conditions in companion doc
  FOCUS compliance: 4/5 — BilledCost (current), EffectiveCost (projected), ChargeCategory (Usage), ServiceCategory (Containers) mapped; adequate for pure-math skill
  Least-privilege: 5/5 — Read Grep Glob only; WebFetch correctly ABSENT; stated in frontmatter and body
  Security posture: 5/5 — Unconditional enumerated refusal of credentials, kubeconfig, bearer tokens, service account JWTs
  Provenance labeling: N/A (uses assumed/excluded label system correctly; no price fetching)
  Specific checks:
    Headroom formula (p95+20% request, p99+30% limit): PRESENT — verbatim in methodology table for CPU and memory
    WebFetch correctly absent: PASS
    Karpenter consolidation eligibility criteria: PRESENT and complete — all five conditions; karpenter-consolidation.md has detailed sub-cases
  Overall: PASS

Skill: carbon-cost-pair
  Clarity: 5/5 — Decision tree, regulatory context with specific article references, data-lag figures per provider
  FOCUS compliance: 4/5 — Uses FOCUS tags (Tags/kgco2e, Tags/carbon_confidence, Tags/carbon_source); no native FOCUS carbon columns (correct per FOCUS v1.2 spec); minor: full FOCUS row pairing not shown
  Least-privilege: 4/5 — Read Grep Glob WebFetch appropriate; sources.md routes partially-authenticated sources to public fallbacks
  Security posture: 5/5 — Unconditional refusal includes sustainability API tokens in addition to standard cloud credentials
  Provenance labeling: 5/5 — Three-tier (vendor-published, third-party, estimated) with source URL, ISO 8601 timestamp, and data period in every output block
  Specific checks:
    Scope 2 market-based default: PRESENT (first operating rule; methodology.md explains market-based vs location-based)
    CSRD/SEC climate disclosure context: PRESENT (CSRD Article 29a/ESRS E1, SEC Climate Rule, GHG Protocol cited)
    kgCO2e units: PRESENT throughout
  Overall: PASS

Skill: focus-spec-normalizer
  Clarity: 5/5 — Three-step mapping behavior, vendor auto-detection by column signatures, gap-note format, five-item response shape with realistic null handling example
  FOCUS compliance: 5/5 — FOCUS v1.2 throughout; focus-columns.md covers required, conditional, and recommended columns; vendor-mapping.md has complete four-vendor tables including BilledCost, EffectiveCost, ServiceCategory, ServiceName, ChargeCategory, SkuPriceId, ResourceId
  Least-privilege: 5/5 — Read Grep Glob only; WebFetch correctly ABSENT; "No network access" in Purpose section
  Security posture: 5/5 — Unconditional refusal of credentials, billing IDs, tenant IDs, service principals; instructs users to paste de-identified/sample data
  Provenance labeling: N/A (uses mapped/derived/null column-level provenance — correct for normalization skill)
  Specific checks:
    Vendor adapters (AWS CUR, Azure Cost Management, GCP Billing Export, OCI): ALL FOUR PRESENT
    FOCUS v1.2 column coverage: Complete including required columns (BillingAccountId, ConsumedQuantity, ListCost, PricingCategory, PricingQuantity, PricingUnit)
    WebFetch correctly absent: PASS
  Overall: PASS

Summary: 6/6 PASS, 0/6 NEEDS WORK
Critical gaps: None
Cross-cutting strengths:
  - Security posture consistently strong — explicit unconditional refusal with per-format enumeration
  - Provenance labeling is first-class in every data-producing skill
  - WebFetch boundary correctly drawn (pure-math/offline exclude it; live-price skills include it with rationale)
  - FOCUS v1.2 references accurate; gaps documented with null handling and resolution notes
Status: SHIP
