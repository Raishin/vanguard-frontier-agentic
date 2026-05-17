---
name: marketing-consent-data-collection-review
description: Use this skill when reviewing a marketing site's consent and data-collection posture — cookie/consent banner (CMP) configuration, tag-manager container exports, Google Consent Mode wiring, or a cookie policy. Trigger when a user provides a CMP configuration, a tag manager container JSON, a consent-banner screenshot description, or asks whether their marketing tracking is GDPR/CCPA/ePrivacy compliant, whether tags fire before consent, or whether their opt-out path is valid.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-17"
  category: compliance
  lifecycle: experimental
---

# Marketing Consent and Data-Collection Review

## Purpose
This skill reviews the consent and data-collection layer of a marketing site for regulatory correctness, coverage gaps, and dark-pattern risk. Marketing analytics and advertising tags are a primary enforcement target under GDPR, the ePrivacy Directive, and US state privacy laws (CCPA/CPRA and successors). A tag that fires before a consent signal, a banner with no symmetric reject control, or a missing "Do Not Sell or Share" path converts routine marketing instrumentation into a regulatory liability and a class-action surface. The review catches consent-gating failures, banner dark patterns, Consent Mode misconfiguration, undeclared trackers, and cross-border transfer gaps before they reach production.

## Lean operating rules
- Treat any analytics or advertising tag that fires before an explicit opt-in consent signal (in a GDPR/ePrivacy-scoped jurisdiction) as HIGH — prior consent is required before non-essential storage or access.
- Treat a consent banner with no reject control, or a reject control that takes more clicks or less visual weight than accept, as HIGH — non-symmetric choice is a recognized dark pattern and invalidates consent.
- Treat pre-ticked consent checkboxes or "consent by continued browsing / scrolling" as HIGH — neither is freely given, specific, informed, and unambiguous consent.
- Treat the absence of a "Do Not Sell or Share My Personal Information" link or an equivalent opt-out preference signal path (Global Privacy Control honoring) as HIGH for sites serving California or other opt-out-regime traffic.
- Treat Google Consent Mode left in its default-granted state, or implemented without `wait_for_update`, as HIGH — tags transmit before the consent decision is captured.
- Treat trackers observed in the tag container that are not disclosed in the cookie policy or consent vendor list as HIGH — undisclosed processing has no lawful basis.
- Flag a single global consent toggle with no per-purpose granularity (analytics vs advertising vs personalization) as MEDIUM — purpose-bundled consent is not specific.
- Flag consent records with no retention of timestamp, scope, and consent-string version as MEDIUM — without a consent record the controller cannot demonstrate compliance.
- Flag advertising tags that send data to ad networks in non-EEA jurisdictions with no referenced transfer mechanism as MEDIUM.
- Do not recommend disabling a tag without naming the marketing measurement it supports and the residual attribution loss.
- Label every finding with evidence basis: configuration provided, policy text provided, documentation-based, or inference from missing config.

## References
Load these only when needed:
- [Workflow and output contract](references/workflow-and-output.md) — use when executing the full review or formatting the final answer.

## Response minimum
Return, at minimum:
- Consent-gating findings (tags firing before the consent signal)
- Banner design assessment (symmetry, granularity, dark-pattern checks)
- Opt-out / Global Privacy Control path assessment
- Consent Mode / tag-manager wiring findings
- Tracker-to-policy disclosure gap list
- Cross-border transfer assessment
- Severity-labelled finding list (critical / high / medium / low)
- Safe next actions
