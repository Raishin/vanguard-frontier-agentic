# Workflow and Output Contract

## Workflow

### Step 1 — Collect inputs

Ask the user to provide a sanitized UX flow specification covering one or more of the following conversion surfaces (replace real copy with representative placeholders; no real payment data, session tokens, or A/B-test PII):

- Step-by-step page descriptions for the subscription sign-up or free-trial enrollment flow, including CTA labels and button visual weight
- Step-by-step page descriptions for the cancellation path, including step count and any save-offer interstitials
- Upsell interstitial specifications, including whether a "continue without upgrade" option exists and its visual treatment
- Pre-checked option inventory (checkboxes, toggles, radio buttons pre-selected at page load)
- Countdown timer specifications (trigger condition, timer source — server-side session or client-side arbitrary duration, reset behavior)
- Visual hierarchy notes: font size, color contrast, and positioning of accept vs. decline CTAs

If the user provides only a partial set, note which surfaces are absent and scope findings accordingly. Do not attempt to infer full flow structure from a single page description.

This skill does not review consent banners or cookie notices — defer those to `marketing-consent-data-collection-review`.

### Step 2 — Pre-checked consent audit

Inspect every option that is pre-checked or pre-selected at page load and assess what obligation or charge it creates:

```text
# HIGH — auto-renew pre-checked on free-trial enrollment form
[✓] Automatically renew at $29.99/month after trial ends
    (checkbox is below the fold; CTA reads "Start Free Trial")

# COMPLIANT — opt-in explicitly unchecked, above the fold
[ ] Add annual plan upgrade at $9.99/month
```

Specifically flag:
- Any pre-checked option that binds the user to a recurring financial charge without affirmative action — prohibited under the FTC Negative Option Rule and invalidates CPRA consent.
- Pre-checked add-ons, SMS marketing, or data-sharing agreements — these require affirmative consent under CPRA § 1798.140(l) and FTC Act Section 5.
- Whether material terms (price, renewal date, cancellation method) appear clearly and conspicuously before billing information is requested — ROSCA pre-billing disclosure requirement.

### Step 3 — Cancellation path symmetry audit

Count and compare steps:

```text
Enrollment path: Landing → Plan select → Account create → Payment → Confirm  (4 decision steps)
Cancellation path: Account → Settings → Cancel? → Save offer 1 → Save offer 2 → Confirm cancel  (5 decision steps)
```

Flag as HIGH when:
- Cancellation requires more decision steps than enrollment.
- Save-offer interstitials appear without a direct "Cancel anyway" option at each step, forcing the user through the entire save sequence before reaching a cancel confirmation.
- The cancellation entrypoint is buried in account settings more than two levels deep while enrollment is available from the top-level navigation or homepage.

Flag as MEDIUM when:
- Save-offer interstitials appear but each step offers a clear "Cancel anyway" option alongside the save offer.
- Cancellation requires the same step count as enrollment but save offers add latency without hiding the exit.

Note: The FTC Negative Option Rule (effective May 14, 2025) requires simple cancellation through the same mechanism as enrollment, and cancellation must be at least as easy as enrollment.

### Step 4 — Countdown timer authenticity audit

For every countdown timer in the flow, assess whether the deadline is real:

```text
# HIGH — client-side timer resets on page reload; offer is always available
"Offer expires in 09:47" — timer resets to 10:00 on browser refresh
→ Artificial urgency; no real deadline; deceptive act under FTC Act Section 5.

# LOW — server-side session timer; offer genuinely expires at session end
"Your reserved cart expires in 14:53" — server validates expiry at checkout
→ Real deadline; authenticate in server logs; document expiry logic.
```

Distinguish: a timer whose deadline is backed by server state and enforced at checkout is a legitimate scarcity signal. A timer that resets, never expires, or applies to an always-available offer is a fabricated urgency device — HIGH.

### Step 5 — Visual weight and decline-path audit

Assess the visual treatment of accept vs. decline paths:

```text
# HIGH — decline option visually suppressed
[Start Free Trial — large, blue, full-width button]
[no, I don't want savings — 11px grey text, below fold]

# COMPLIANT — balanced visual weight
[Start Free Trial]   [No thanks]   (equal size, both above fold)
```

Flag as HIGH when:
- The decline or "no thanks" option is absent, below the fold, or uses a contrast ratio below 4.5:1 while the accept CTA uses high-contrast primary styling.
- The accept CTA is a full-width button while the decline option is a text link, creating materially asymmetric affordance.

Flag as MEDIUM when:
- Confirm-shaming copy ("No thanks, I prefer to pay more") is used — note it may escalate to HIGH in combination with visual suppression.

### Step 6 — Upsell interstitial consent audit

For each upsell interstitial (a mandatory step between enrollment start and confirmation):

- Confirm a "continue without upgrade" option exists and is reachable without completing the upsell flow.
- Assess whether the interstitial can be bypassed or only dismissed — a mandatory interstitial with no decline path eliminates meaningful consent.
- Confirm the interstitial does not pre-check the upgrade or add charges to the user's cart without affirmative action.

An upsell interstitial with no bypass is HIGH — the user cannot consent to the base product without also being offered (and potentially trapped in) the upsell.

### Step 7 — Material-term pre-billing disclosure audit

Before any billing information is collected, confirm the flow discloses clearly and conspicuously:
- The price and billing frequency after any trial period.
- The exact trial period length and the date on which recurring charges begin.
- How to cancel and through what mechanism.

ROSCA requires these disclosures before collecting billing information. Absence or relegation to fine print is HIGH.

### Step 8 — Produce the output

Format findings using the Output section below.

---

## Output

Return findings in this structure:

```
## Verdict
<one sentence: pass / needs work / critical issues found>

## Evidence level
<flow specification provided | wireframe provided | documentation-based | inference from missing element>

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

- This is a static review of a sanitized artifact. Never request real payment credentials, live user-session recordings, or production A/B-test data containing real user identities.
- Findings indicating violation of the FTC Negative Option Rule carry civil penalty exposure — route enforcement-risk assessment to qualified legal counsel before acting on findings. Do not quantify penalty exposure yourself.
- This skill is scoped to marketing conversion flows: sign-up, upsell, free-trial, and cancellation. Consent banners and cookie notices are out of scope — refer to `marketing-consent-data-collection-review`.
- When evidence is partial, scope each finding to what was provided and state the assumption explicitly.
- A flow that is FTC-compliant under the Negative Option Rule may still violate CPRA or EU AI Act Article 5(1)(b) — assess each regulatory frame independently.
