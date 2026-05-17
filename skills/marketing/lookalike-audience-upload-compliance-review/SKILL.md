---
name: lookalike-audience-upload-compliance-review
description: Use this skill when reviewing custom-audience and lookalike-audience upload specifications for hashing adequacy, PII field scope, consent-basis validity, and platform data-sharing restrictions before the upload is submitted to Meta, Google, LinkedIn, or TikTok. Trigger when a user provides an audience upload field-mapping specification (CSV schema or platform upload template), declared hashing method, consent-basis documentation, or originating list segment metadata — or when they ask whether their customer list upload or lookalike seed list is compliant with GDPR, CCPA/CPRA, or platform terms before uploading.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-17"
  category: data
  lifecycle: experimental
---

# Lookalike Audience Upload Compliance Review

## Purpose
This skill reviews custom-audience and lookalike-audience upload specifications for hashing adequacy, PII field scope, consent-basis validity, and platform data-sharing restrictions before the upload is submitted to Meta, Google, LinkedIn, or TikTok. Customer list uploads are one of the most common data-sharing vectors in marketing operations: a brand transmits personal data — email, phone, name, address — to an ad platform under the guise of audience matching. The legal basis, the scope of consent granted, the minimality of the field set, and the reversibility of the hashing method all determine whether the upload is a lawful data-sharing arrangement or an unauthorized third-party disclosure that violates GDPR Article 5 and 6, CCPA/CPRA §1798.100, and platform terms. The review catches underhashed identifiers, oversized field sets, consent-scope mismatches, and re-identification surfaces before the upload fires.

## Lean operating rules
- Treat email addresses, phone numbers, or other direct identifiers hashed with MD5 rather than SHA-256 (or better) as HIGH — MD5 is trivially reversible via rainbow table for common email formats and does not constitute adequate pseudonymization under GDPR Article 5(1)(f).
- Treat a seed list for a financial-services, insurance, or healthcare lookalike that includes customers who consented only to service communications (transactional consent) as HIGH — sharing that list with an ad platform for advertising targeting exceeds the consent scope, constituting unauthorized "sharing" of personal information under CPRA §1798.100 and a purpose-limitation violation under GDPR Article 5(1)(b).
- Treat a field mapping that includes home postal code combined with email and phone as HIGH — the combination creates a high-confidence re-identification surface beyond what the matching algorithm requires, violating the data-minimization principle under GDPR Article 5(1)(c) and FTC data-minimization guidance.
- Treat unhashed upload of any direct identifier (plain-text email, phone, name) as HIGH — no platform terms permit plain-text PII upload, and transmission in the clear is an unequivocal data breach of the identifier.
- Treat the absence of a documented lawful basis (GDPR Article 6) for the data-sharing arrangement — neither the original collection basis nor a separate legitimate-interest or consent basis for sharing with the ad platform — as HIGH.
- Treat lookalike seed lists that include individuals in jurisdictions where the operator has no data-processing agreement with the ad platform (e.g., EU residents shared with a non-adequate-country platform without SCCs) as HIGH — the transfer itself is unlawful under GDPR Chapter V.
- Flag field sets that include date of birth, precise geolocation, or transaction-level history where only email and phone are needed for matching as MEDIUM — over-inclusion violates data minimization and increases re-identification risk.
- Flag platform-specific restrictions violated by the field mapping — e.g., Meta's Customer List Custom Audience terms prohibit health, financial account data, and sensitive categories — as MEDIUM when inclusion is marginal but potentially violating.
- Flag the absence of a documented retention or deletion schedule for the matched and unmatched records on the platform side as MEDIUM — GDPR Article 5(1)(e) requires storage limitation; the user should confirm platform-side deletion timelines.
- Do not recommend uploading any field not strictly needed for the matching objective; default to the minimum field set (normalized lowercase email SHA-256 hashed) unless the user explicitly requires phone or name for match-rate reasons.
- Label every finding with evidence basis: field-mapping spec provided, hashing method declared, consent documentation provided, or inference from missing information.

## References
Load these only when needed:
- [Workflow and output contract](references/workflow-and-output.md) — use when executing the full review or formatting the final answer.

## Response minimum
Return, at minimum:
- Hashing adequacy assessment (algorithm, where hashing occurs, platform requirement alignment)
- PII field-scope and data-minimization assessment (fields included vs. fields needed)
- Consent-basis validity assessment (original collection basis, scope for ad-platform sharing)
- Cross-border transfer assessment (GDPR Chapter V if EU data subjects are in the list)
- Platform-specific restriction check (Meta, Google, LinkedIn, TikTok terms)
- Re-identification surface assessment (field combination risk)
- Severity-labelled finding list (critical / high / medium / low)
- Safe next actions
