# Routing table and domain taxonomy

Use this reference when classifying a task or selecting the right specialist(s).

## Routing table

| Signal keywords | Agent ID | Domain | Live-guard? |
|---|---|---|---|
| consent banner, cookie banner, CMP, consent management platform, tags fire before consent, Consent Mode, GDPR consent, ePrivacy, cookie policy, pre-ticked, reject all, dark pattern banner, Do Not Sell, Global Privacy Control, opt-out link, tracker disclosure, cross-border transfer, consent record | marketing-consent-data-collection-review-agent | Consent and data-collection posture | No |
| advertising pixel, Meta Pixel, TikTok pixel, Google Ads tag, LinkedIn Insight Tag, conversion event, dataLayer, PII in URL, email in query parameter, form-field capture, advanced matching, enhanced conversions, pixel on health page, pixel on checkout, PHI leakage, hashed identifier, pixel data leakage | marketing-pixel-data-leakage-review-agent | Advertising-pixel personal-data leakage | No |
| OAuth grant, connected app, API key, integration scope, CRM role, marketing automation permission, martech access, least privilege, over-permissioned connector, stale token, refresh token, shared admin key, token rotation, service account, bulk export permission, access review | martech-access-governance-review-agent | Martech access governance | No |

## Domain taxonomy

| Domain | Keywords and signals |
|---|---|
| `consent` | Consent banner / CMP configuration, tag-manager consent gating, Consent Mode wiring, cookie policy disclosure, opt-out and Global Privacy Control paths, consent records, cross-border transfer mechanisms |
| `pixel-leakage` | Advertising and social pixels, conversion event payloads, `dataLayer` values, URL-parameter PII, form-field auto-capture, pixels on sensitive or authenticated pages, identifier hashing and redaction |
| `access-governance` | OAuth connected apps and scopes, API keys and tokens, CRM and marketing-automation roles, shared and stale credentials, token rotation and expiry, integration ownership, bulk-export permission spread |

## Dispatch examples

### Example 1: Consent-gating question

**User request:** "Do our Google Analytics tags fire before the visitor accepts the cookie banner?"

**Routing:**
```
Route: marketing-consent-data-collection-review-agent
Reason: Task is a pure consent-gating question about tag firing order — single consent domain.
Mode: single
```

`marketing-consent-data-collection-review-agent` reviews the tag-manager container and Consent Mode wiring, determines whether tags hold for the consent signal, and returns severity-labelled findings.

---

### Example 2: Consent banner + pixel leakage

**User request:** "Review our cookie banner for GDPR compliance and check whether our Meta pixel is sending customer emails."

**Routing:**
```
Route: marketing-consent-data-collection-review-agent, marketing-pixel-data-leakage-review-agent
Reason: Task spans consent-banner compliance (CMP design, consent gating) and advertising-pixel PII leakage (identifiers sent to an ad network) — two distinct governance domains.
Mode: parallel (2)
```

`marketing-consent-data-collection-review-agent` audits the banner design and consent records; `marketing-pixel-data-leakage-review-agent` inspects the Meta Pixel payload for raw or auto-captured email addresses.

---

### Example 3: Martech access review

**User request:** "Which of our connected CRM apps have more access than they need?"

**Routing:**
```
Route: martech-access-governance-review-agent
Reason: Task is a least-privilege review of OAuth connected-app scopes — single access-governance domain.
Mode: single
```

`martech-access-governance-review-agent` compares each connected app's granted scope to its function and flags over-broad grants.

---

### Example 4: Full marketing-governance posture review

**User request:** "Review our whole marketing privacy and security posture — consent, pixels, and who has access to the CRM."

**Routing:**
```
Route: marketing-consent-data-collection-review-agent, marketing-pixel-data-leakage-review-agent, martech-access-governance-review-agent
Reason: Task spans consent and data-collection posture, advertising-pixel data leakage, and martech access governance — three distinct governance domains.
Mode: parallel (3)
```

`marketing-consent-data-collection-review-agent` reviews the consent layer; `marketing-pixel-data-leakage-review-agent` reviews pixel payloads for PII leakage; `martech-access-governance-review-agent` reviews OAuth and CRM access for least-privilege violations. Hard ceiling of 4 specialists; this stays under the limit.

---

### Refused request: live mutation

**User request:** "Revoke the SurveyTool OAuth grant in our CRM and publish the updated tag container."

**Routing:**
```
Route: REFUSED
Reason: This request requires live writes — an OAuth revocation and a tag-container publish. No live-guard agents exist in v1. Escalate to a human operator with CRM admin and tag-manager publish rights.
Mode: N/A
```

No agent in this provider executes OAuth revocations, tag-container publishes, consent-banner changes, or key rotations. The human operator must apply the change. `martech-access-governance-review-agent` can produce the scoped-down replacement-grant recommendation, but no agent writes to external systems.

---

## Provenance label protocol

Every value produced by a routed specialist must carry one of these labels:

| Label | Meaning |
|---|---|
| `live-evidence` | Observed in the sanitized configuration or artifact the user provided in this session |
| `documentation-based` | Sourced from official regulation or platform documentation |
| `inference` | Derived by the specialist from inputs using documented methodology |
| `excluded` | Data intentionally excluded from the output, and why |
