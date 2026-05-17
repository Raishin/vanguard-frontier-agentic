# Workflow and Output Contract

## Workflow

### Step 1 — Collect inputs

Ask the user to provide one or more of the following as sanitized documents (replace real records with schema descriptions; no actual customer PII, no platform API credentials):
- Audience upload field-mapping specification (CSV column headers, platform upload template, or field list with data types)
- Declared hashing method (algorithm, whether hashing occurs client-side or server-side, and normalization steps applied before hashing)
- Consent-basis documentation (privacy notice excerpt, consent-collection mechanism, opt-in/opt-out flow, original collection purpose)
- Originating list segment metadata (how the list was segmented, which customer population it covers, what action or consent triggered inclusion)
- Target platform(s) (Meta, Google, LinkedIn, TikTok, or DSP)
- Whether EU or California resident data subjects are included

If the user provides only a partial set, note which sections are absent and scope findings accordingly.

### Step 2 — Platform identification and terms baseline

Identify the target platform and retrieve the relevant customer-list terms:

```text
Meta Custom Audiences: Prohibit sensitive categories (health, financial account data, sexual orientation,
  religious beliefs, political views, union membership, biometric data, criminal records, or data from
  users under 13). Require SHA-256 hashing of email, phone, name. Normalize before hashing
  (lowercase, no spaces).

Google Customer Match: Prohibit sensitive-category data. Require SHA-256 hashing. Normalization
  required per Google's specification (lowercase email, E.164 phone format before hashing).

LinkedIn Matched Audiences: Prohibit sensitive categories. Require SHA-256. Minimum list size
  enforced for privacy (300 matched members).

TikTok Custom Audiences: Require SHA-256. Prohibit sensitive categories. GDPR and CCPA
  compliance certifications required for respective geographies.
```

Note any field in the specification that appears to violate platform-specific prohibitions.

### Step 3 — Hashing adequacy audit

Inspect the declared hashing method against minimum requirements:

```text
# HIGH — MD5 hashing declared
MD5 produces a 128-bit digest. For common email formats (first.last@domain.com),
precomputed rainbow tables resolve ~80-90% of hashes. This is not adequate pseudonymization
under GDPR Article 5(1)(f) and violates platform terms for Meta, Google, LinkedIn, and TikTok.
Remediation: Replace MD5 with SHA-256. Apply normalization (lowercase, trim whitespace) before hashing.

# HIGH — plain-text upload (no hashing declared)
Direct transmission of email or phone in the clear constitutes unambiguous PII disclosure
to the ad platform. No platform terms permit this.

# MEDIUM — SHA-256 declared but normalization step not documented
Without documented normalization (lowercase, strip punctuation), match rates degrade and
partial hash collisions become possible. Confirm normalization spec.

# CORRECT — SHA-256 with documented normalization
Email: lowercase → strip whitespace → SHA-256
Phone: E.164 format → strip non-numeric → SHA-256
```

Hashing reduces re-identification risk but does not eliminate it — flag this explicitly. Hashed identifiers are still personal data under GDPR.

### Step 4 — PII field-scope and data-minimization audit

Inspect the field mapping for over-inclusion relative to the matching objective:

```text
# Minimum field set for match-rate adequacy (any platform)
- SHA-256 hashed email  ← sufficient for >85% match rates on most platforms

# Extended field set (justified only when match rate is demonstrably inadequate)
- SHA-256 hashed phone number
- SHA-256 hashed first name + last name (separate fields per platform spec)

# Over-included fields (data-minimization violation)
- Date of birth → not needed for matching; increases re-identification
- Home postal code → combined with email + phone = high-confidence re-identification surface
- Transaction history columns → no matching function; pure data exposure
- IP address → not a valid matching identifier; exposes behavioral fingerprint
```

Flag any field beyond the minimum set needed for the stated matching objective as MEDIUM. Flag postal code combined with email and phone as HIGH (re-identification surface).

### Step 5 — Consent-basis validity audit

Map the originating consent basis against the intended use:

```text
Scenario A — Transactional consent only
Original consent: "I agree to receive order confirmations and shipping updates."
Intended use: Seed list for Facebook lookalike audience targeting financial product ads.
Assessment: HIGH — sharing for advertising targeting exceeds the transactional consent scope.
GDPR: Purpose-limitation violation (Article 5(1)(b)). Separate consent for advertising use required.
CPRA: Unauthorized "sharing" of personal information for cross-context behavioral advertising
      (§1798.100) — constitutes a sale/share requiring opt-out mechanism.

Scenario B — Marketing consent with opt-in
Original consent: "I agree to receive marketing communications from [Brand]."
Intended use: Custom audience upload for retargeting on Meta.
Assessment: MEDIUM — first-party retargeting may fall within scope, but sharing PII with Meta
            as a data controller may require separate disclosure in the privacy notice.
            Confirm whether privacy notice discloses ad-platform data sharing.

Scenario C — No documented consent, legitimate interest asserted
Assessment: HIGH — legitimate interest is a narrow basis that rarely survives for ad-platform
            data sharing. LIA (Legitimate Interest Assessment) must be documented; data-subject
            rights (opt-out, erasure) must be honored.
```

### Step 6 — Cross-border transfer assessment

If EU resident data subjects are in the list and the ad platform is a non-adequate-country processor:

```text
# Required safeguards for EU → US transfer (post-Schrems II)
- Standard Contractual Clauses (SCCs) — Module 2 (controller to processor) or
  Module 1 (controller to controller depending on platform's legal role)
- UK Addendum if UK residents included
- Transfer Impact Assessment (TIA) documented

# HIGH — EU residents in list, no SCC or EU-US DPF certification documented for the platform
GDPR Chapter V prohibits transfer without adequate safeguard. Confirm platform's DPF
certification status or execute SCCs before upload.
```

### Step 7 — Platform-specific sensitive-category restriction check

Cross-check each field against platform-specific prohibited categories:

| Field | Meta | Google | LinkedIn | TikTok |
|---|---|---|---|---|
| Health condition inferred from segment | PROHIBITED | PROHIBITED | PROHIBITED | PROHIBITED |
| Financial hardship segment label | PROHIBITED | PROHIBITED | Review | Review |
| Religious affiliation in segment metadata | PROHIBITED | PROHIBITED | PROHIBITED | PROHIBITED |
| Age (exact DOB) | Allowed (hashed) | Allowed (hashed) | Caution | Caution |
| Postal code (unhashed) | Not a match field | Not a match field | Not a match field | Not a match field |

Flag any field or segment label that maps to a platform-prohibited category.

### Step 8 — Retention and deletion assessment

Flag the absence of documented platform-side retention limits as MEDIUM:

- Confirm the platform's stated retention period for unmatched records (typically 48-72 hours for most platforms).
- Confirm whether the operator has a deletion schedule for the source list post-upload.
- Confirm whether the list can be deleted from the platform after campaign completion.

### Step 9 — Produce the output

Format findings using the Output section below.

---

## Output

Return findings in this structure:

```
## Verdict
<one sentence: pass / needs work / critical issues found>

## Evidence level
<field-mapping spec provided | hashing method declared | consent documentation provided | inference>

## Platform(s) in scope
<Meta | Google | LinkedIn | TikTok | DSP>

## Findings

### CRITICAL
- [C1] <finding title>: <description> — <remediation>

### HIGH
- [H1] <finding title>: <description> — <remediation>

### MEDIUM
- [M1] <finding title>: <description> — <remediation>

### LOW
- [L1] <finding title>: <description> — <remediation>

## Recommended minimum field set
<field list with hashing spec>

## Safe next actions
1. <action>
2. <action>

## Open questions
- <question requiring user clarification>
```

---

## Security and scope notes

- This is a static review. Never request actual audience files, real customer records, or platform API credentials. Work from sanitized field-mapping specifications, declared hashing methods, and consent-basis documentation.
- SHA-256 hashing of a common email address is pseudonymization, not anonymization — the hashed identifier is still personal data under GDPR and still requires a lawful basis for sharing with the ad platform.
- A consent-scope mismatch discovered here may constitute a reportable breach or an unauthorized "sale/share" of personal information under CPRA — flag that possibility and route the legal determination to qualified counsel and the privacy compliance team.
- Never recommend uploading a field that is not strictly needed for the matching objective. Default to the minimum field set.
- When evidence is partial, scope each finding to what was provided and state the assumption explicitly.
