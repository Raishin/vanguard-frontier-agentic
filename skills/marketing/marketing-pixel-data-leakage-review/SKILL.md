---
name: marketing-pixel-data-leakage-review
description: Use this skill when reviewing advertising pixels and event-tracking for personal-data leakage to third-party ad networks. Trigger when a user provides a tag-manager container, a Meta/TikTok/Google/LinkedIn pixel snippet, a conversion-event payload, a dataLayer specification, or asks whether their tracking pixels leak email, phone numbers, health, or financial data to ad platforms, or whether pixels on sensitive pages create a breach or HIPAA exposure.
allowed-tools: Read Grep Glob
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-05-17"
  category: security
  lifecycle: experimental
---

# Marketing Pixel Data-Leakage Review

## Purpose
This skill reviews advertising pixels and conversion event tracking for unintended exfiltration of personal data to third-party ad networks. Marketing pixels are an attacker-irrelevant but regulator-relevant data path: a pixel that captures an email in a URL parameter, auto-collects form fields, or sits on a health or financial page silently transmits identifiable data to Meta, TikTok, Google, or LinkedIn with no contract, no consent scope, and no breach visibility. This pattern has produced large HIPAA settlements, FTC Health Breach Notification Rule actions, and wiretap class actions. The review catches PII in event payloads, form-field auto-capture, pixels on sensitive-context pages, unhashed identifier transmission, and missing data-redaction controls before they ship.

## Lean operating rules
- Treat raw email address, phone number, full name, or government identifier sent to an ad network in a URL query parameter, event parameter, or `dataLayer` value as HIGH — this is uncontracted disclosure of personal data to a third party.
- Treat tag-manager or pixel features that auto-collect form field values (advanced/automatic matching, form-input listeners, generic "form submit" variables capturing field contents) as HIGH — they capture whatever the user typed, including sensitive fields.
- Treat any advertising or social pixel present on a health, medical, financial, legal, or other special-category page (symptom checkers, patient portals, loan applications, insurance quotes) as HIGH — page context alone reveals special-category data.
- Treat advertising pixels on authenticated/post-login pages that carry account or transaction context as HIGH — the URL and page state themselves leak personal circumstances.
- Treat identifiers sent to ad networks without SHA-256 hashing where the platform's API requires hashing as HIGH — and note that hashing reduces but does not eliminate the disclosure.
- Treat URL query strings containing PII forwarded verbatim into pixel page-view events as HIGH — strip or redact before the pixel reads `location`.
- Flag conversion values transmitting precise revenue, order contents, or account balances when only a conversion flag is needed as MEDIUM.
- Flag pixels loaded before the consent signal as MEDIUM here and defer the full consent-gating analysis to `marketing-consent-data-collection-review`.
- Flag the absence of a documented redaction or allowlist layer between the page and ad pixels as MEDIUM.
- Do not recommend removing a pixel without naming the conversion measurement it supports and the attribution loss.
- Label every finding with evidence basis: payload provided, container provided, documentation-based, or inference from missing config.

## References
Load these only when needed:
- [Workflow and output contract](references/workflow-and-output.md) — use when executing the full review or formatting the final answer.

## Response minimum
Return, at minimum:
- PII-in-payload findings (URL params, event params, dataLayer values)
- Form-field auto-capture assessment
- Sensitive-context page assessment (health, financial, legal, authenticated)
- Identifier handling assessment (hashing, redaction, allowlist)
- Conversion-payload minimization findings
- Severity-labelled finding list (critical / high / medium / low)
- Safe next actions
