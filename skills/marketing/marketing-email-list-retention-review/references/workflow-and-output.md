# Workflow and Output Contract

## Workflow

### Step 1 — Collect inputs

Ask the user to provide one or more of the following as sanitized exports (replace real subscriber email addresses and IDs with placeholders; no real PII, no live CRM credentials):
- CRM or ESP export of list segment metadata fields, including: consent source, consent timestamp, last-engagement date, subscription status, and suppression-list entry flag
- The organization's documented email data-retention policy (maximum age for active contacts, suppression-list retention period, deletion-request SLA)
- Any documented re-permission workflow or re-engagement schedule
- Suppression-list storage and sync architecture (same system, separate file, sync cadence)
- Third-party send partner list and data-sharing basis documentation

If the user provides only a partial set, note which sections are absent and scope findings accordingly.

### Step 2 — Consent-record completeness audit

For the exported segment, assess the completeness of consent records:
- **Consent-source field**: Is it populated for all active-send contacts? What proportion have a blank or null value? A blank consent-source means the controller cannot demonstrate lawful basis for that contact — a GDPR Article 5(2) accountability failure.
- **Consent-source values**: Are values standardized and machine-queryable (e.g., `website-signup-form-2024`, `trade-show-paper-form-2023`) or free-text and inconsistent? Inconsistent coding prevents automated compliance queries at scale.
- **Consent timestamp**: Is it present for all contacts? Are any timestamps absent or obviously implausible (e.g., epoch zero, future dates)?

```text
# HIGH — material proportion of active-send contacts with blank consent_source
contact_id  | consent_source | consent_timestamp   | status
------------|----------------|---------------------|-------
[ID-001]    | website-signup | 2022-03-14 09:00:00 | active
[ID-002]    | (null)         | (null)              | active   ← no lawful basis
[ID-003]    | (null)         | (null)              | active   ← no lawful basis

# COMPLIANT — all active contacts have a consent source and timestamp
[ID-004]    | trade-show-2024 | 2024-06-01 14:00:00 | active
```

### Step 3 — CASL three-year record-keeping audit

CASL §11 requires that the organization be able to demonstrate consent for every commercial electronic message sent. The consent record must cover the entire period of the relationship:
- Identify contacts whose earliest consent timestamp predates the review date by more than 36 months with no documented re-engagement or re-permission event in the intervening period.
- Identify contacts whose consent basis is "implied" under CASL (e.g., existing business relationship) and assess whether the implied consent window (2 years) has expired.
- A broken record chain — consent recorded, then a gap, then sends resumed without a re-permission event — is a CASL §6 violation for each message sent during the gap.

```text
# HIGH — consent older than 36 months, no re-permission event
contact_id  | consent_timestamp   | last_repermission | months_since_consent
------------|---------------------|-------------------|---------------------
[ID-010]    | 2021-11-05 00:00:00 | (null)            | 42   ← CASL risk

# COMPLIANT — re-permission event within 36-month window
[ID-011]    | 2020-08-01 00:00:00 | 2024-01-15        | 69, re-permissioned
```

### Step 4 — GDPR storage-limitation and erasure audit

GDPR Article 5(1)(e) requires personal data be kept no longer than necessary. Article 17 grants data subjects the right to erasure:
- Review the documented retention policy: does it set a maximum age for active-send contacts? If no maximum age is defined, the list may accumulate contacts indefinitely — a storage-limitation failure.
- Review the deletion-request SLA: does the policy commit to erasing (or suppressing) within 30 days? Are there contacts in the export whose deletion-request date plus the SLA has passed and who remain in an active segment?
- Assess whether suppressed contacts are retained only as long as necessary to enforce ongoing suppression, and no longer.

```text
# HIGH — deletion request received, contact still active past SLA
contact_id  | deletion_requested  | status | days_past_sla
------------|---------------------|--------|---------------
[ID-020]    | 2026-02-01          | active | 45   ← GDPR Art. 17 violation

# MEDIUM — retention policy sets no maximum age
retention_policy.max_active_contact_age = (not defined)
```

### Step 5 — CCPA/CPRA deletion-right posture

California Consumer Privacy Act §1798.105 grants consumers the right to request deletion of their personal information. Assess:
- Whether deletion requests from California residents result in removal from the active-send list within 45 days (or up to 90 days with notice of extension).
- Whether the export shows any California-resident contacts (where identifiable by state field or domain inference) who submitted deletion requests and remain active.
- Whether the suppression list is used to enforce deletion (preventing re-addition on next import) rather than merely removing the contact from one segment.

### Step 6 — Suppression-list integrity audit

The suppression list is the mechanism that enforces both unsubscribes and deletion requests. Weaknesses here cause compliance failures to recur:
- Is the suppression list stored in the same system as the active-send list, or separately? A separately stored file that requires manual sync is HIGH — a missed sync cycle allows deleted or unsubscribed contacts to re-enter active sends.
- What is the documented sync cadence? Real-time or near-real-time sync is the target; periodic batch sync introduces a window of non-compliance.
- Is the suppression list checked against every list import and segment build, or only against scheduled sends? An import that bypasses the suppression check can re-add suppressed contacts silently.

### Step 7 — Third-party send partner assessment

If the segment metadata indicates sends to third-party partners or via third-party ESPs:
- Confirm a data-processing agreement (DPA) or data-sharing agreement is documented for each partner.
- Confirm that the consent scope collected covers the specific send type (e.g., consent to marketing emails from the controller does not automatically extend to sends on behalf of a partner brand).
- Flag absent DPA documentation as MEDIUM.

### Step 8 — Retention schedule and re-permission program assessment

- Review whether the organization's documented policy includes a scheduled re-permission workflow for contacts approaching the consent-age threshold.
- An absence of a re-permission program means the list will accumulate CASL-non-compliant contacts continuously over a 3-year cycle.
- Flag the absence of a re-permission workflow as MEDIUM with a recommendation to implement a 30-month re-engagement trigger.

### Step 9 — Produce the output

Format findings using the Output section below.

---

## Output

Return findings in this structure:

```
## Verdict
<one sentence: pass / needs work / critical issues found>

## Evidence level
<export provided | policy document provided | documentation-based | inference>

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

- This is a static review of list segment metadata and retention policy documents. Never request real subscriber email addresses, real subscriber IDs, live CRM credentials, or live ESP API keys. Work from sanitized exports with placeholder values.
- This skill reviews the stored list inventory and retention posture only. For consent collection mechanisms (banners, opt-in forms, consent strings), defer to `marketing-consent-data-collection-review`.
- A finding of contacts persisting beyond a deletion-request SLA may constitute an ongoing Article 17 or CCPA §1798.105 violation. Surface this and route the determination and remediation to qualified legal counsel and the incident-response process.
- Never recommend deleting suppression-list entries without confirming that the entries are not needed to enforce ongoing suppression — erasing suppression records can cause previously unsubscribed contacts to be re-added.
- When evidence is partial (e.g., policy document provided but no export), scope each finding to the available evidence and state assumptions explicitly.
- CASL record-keeping obligations extend to every commercial electronic message sent; a finding of a broken consent chain covers all messages sent during the gap, not just future sends.
