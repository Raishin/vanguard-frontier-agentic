# Workflow and Output Contract

## Workflow

### Step 1 — Collect inputs

Ask the user to provide one or more of the following as sanitized exports or descriptions (no real visitor identifiers, no analytics account credentials, no consent-string archives):
- Consent Management Platform (CMP) configuration — vendor, banner layout, button set, default consent state, per-purpose toggles
- Tag manager container export (e.g. GTM container JSON) showing tags, triggers, and consent settings
- Google Consent Mode / consent initialization snippet
- Cookie / privacy policy text, or the disclosed cookie and vendor table
- Target jurisdictions and the regimes that apply (EEA/UK, California, other US states, Brazil, etc.)

If the user provides only a partial set, note which sections are absent and scope findings accordingly.

### Step 2 — Jurisdiction and regime scoping

Establish which legal model applies before assessing tags:
- **Opt-in regimes** (GDPR + ePrivacy, UK GDPR/PECR): non-essential storage and access require prior consent. Default state must be denied.
- **Opt-out regimes** (CCPA/CPRA and most US state laws): collection may proceed, but a "Do Not Sell or Share" path and Global Privacy Control honoring are required.
- A global site usually serves both; the CMP must geo-resolve the correct model per visitor.

Flag a single consent model applied globally when traffic spans both regimes as MEDIUM.

### Step 3 — Consent-gating audit

For every analytics and advertising tag, determine whether it fires before or after the consent signal.

Check for:
- Tags with a firing trigger of "page view" / "DOM ready" and no consent condition (HIGH in opt-in regimes)
- Tag manager "additional consent checks" left unconfigured
- A hardcoded analytics or pixel snippet in page source, bypassing the tag manager and the CMP entirely (HIGH)
- Server-side tagging that forwards events with no consent state propagated

```text
# RISKY — tag fires on every page view, no consent gate
Tag: GA4 Configuration
Trigger: All Pages
Consent settings: No additional consent required

# CORRECT — tag waits for the analytics_storage grant
Tag: GA4 Configuration
Trigger: All Pages
Consent settings: Require additional consent for: analytics_storage
```

### Step 4 — Banner design audit

Assess the banner against recognized dark-pattern guidance:
- **Symmetry**: accept and reject must be equally prominent and equally reachable. A prominent "Accept All" with reject buried in a secondary "Manage" screen is HIGH.
- **Pre-selection**: any consent toggle pre-set to ON, or pre-ticked checkbox, is HIGH.
- **Implied consent**: "by continuing to browse you agree" or scroll-to-consent is HIGH.
- **Granularity**: distinct purposes (analytics, advertising, personalization) must be independently refusable. A single on/off is MEDIUM.
- **Nagging / re-prompting**: re-showing the banner to pressure a reluctant visitor is MEDIUM.
- **Withdrawal**: withdrawing consent must be as easy as giving it — a persistent preferences link must exist.

### Step 5 — Consent Mode and signal-propagation audit

If Google Consent Mode (or an equivalent) is used:
- Default consent state must be `denied` for `ad_storage`, `analytics_storage`, `ad_user_data`, `ad_personalization` in opt-in regimes.
- `wait_for_update` must be set so tags hold until the CMP resolves the choice.
- Verify the CMP actually calls `gtag('consent', 'update', ...)` on the visitor's decision.

```text
# RISKY — default granted, no wait
gtag('consent', 'default', { ad_storage: 'granted', analytics_storage: 'granted' });

# CORRECT — default denied, wait for the CMP update
gtag('consent', 'default', {
  ad_storage: 'denied', analytics_storage: 'denied',
  ad_user_data: 'denied', ad_personalization: 'denied',
  wait_for_update: 500
});
```

### Step 6 — Tracker-to-policy disclosure audit

Cross-check every tracker observed in the container against the cookie policy and CMP vendor list:
- Each cookie and pixel must be named, categorized by purpose, and given a stated retention.
- Vendors receiving data must appear in the disclosed vendor list.
- A tracker present in the container but absent from disclosure is HIGH — undisclosed processing has no lawful basis and breaches the transparency obligation.

### Step 7 — Opt-out and cross-border audit

- Confirm a "Do Not Sell or Share My Personal Information" link (or a Limit-Use link for sensitive data) where opt-out regimes apply.
- Confirm the CMP honors the Global Privacy Control browser signal.
- For advertising tags transmitting to ad networks outside the visitor's region, confirm a referenced transfer mechanism exists in the policy (Standard Contractual Clauses, an adequacy decision, or the relevant framework).

### Step 8 — Consent-record audit

Confirm the CMP retains, per consent event: a timestamp, the scope/purposes accepted, the consent-string version, and a withdrawal record. Without this the controller cannot demonstrate compliance on request. Missing records is MEDIUM.

### Step 9 — Produce the output

Format findings using the Output section below.

---

## Output

Return findings in this structure:

```
## Verdict
<one sentence: pass / needs work / critical issues found>

## Evidence level
<configuration provided | policy text provided | documentation-based | inference>

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

- This is a static review. Never request real visitor data, raw consent-string archives, analytics account credentials, or tag-manager publish access.
- Do not provide definitive legal conclusions; surface regulatory risk and route binding determinations to qualified privacy counsel.
- Never recommend removing a consent gate to recover attribution data.
- When evidence is partial, scope each finding to what was provided and state the assumption explicitly.
