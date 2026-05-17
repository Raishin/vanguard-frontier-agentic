# Routing table and domain taxonomy

Use this reference when classifying a task or selecting the right specialist(s).

## Routing table

| Signal keywords | Agent ID | Domain | Live-guard? |
|---|---|---|---|
| consent banner, cookie banner, CMP, consent management platform, tags fire before consent, Consent Mode, GDPR consent, ePrivacy, cookie policy, pre-ticked, reject all, dark pattern banner, tracker disclosure, cross-border transfer, consent record | marketing-consent-data-collection-review-agent | Consent and data-collection posture | No |
| advertising pixel, Meta Pixel, TikTok pixel, Google Ads tag, LinkedIn Insight Tag, conversion event, dataLayer, PII in URL, email in query parameter, form-field capture, advanced matching, enhanced conversions, pixel on health page, PHI leakage, hashed identifier | marketing-pixel-data-leakage-review-agent | Advertising-pixel personal-data leakage | No |
| OAuth grant, connected app, API key, integration scope, CRM role, marketing automation permission, martech access, least privilege, over-permissioned connector, stale token, refresh token, shared admin key, token rotation, bulk export permission, access review | martech-access-governance-review-agent | Martech access governance | No |
| Global Privacy Control, GPC, opt-out signal, Do Not Sell, opt-out honored, GPC not propagated, tag bypass on opt-out, server-side opt-out, CPPA enforcement, opt-out cookie | marketing-gpc-signal-honoring-review-agent | GPC opt-out signal honoring | No |
| SPF, DKIM, DMARC, BIMI, sender authentication, DNS TXT record, p=none, DKIM selector, SPF lookup limit, email spoofing, bulk sender, deliverability, VMC, CMC | email-sender-authentication-review-agent | Email sender authentication | No |
| ads.txt, app-ads.txt, sellers.json, SupplyChain Object, programmatic supply chain, unauthorized reseller, domain spoofing, IVT, invalid traffic, authorized seller, DIRECT, RESELLER | programmatic-supply-chain-integrity-review-agent | Programmatic supply-chain integrity | No |
| ad targeting, audience targeting, Advantage+, lookalike targeting, protected class, discrimination, Fair Housing, ECOA, disparate impact, targeting fairness, automated bidding bias, health-proxy segment | ai-advertising-targeting-fairness-review-agent | AI ad-targeting fairness | No |
| EU AI Act, AI Act, high-risk AI, prohibited AI practice, AI risk tier, conformity assessment, AI system classification, Annex III, human oversight, profiling AI, fundamental-rights impact | eu-ai-act-marketing-system-review-agent | EU AI Act marketing-system classification | No |
| custom audience, lookalike audience, audience upload, customer match, matched audience, hashing, SHA-256, MD5, field mapping, audience consent basis, data upload to ad platform | lookalike-audience-upload-compliance-review-agent | Lookalike-audience upload compliance | No |
| email list, list hygiene, suppression list, consent record, consent timestamp, data retention, storage limitation, CASL record-keeping, list segment, last-engagement date, re-permission | marketing-email-list-retention-review-agent | Email list retention and hygiene | No |
| influencer, creator, endorsement, material connection, disclosure, FTC Endorsement Guides, sponsored post, gifted product, #ad, disclosure placement, creator brief | influencer-disclosure-compliance-review-agent | Influencer disclosure compliance | No |
| conversion flow, sign-up flow, upsell, free trial enrollment, cancellation flow, dark pattern, pre-checked box, false urgency, countdown timer, negative option, ROSCA, hard to cancel | marketing-conversion-flow-dark-pattern-review-agent | Conversion-flow dark patterns | No |
| analytics config, GA4, BigQuery export, data minimization, event parameter, user property, analytics retention, user-scoped dimension, persistent user ID, raw event export | analytics-data-minimization-review-agent | Analytics data minimization | No |

## Domain taxonomy

| Domain | Keywords and signals |
|---|---|
| `consent` | CMP banner configuration, tag-manager consent gating, Consent Mode wiring, cookie policy disclosure, consent records, cross-border transfer mechanisms |
| `pixel-leakage` | Advertising and social pixels, conversion event payloads, `dataLayer` values, URL-parameter PII, form-field auto-capture, pixels on sensitive pages, identifier hashing |
| `access-governance` | OAuth connected apps and scopes, API keys, CRM and marketing-automation roles, shared and stale credentials, token rotation, integration ownership, bulk-export spread |
| `gpc` | Global Privacy Control signal path, opt-out propagation to tag execution, server-side conversion-API bypass, pre-existing-GPC handling |
| `email-auth` | SPF, DKIM, DMARC, BIMI DNS records, policy enforcement level, alignment, SPF lookup limits, ESP subdomain authentication |
| `supply-chain` | ads.txt and app-ads.txt entries, sellers.json records, SupplyChain Object, authorized resellers, domain-spoofing exposure, invalid-traffic risk |
| `targeting-fairness` | Ad-platform audience targeting, AI delivery optimization, protected-class proxies, disparate impact, Fair Housing and ECOA exposure |
| `ai-act` | EU AI Act risk-tier classification, prohibited practices, high-risk Annex III triggers, human-oversight adequacy, profiling, conformity assessment |
| `audience-upload` | Custom and lookalike audience uploads, hashing adequacy, PII field scope, consent basis for the seed list, platform data-sharing restrictions |
| `list-retention` | Email list segment metadata, consent-record completeness, suppression-list coverage, retention schedules, storage limitation |
| `influencer` | Influencer campaign briefs and contracts, material-connection disclosure, disclosure placement and conspicuousness, honest-opinion suppression |
| `conversion-dark-patterns` | Sign-up, upsell, trial-enrollment and cancellation flows, pre-checked options, asymmetric accept/decline weight, false urgency, negative-option cancellation friction |
| `analytics-minimization` | Analytics platform schema, event parameters, user properties, raw-data export configuration, data-retention periods, internal collection scope |

## Dispatch examples

### Example 1: Single-domain question

**User request:** "Does our DMARC record actually block spoofed mail, or is it just monitoring?"

**Routing:**
```
Route: email-sender-authentication-review-agent
Reason: Task is a pure sender-authentication question about DMARC enforcement posture — single email-auth domain.
Mode: single
```

`email-sender-authentication-review-agent` reviews the DNS records and reports whether the DMARC policy enforces or only monitors.

---

### Example 2: Two domains in parallel

**User request:** "Check whether our opt-out signal actually stops the pixels, and review our cookie banner for dark patterns."

**Routing:**
```
Route: marketing-gpc-signal-honoring-review-agent, marketing-consent-data-collection-review-agent
Reason: Task spans GPC opt-out signal propagation and consent-banner design — two distinct governance domains.
Mode: parallel (2)
```

---

### Example 3: Multi-domain posture review (ceiling enforced)

**User request:** "Full audit: consent banner, pixel leakage, who has CRM access, and whether our audience uploads are hashed correctly."

**Routing:**
```
Route: marketing-consent-data-collection-review-agent, marketing-pixel-data-leakage-review-agent, martech-access-governance-review-agent, lookalike-audience-upload-compliance-review-agent
Reason: Task spans four distinct governance domains — consent, pixel leakage, access governance, and audience-upload compliance.
Mode: parallel (4)
```

Four specialists is the hard ceiling. If a request implies more than four domains, dispatch the four highest-risk and tell the caller which domains were deferred to a follow-up.

---

### Refused request: live mutation

**User request:** "Revoke the SurveyTool OAuth grant and republish the tag container with the GPC fix."

**Routing:**
```
Route: REFUSED
Reason: This request requires live writes — an OAuth revocation and a tag-container publish. No live-guard agents exist in v1. Escalate to a human operator.
Mode: N/A
```

No agent in this provider executes mutations. Specialists produce the scoped recommendation; the human operator applies it.

---

## Provenance label protocol

Every value produced by a routed specialist must carry one of these labels:

| Label | Meaning |
|---|---|
| `live-evidence` | Observed in the sanitized configuration or artifact the user provided in this session |
| `documentation-based` | Sourced from official regulation or platform documentation |
| `inference` | Derived by the specialist from inputs using documented methodology |
| `excluded` | Data intentionally excluded from the output, and why |
