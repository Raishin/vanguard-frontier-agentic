---
name: frontend-finops-cost-to-serve-review
description: Build a cost-to-serve model covering CDN egress, SSR/edge compute, image transformation, and CI build-minute spend for a frontend surface, and rank remediation options by dollar savings weighed against Core Web Vitals and security impact, without treating cost-cutting and security/performance as unrelated trade-offs.
allowed-tools: Read Grep Glob WebFetch
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-07-02"
  category: finops
---

# Frontend FinOps Cost-to-Serve Review

## Purpose

Frontend architecture choices — SSR vs. static generation, ISR revalidation cadence, image-pipeline design, third-party script sprawl, CI build-minute consumption — are cloud-spend decisions as much as they are UX decisions, but they are almost never modeled that way: performance is reviewed by one team, cloud spend by another, and the causal link between them goes unmeasured. This skill exists to build a defensible cost-to-serve model for a frontend surface (CDN egress, SSR/edge compute, image transform, build-minutes), tie every dollar figure to an explicit evidence level, and rank remediation options by savings-to-risk ratio — never presenting a modeled estimate as an audited invoice, and never treating cost reduction as separable from performance and security posture.

## When to use

Use this skill when the user asks to:

- estimate the CDN egress, SSR/edge-compute, or image-transform cost of a frontend surface,
- evaluate whether an SSR/ISR/edge-function architecture choice is cost-appropriate at current or projected traffic,
- identify which third-party scripts or dependencies are the largest cost/performance line items,
- rank cost-reduction options by dollar impact versus Core Web Vitals or security trade-off,
- explain why a frontend surface's cloud bill grew disproportionately to its traffic.

## Context7 Documentation Protocol

Before making any framework-specific claim about caching, revalidation, rendering mode, or image-optimization behavior (Next.js, or any other framework named in the task), resolve the library via Context7 (`resolve-library-id`) and query current docs (`query-docs`) rather than relying on training-data memory. Frameworks change caching/revalidation defaults across major versions (for example, Next.js has changed default `fetch` caching behavior between major versions), and a cost model built on a stale caching assumption will misstate invocation counts by an order of magnitude. Label every framework-behavior claim as `context7-grounded (as of <library-id>/<version if resolved>)`, `documentation-based (unverified against Context7)`, or `inference` — never state framework caching/billing behavior as fact without one of these labels. If Context7 has no coverage for a named library or version, say so explicitly and fall back to official vendor docs, marking the claim uncertain.

## Lean operating rules

- Always state the evidence level of every dollar figure: `billing-data-verified` (user supplied actual invoice/usage export), `modeled-from-public-pricing` (calculated from published rate cards and estimated volume), or `inference` (no volume data, rough order of magnitude only). Never present a modeled estimate as an audited number.
- Ground SSR/ISR/edge-invocation-count assumptions in the actual framework's documented caching/revalidation behavior (via Context7/official docs) before estimating compute cost — invocation-shape assumptions are the single biggest source of cost-model error. Time-based revalidation (stale-while-revalidate) and on-demand revalidation (tag/path invalidation) produce fundamentally different invocation curves; do not conflate them.
- Do not recommend removing a script, feature, or rendering mode purely because it is expensive without checking whether it drives revenue (checkout, support chat, personalization) — cost-to-serve is a trade-off model, not a cost-minimization mandate.
- Never recommend removing a named security control (CSP, WAF rule, image-pipeline malware/content scan, TLS termination tier, bot-mitigation layer) to cut cost without flagging it as requiring explicit security-owner approval — cost review is not a backdoor to loosen the security posture reviewed elsewhere.
- Distinguish traffic-linear cost growth (predictable, budgetable) from non-linear cost growth (e.g., uncached per-request SSR, unbounded on-demand image-transform variants, retry storms) and flag non-linear growth as an urgent architectural risk regardless of current dollar total — a small bill growing 3x per traffic-doubling is a bigger red flag than a large flat bill.
- Do not treat a public cloud/CDN price sheet as guaranteed pricing for the user's account: committed-use discounts, negotiated enterprise rates, and regional price variance can change real cost by 30-70%. Label public-rate-card math accordingly and ask for a billing export when precision matters.
- Load `references/ssr-isr-invocation-cost-modeling.md` only when the SSR/ISR/edge-function invocation shape is the primary cost driver being modeled.
- Load `references/third-party-script-cost-attribution.md` only when ranking third-party scripts/dependencies by cost and performance impact against business value.

## References

Load these only when needed:

- [SSR/ISR invocation cost modeling](references/ssr-isr-invocation-cost-modeling.md) — use when modeling edge/SSR compute cost, grounding invocation-count assumptions in the framework's actual caching/revalidation behavior, and distinguishing linear from non-linear cost-growth patterns.
- [Third-party script cost attribution](references/third-party-script-cost-attribution.md) — use when ranking third-party scripts/dependencies by their bundle-weight, request-count, and CDN-egress contribution against their measured business value.

## Response minimum

Return, at minimum:

- the cost-to-serve breakdown by category (CDN egress, SSR/edge compute, image transform, CI build-minutes) with an explicit evidence level per figure,
- dollar cost per 1,000 pageviews (or per 1,000 requests) at current or stated traffic,
- a ranked remediation list, each item with estimated dollar savings and its stated Core Web Vitals or security trade-off,
- an explicit flag on any non-linear cost-growth risk found, independent of current dollar total,
- an explicit flag on any recommendation that touches a named security control, stating it requires named-owner sign-off before action.
