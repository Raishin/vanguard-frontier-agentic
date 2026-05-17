---
name: marketing-email-list-retention-review
description: Use this skill when reviewing marketing email list segment metadata, consent-record completeness, suppression-list coverage, and documented data-retention schedules for GDPR storage-limitation, CASL record-keeping, and CCPA deletion-right compliance. Trigger when a user provides a CRM or ESP export of list segment metadata fields — consent source, consent timestamp, last-engagement date, subscription status, suppression-list entries — plus the organization's documented email data-retention policy, and asks whether the stored list inventory and retention posture meets regulatory obligations.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-17"
  category: compliance
  lifecycle: experimental
---

# Marketing Email List Retention Review

## Purpose
This skill reviews the stored email list inventory and retention posture of a marketing program against GDPR storage-limitation (Article 5(1)(e)), accountability (Article 5(2)), and erasure (Article 17) obligations; CASL section 6 consent requirements and section 11 three-year record-keeping mandate; and CCPA/CPRA section 1798.105 deletion rights. Marketing email lists accumulate contacts whose consent may have lapsed, whose consent source was never recorded, or who were deleted from the CRM but remain in a detached suppression list — all conditions that expose the controller to regulatory enforcement and litigation. This review assesses the metadata fields of an exported list segment, not the consent banner or collection mechanism (defer that to `marketing-consent-data-collection-review`), and it does not process real subscriber PII.

## Lean operating rules
- Treat contacts with consent timestamps older than 36 months with no documented re-engagement or re-permission event as HIGH — CASL §11 requires demonstrable consent records covering the entire send period, and a gap breaks the chain of proof.
- Treat any active-send segment where a material proportion of contacts (assess whether the proportion is notable relative to the list size) have no consent-source field populated as HIGH — the controller cannot demonstrate lawful basis, violating GDPR Article 5(2) accountability.
- Treat suppression lists stored in a separate system with no documented automated sync cadence as HIGH — contacts deleted or unsubscribed from the primary CRM may re-enter active sends through list imports, segment refreshes, or CRM migrations.
- Treat contacts for whom a deletion request was received but whose record persists beyond the organization's documented deletion SLA as HIGH — a GDPR Article 17 and CCPA §1798.105 violation in progress.
- Treat a retention schedule that sets no maximum age for active-send contacts, or that retains suppressed contacts beyond what is necessary to enforce suppression, as MEDIUM — GDPR Article 5(1)(e) requires data be kept no longer than necessary.
- Treat the absence of a last-engagement date field, or engagement dates older than the stated re-permission interval with no re-permission event recorded, as MEDIUM — these contacts may lack a legitimate-interest or consent basis for continued sends.
- Treat consent-source values that are free-text or inconsistently coded (preventing automated compliance queries) as MEDIUM — the controller must be able to demonstrate lawful basis programmatically at scale.
- Treat the absence of a documented re-permission workflow for lapsing or aged consent as MEDIUM — without a scheduled re-permission program, the list will accumulate non-compliant contacts over time.
- Flag any segment exported for a third-party send partner where the third-party processor agreement or data-sharing basis is absent from the metadata as MEDIUM.
- Label every finding with evidence basis: export provided, policy document provided, documentation-based, or inference from missing fields.
- Do not recommend deleting contacts without first confirming whether suppression-list entries are needed for ongoing suppression enforcement.

## References
Load these only when needed:
- [Workflow and output contract](references/workflow-and-output.md) — use when executing the full review or formatting the final answer.

## Response minimum
Return, at minimum:
- Consent-record completeness findings (consent-source field population, timestamp age, re-permission events)
- CASL record-keeping assessment (three-year demonstrability of consent)
- GDPR storage-limitation and erasure findings (retention schedule, deletion-request SLA)
- CCPA deletion-right posture
- Suppression-list sync and integrity assessment
- Severity-labelled finding list (critical / high / medium / low)
- Safe next actions
