# Workflow and Output Contract

## Workflow

### Step 1 — Collect inputs

Ask the user to provide one or more of the following as sanitized exports (replace real values with placeholders; no real visitor data, no ad-platform credentials):
- Tag manager container export showing pixels, triggers, and the variables they read
- Pixel snippets (Meta Pixel, TikTok Pixel, Google Ads / floodlight, LinkedIn Insight Tag, etc.)
- A representative conversion or page-view event payload (the parameters actually sent)
- The `dataLayer` specification or a sample `dataLayer` push
- The list of page types the pixels load on, especially any sensitive-context pages
- Whether advanced/automatic matching is enabled on any pixel

If the user provides only a partial set, note which sections are absent and scope findings accordingly.

### Step 2 — Page-context classification

Classify the pages each pixel loads on before inspecting payloads. Context alone can be the leak:
- **Special-category context**: health/medical, mental health, reproductive, financial hardship, loan/credit, legal, sexual orientation, religious, immigration.
- **Authenticated context**: post-login account, transaction, order-history, or profile pages.
- **General marketing context**: homepage, blog, top-of-funnel landing pages.

Any advertising or social pixel firing in a special-category or authenticated context is HIGH on context alone — the URL, referrer, and page title reveal the person's circumstances regardless of payload contents.

### Step 3 — Payload PII audit

Inspect every parameter the pixel transmits. Flag direct identifiers and quasi-identifiers:

```text
# HIGH — raw email in the page URL, forwarded into the pixel page-view event
https://example.com/welcome?email=jane.doe@example.com
Meta Pixel: track('PageView')   # pixel reads location.href → email leaves the page

# HIGH — PII pushed into dataLayer and mapped to event parameters
dataLayer.push({ event: 'signup', user_email: 'jane.doe@example.com', phone: '+15551234567' });

# CORRECT — only a non-identifying conversion signal
dataLayer.push({ event: 'signup', signup_tier: 'free' });
```

Check for: email, phone, full name, postal address, date of birth, government IDs, account numbers, precise geolocation, and free-text fields that may contain any of the above.

### Step 4 — Form-field auto-capture audit

Identify any feature that captures form input without an explicit field allowlist:
- Meta Pixel **Advanced Matching** (automatic) — scrapes form fields on the page.
- Google **enhanced conversions** with auto-detection rather than a defined selector.
- Tag-manager **form-submit** triggers with a variable capturing all field values.
- Generic input/keystroke listeners.

```text
# HIGH — automatic advanced matching scrapes every form field on the page
fbq('init', 'PIXEL_ID', {}, { autoConfig: true });

# LOWER RISK — matching restricted to explicitly chosen, hashed fields
fbq('init', 'PIXEL_ID');
fbq('track', 'Lead');   # no automatic field scraping; identifiers handled server-side if needed
```

Auto-capture is HIGH because it collects whatever the visitor typed, including fields the marketer never intended to send.

### Step 5 — Identifier-handling audit

For any identifier intentionally sent for matching:
- Confirm SHA-256 hashing where the ad platform's API requires it, and confirm hashing happens before transmission, not by the ad network on receipt.
- Note that hashing is pseudonymization, not anonymization — it still constitutes disclosure of personal data and still requires a lawful basis and consent scope.
- Confirm a redaction or allowlist layer strips PII from URLs and referrers before any pixel reads `location`.

Missing hashing where required is HIGH. Missing a redaction layer is MEDIUM.

### Step 6 — Payload minimization audit

Conversion events should send the minimum signal needed for measurement:
- A conversion flag and a coarse category are usually sufficient.
- Precise revenue, full order line items, account balances, or SKU-level health/financial detail sent to an ad network is MEDIUM — it is more than measurement requires.

### Step 7 — Consent-load ordering check

Note whether pixels load before the consent signal. Flag as MEDIUM and explicitly defer the full consent-gating analysis to the `marketing-consent-data-collection-review` skill — do not duplicate that analysis here.

### Step 8 — Produce the output

Format findings using the Output section below.

---

## Output

Return findings in this structure:

```
## Verdict
<one sentence: pass / needs work / critical issues found>

## Evidence level
<payload provided | container provided | documentation-based | inference>

## Findings

### CRITICAL
- [C1] <finding title>: <description> — <remediation>

### HIGH
- [H1] <finding title>: <description> — <remediation>

### MEDIUM
- [M1] <finding title>: <description> — <remediation>

### LOW
- [L1] <finding title>: <description> — <remediation>

## Safe next actions
1. <action>
2. <action>

## Open questions
- <question requiring user clarification>
```

---

## Security and scope notes

- This is a static review. Never request real visitor data, real conversion logs, or ad-platform credentials. Work from sanitized payloads with placeholder values.
- A leak found here may be a reportable breach under HIPAA, the FTC Health Breach Notification Rule, or state law — flag that possibility and route the determination to qualified counsel and the incident-response process. Do not make the breach-notification call yourself.
- Never recommend a fix that keeps PII flowing to the ad network "but hashed" as a complete remedy — hashing is mitigation, not elimination.
- Never recommend removing a pixel without naming the conversion measurement lost and a server-side or consent-scoped alternative.
- When evidence is partial, scope each finding to what was provided and state the assumption explicitly.
