# Workflow and Output Contract

## Workflow

### Step 1 — Collect inputs

Ask the user to provide the following as sanitized exports (replace real account IDs, pixel IDs, and domain values with placeholders; no real visitor consent records, no ad-platform credentials):
- Tag-manager container export (Google Tag Manager JSON, Tealium profile export, Segment source config, or equivalent) showing all tags, triggers, and variables
- CMP opt-out configuration showing how the GPC signal is read, which consent category it maps to, and which variable or data layer key is published on detection
- Server-side tag container export or forwarding configuration, if a server-side GTM or equivalent is in use
- The list of ad tags and conversion API forwarding rules currently active in the container
- Whether a GPC-state variable or consent-state variable exists in the variable layer and which firing rules reference it

If the user provides only the client-side container without the CMP config, note that the pre-first-visit suppression and CMP-propagation findings are inference only.

### Step 2 — GPC variable propagation assessment

Determine whether the CMP publishes the GPC state into a form the tag manager can consume:
- Does the CMP set a consent cookie, a `dataLayer` push, or a JavaScript variable on GPC detection?
- Is that value mapped to a named variable in the tag-manager variable layer?
- Does the variable resolve correctly on the first page load before any cookie is written (fresh-session case)?

```text
# CORRECT — CMP publishes GPC state to dataLayer; GTM variable reads it
dataLayer.push({ event: 'consent_update', gpc_opt_out: true });
GTM variable: "GPC Opt-Out State" → reads dataLayer key gpc_opt_out

# BROKEN — CMP sets opt-out cookie only; GTM has no variable reading that cookie
document.cookie = "opt_out=1";
GTM variable layer: no cookie variable defined for opt_out
→ all existing firing rules are unaffected
```

A CMP that acknowledges GPC but does not propagate the state to the tag-manager variable layer is HIGH — the acknowledgment is cosmetic.

### Step 3 — Firing-rule guard audit

For every ad conversion tag and social pixel in the container, inspect the firing rules:
- Does every rule that fires the tag have a GPC-state condition that suppresses firing when the opt-out is active?
- Are exception triggers used to block firing, or is the GPC condition embedded in the trigger itself?
- Are any tags set to fire on "All Pages" or unconditional triggers without a GPC exception?

```text
# HIGH — conversion tag fires on all form submissions with no GPC guard
Trigger: "Form Submit - Lead"
  Conditions: Form ID equals "contact-form"
  [no GPC-state condition]

# CORRECT — same trigger with a GPC-state exception
Trigger: "Form Submit - Lead - GPC Allowed"
  Conditions: Form ID equals "contact-form"
              GPC Opt-Out State does not equal "true"
```

Each ad conversion tag lacking a GPC-state condition in its firing rule is a separate HIGH finding.

### Step 4 — Server-side forwarding path audit

Inspect any server-side tag container or first-party endpoint forwarding to conversion APIs:
- Meta Conversions API (CAPI) via server-side GTM or custom endpoint
- Google Enhanced Conversions via server-side container
- TikTok Events API, LinkedIn CAPI, Pinterest API, or equivalent

For each server-side forwarding path:
- Is the GPC state (or a consent signal derived from it) passed to the server-side container as a request parameter or header?
- Does the server-side tag have a condition that suppresses forwarding when GPC opt-out is active?
- Is the server-side endpoint documented as a bypass of the client-side CMP?

```text
# HIGH — first-party CAPI endpoint forwards all purchase events; no GPC check at endpoint
POST /api/conversions
Body: { event: "Purchase", user_data: { em: "<hashed_email>" } }
Server-side GTM tag: "Meta CAPI - Purchase"
  Trigger: All custom events named "purchase"
  [no consent-state variable in server-side container]
```

### Step 5 — Pre-first-visit suppression audit

Assess whether a user who has GPC active in their browser before their very first visit to the site receives suppression on that initial page load:
- On a completely fresh session (no prior cookie, no localStorage), does the CMP read the GPC header and suppress tags before any tag fires?
- Or does the CMP first write a consent cookie on opt-out and only then suppress — meaning the first page load fires tags before suppression activates?
- Does the tag-manager firing sequence (consent initialization order) ensure GPC is resolved before any ad tag trigger evaluates?

Pre-first-visit non-suppression is HIGH under CPPA guidance: the GPC signal must be honored from the moment of receipt, not only after a cookie is established.

### Step 6 — Opt Me Out Act (AB 566) consistency check

AB 566 (effective Oct 2025) requires that the opt-out link mechanism and the GPC signal produce identical downstream suppression:
- If the site honors an opt-out link click but the GPC path is broken (Steps 2–5), the link mechanism and the automated signal produce inconsistent results — flag as MEDIUM.
- If neither path is technically honored, elevate the AB 566 finding to accompany the CPRA HIGH findings.

### Step 7 — Logging and attestation gap check

Compliance attestation requires evidence:
- Does the server-side forwarding log the GPC state at the time each event is suppressed or forwarded?
- Is there a documented test procedure (e.g., browser extension sending GPC header, reviewing network tab or server log) confirming suppression?
- Flag the absence of both as MEDIUM.

### Step 8 — Produce the output

Format findings using the Output section below.

---

## Output format

```
## Verdict
<one sentence: pass / needs work / critical issues found>

## Evidence level
<container provided | CMP config provided | documentation-based | inference>

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

- This is a static review. Never request real visitor consent records, live CMP logs, ad-platform credentials, or server-side endpoint access. Work from sanitized container exports with placeholder account IDs.
- A confirmed failure to honor GPC may constitute a violation of CCPA/CPRA §1798.135 and may be subject to enforcement by the California Privacy Protection Agency. Do not make the violation determination yourself — flag the issue and route the legal assessment to qualified privacy counsel.
- The CPPA September 2025 enforcement sweep confirmed that cosmetic CMP acknowledgment without downstream tag suppression is treated as non-compliance. Surface this distinction explicitly in findings.
- When evidence is partial (e.g., container provided but no CMP config), scope each finding to what was provided and state the inference basis explicitly.
- Do not recommend disabling all ad tags as the remediation. Identify the specific firing-rule conditions missing a GPC guard and propose the minimal surgical fix per tag.
