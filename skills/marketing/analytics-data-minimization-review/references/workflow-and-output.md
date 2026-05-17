# Workflow and Output Contract

## Workflow

### Step 1 — Collect inputs

Ask the user to provide a sanitized analytics configuration export covering one or more of the following artifacts (replace real user IDs, property IDs, and API keys with placeholders; do not include live event exports or actual user data):

- GA4 property data-retention setting (event data and user data retention periods)
- GA4 custom event definitions: event name, parameters, and the data-layer or gtag call that populates them
- GA4 user-property definitions: property name, scope (user vs. session), and the value being populated
- GA4 custom dimension and metric registrations and their mapped event parameters
- BigQuery export schema: table name, field list with data types, partition strategy, and any scheduled queries or deletion jobs
- IP anonymization setting (GA4 anonymizes by default; confirm the property has not overridden this via Measurement Protocol or server-side tagging)
- Linked product integrations (Google Ads, Search Console, Firebase) that may receive exported user data

If the user provides only a partial set, note which artifacts are absent and scope findings accordingly. Do not attempt to infer schema from event names alone.

This skill is scoped to what analytics platforms collect and retain internally. Outbound pixel payloads to ad networks are out of scope — defer to `marketing-pixel-data-leakage-review`.

### Step 2 — User-scoped custom dimension and user-property audit

Inspect every user-scoped custom dimension and user property for identifiers that link an analytics profile to a real-world person:

```text
# HIGH — user-scoped custom dimension maps GA4 user_pseudo_id to CRM contact ID
user_property: crm_contact_id = "C-00123456"   # value from logged-in session

→ GA4 user_pseudo_id + crm_contact_id = identified natural person.
  GA4 is now a personal-data processor for that contact.
  Requires: documented lawful basis, DPA record of processing, and a valid
  transfer mechanism if the BigQuery project is outside the EEA.

# LOWER RISK — session-scoped experiment variant; no persistent identifier
event_parameter: experiment_variant = "control"   # session-scoped, no CRM link
```

Also flag:
- Persistent advertising identifiers passed as user properties (GCLID, FBCLID stored across sessions).
- Device fingerprint components (user-agent, screen resolution, timezone combined) stored as user properties.
- Email addresses or phone numbers collected in user properties, even in hashed form — still personal data.

### Step 3 — BigQuery export schema audit

For each table in the BigQuery export, assess the combination of fields and retention controls:

```text
# HIGH — raw export retains user_pseudo_id + geo.city + geo.region at full precision
# with no partition expiry and no anonymization transform

Table: events_YYYYMMDD
Fields: user_pseudo_id (STRING), geo.city (STRING), geo.region (STRING),
        event_timestamp (INTEGER), event_name (STRING)
Partition expiry: NONE   # rows never auto-deleted
Scheduled deletion job: NONE

→ user_pseudo_id is a persistent pseudonymous identifier.
  Combined with geo.city + geo.region it can identify a natural person
  in a small geography. GDPR applies. No ceiling on retention = violation
  of storage limitation (Article 5(1)(e)).

# LOWER RISK — export anonymized before landing in BigQuery
Scheduled query: masks user_pseudo_id to k-anonymized cohort bucket
Partition expiry: 90 days aligned to GA4 retention setting
```

Check for:
- user_pseudo_id retention beyond the GA4 property's configured retention period.
- geo fields at city or finer precision without a coarsening transform.
- Absence of partition expiry or scheduled deletion query in the BigQuery dataset.
- Cross-project export to a dataset in a non-EEA GCP region without a valid SCCs or transfer mechanism documented in the DPA record.

### Step 4 — Data-retention period audit

Assess the GA4 property's retention settings against documented justification:

```text
# HIGH — retention set to 14 months (maximum); no documented justification
GA4 retention: User data = 14 months, Event data = 14 months
Justification in DPA record: NONE

→ GDPR Article 5(1)(e) requires retention only as long as necessary for the
  stated purpose. The 14-month maximum is not an entitlement; it requires a
  specific analytical purpose (e.g., year-over-year comparison) that justifies
  the full period.

# COMPLIANT — 2 months; justification documented
GA4 retention: 2 months
DPA record entry: "Session and conversion attribution; 60-day window matches
  last-click attribution window in ad platform; no year-over-year use case."
```

Also verify:
- Whether the BigQuery export enforces the same or shorter retention via partition expiry.
- Whether "Reset user data on new activity" is enabled — if so, the effective retention period may be much longer than the configured window for active users.

### Step 5 — Event-parameter PII audit

Inspect custom event parameters for content that exceeds the analytics collection purpose:

```text
# HIGH — search query parameter captures free-text; may contain PII
event: site_search
parameter: search_term = "{{DL - search_term}}"   # raw dataLayer value

→ Free-text search queries frequently contain full names, email addresses,
  medical terms, or financial account numbers typed by users.
  Collecting raw search terms in GA4 is a data-minimization violation
  unless the value is scrubbed before collection.

# HIGH — URL parameter includes email in query string
event: page_view
parameter: page_location = "https://example.com/reset?email=user@example.com"

→ URL-embedded PII is personal data regardless of intent.
  Strip PII from page_location before it reaches GA4 using a tag-manager
  URL-redaction variable or server-side tagging.

# COMPLIANT — search term replaced with a sanitized flag
event: site_search
parameter: search_performed = true   # no content; confirms intent only
```

### Step 6 — Schema governance audit

Assess whether each custom event, parameter, and user property has documented ownership and purpose:

- Every custom dimension registered in a GA4 property should have: owner (team or role), collection purpose, retention justification, and a review date.
- Absence of governance metadata for any field is MEDIUM — it is a proxy indicator of speculative or abandoned collection that cannot be justified in a DPA record of processing.
- Flag any custom event or user property whose name does not map to a documented analytical use case in the artifact provided.

### Step 7 — Cross-border transfer assessment

If the BigQuery project or linked export destination is outside the EEA, assess the transfer mechanism:

- Standard Contractual Clauses (SCCs) between the controller and Google must be documented.
- The Austrian DSB (2022), French CNIL (2022), and Italian Garante (2022) have each found that Google Analytics transfers to US-based Google infrastructure violate GDPR Chapter V in the absence of adequacy or valid SCCs with sufficient supplementary measures.
- If no transfer mechanism is documented in the DPA record of processing, flag as HIGH.

### Step 8 — Produce the output

Format findings using the Output section below.

---

## Output

Return findings in this structure:

```
## Verdict
<one sentence: pass / needs work / critical issues found>

## Evidence level
<configuration export provided | schema provided | documentation-based | inference from missing element>

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

- This is a static review of sanitized configuration exports and schema definitions. Never request live analytics data, raw event exports containing real user identifiers, GA4 admin credentials, BigQuery service-account keys, or OAuth tokens.
- Findings indicating cross-border transfer violations may require DPA notification or supervisory authority engagement — route remediation and legal assessment to qualified privacy counsel before acting on findings. Do not assess DPA notification obligations yourself.
- This skill is scoped to what analytics platforms collect and retain internally. Outbound pixel payloads transmitted to ad networks are out of scope — refer to `marketing-pixel-data-leakage-review`.
- When evidence is partial, scope each finding to what was provided and state the assumption explicitly.
- A GA4 configuration that is GDPR-compliant for EU users may still create obligations under CCPA/CPRA, LGPD, or other jurisdiction-specific laws — note the applicable framework but limit detailed analysis to GDPR unless the user specifies otherwise.
