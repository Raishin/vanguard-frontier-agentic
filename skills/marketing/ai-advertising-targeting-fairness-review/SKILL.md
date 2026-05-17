---
name: ai-advertising-targeting-fairness-review
description: Use this skill when reviewing ad-platform audience targeting configurations and declared AI feature usage for protected-class discrimination risk. Trigger when a user provides a Meta Ads Manager audience definition, Google Ads targeting layer export, DSP deal config, or any ad platform audience spec annotated with AI features enabled (Advantage+ Audience, broad match, automated bidding, lookalike seeds). Use when a campaign is in housing, credit, employment, or insurance verticals, or when automated bidding or AI audience expansion is active on any campaign reaching the US or EU and the user needs to assess Fair Housing Act, ECOA, or EU AI Act Article 5 exposure.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-17"
  category: ai
  lifecycle: experimental
---

# AI Advertising Targeting Fairness Review

## Purpose
This skill reviews ad-platform audience targeting configurations and declared AI feature usage for protected-class discrimination risk under the Fair Housing Act (42 U.S.C. §3604), the Equal Credit Opportunity Act (ECOA), and EU AI Act Article 5. Ad platforms increasingly offer AI-driven audience expansion features — Meta Advantage+ Audience, Google broad match and Performance Max, DSP algorithmic deal targeting — that optimize delivery based on historical conversion patterns. When historical converters skew along protected-class lines (race, sex, age, national origin, familial status, disability, religion), algorithmic optimization propagates that skew without explicit intent. The review examines declared AI feature usage, audience seed composition, interest-segment proxy risk, and the absence of protected-category exclusion declarations on special-category campaigns before the configuration ships.

## Lean operating rules
- Treat Meta Advantage+ Audience enabled on a housing, credit, employment, or insurance campaign with no declared protected-category exclusions as HIGH — the system expands targeting beyond the declared audience using engagement signals that may correlate with race, sex, or national origin.
- Treat interest-based segments that function as proxies for health conditions, religion, national origin, or familial status used on an insurance or financial-services campaign as HIGH — proxy targeting on protected classes is substantively equivalent to explicit targeting under FHA and ECOA case law.
- Treat automated bidding (Target CPA, Target ROAS, Smart Bidding) optimizing a credit-offer, rental, or employment campaign on lookalike audiences seeded from historical converters as HIGH — disparate impact is propagated algorithmically when the seed population reflects historical discriminatory patterns.
- Treat any AI-generated audience expansion (broad match, Performance Max audience signals, DSP algorithmic reach extension) active on a special-category campaign (housing, credit, employment, insurance) with no fairness audit trail as HIGH — the optimization objective does not include disparate-impact minimization.
- Treat geofencing or geographic exclusion zones that closely follow racially or ethnically concentrated neighborhood boundaries on a housing or credit campaign as HIGH — geographic redlining is prohibited under FHA regardless of whether intent is declared.
- Treat the absence of a Special Ad Category declaration on a Meta campaign reasonably classifiable as housing, employment, or credit as HIGH — the declaration unlocks mandatory fairness restrictions; omitting it circumvents them.
- Flag automated bidding that optimizes on a conversion event defined as a past purchase or application when the historical converter population is not documented for demographic representativeness as MEDIUM — undocumented seed bias is a disparate-impact risk even when not yet proven.
- Flag interest segments that include health-condition or medication-related categories on campaigns not in the healthcare vertical as MEDIUM — health proxies reach users based on inferred sensitive characteristics.
- Flag AI feature disclosures that are absent or vague (e.g., "algorithmic optimization enabled" with no named feature, no version, no opt-out path) as MEDIUM — EU AI Act Article 13 and FTC guidance require meaningful transparency.
- Do not recommend disabling AI features without naming the performance impact and the manual alternative that preserves reach.
- Label every finding with evidence basis: audience spec provided, AI feature declaration provided, documentation-based, or inference from missing config.

## References
Load these only when needed:
- [Workflow and output contract](references/workflow-and-output.md) — use when executing the full review or formatting the final answer.

## Response minimum
Return, at minimum:
- AI feature inventory (named features enabled per campaign, evidence basis)
- Special-category campaign detection (housing, credit, employment, insurance)
- Protected-class proxy segment assessment (interest segments, lookalike seeds)
- Algorithmic disparate-impact assessment (bidding, audience expansion)
- Special Ad Category declaration check (Meta) or equivalent platform declaration
- Severity-labelled finding list (critical / high / medium / low)
- Safe next actions
