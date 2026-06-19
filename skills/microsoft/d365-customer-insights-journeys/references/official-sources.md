# Official sources

Use this reference only when you need source grounding for Dynamics 365 Customer Insights — Data or Customer Insights — Journeys behavior, data unification, segment design, consent compliance, or journey orchestration guidance.

## Microsoft Learn documentation

Use these as starting points, not as proof of the user's live environment configuration or consent posture:

- https://learn.microsoft.com/dynamics365/customer-insights/data/data-unification
- https://learn.microsoft.com/dynamics365/customer-insights/data/segments
- https://learn.microsoft.com/dynamics365/customer-insights/data/measures
- https://learn.microsoft.com/dynamics365/customer-insights/data/get-started
- https://learn.microsoft.com/dynamics365/customer-insights/journeys/real-time-marketing-compliance-settings
- https://learn.microsoft.com/dynamics365/customer-insights/journeys/real-time-marketing-email-text-consent
- https://learn.microsoft.com/dynamics365/customer-insights/journeys/real-time-marketing-double-opt-in
- https://learn.microsoft.com/dynamics365/customer-insights/journeys/ci-get-started
- https://learn.microsoft.com/dynamics365/customer-insights/journeys/real-time-marketing-migrate-consent
- https://learn.microsoft.com/dynamics365/customer-insights/journeys/unified-profile-segment-creation

## Grounding rule

Official documentation explains Dynamics 365 Customer Insights behavior, data unification mechanics, segment evaluation rules, and consent enforcement logic. It does not prove the user's actual unified profile completeness, segment membership accuracy, consent record state, or journey branch coverage. Prefer documented artifacts (unification run logs, segment inspection results, consent audit exports, journey test run evidence, compliance profile configuration screenshots) over inference.

## Service facts (verified 2026-06-17)

Customer Insights — Data (CDP) structure:
- Customer Insights — Data is a customer data platform (CDP) that ingests data from multiple sources, runs identity resolution through deduplication and match rules, and produces unified customer profiles stored in Dataverse.
- **Data sources**: ingested via Power Query connectors, Azure Data Lake Storage, Dataverse, or other supported connectors. Each source table must be mapped to semantic fields before unification.
- **Unification phases**: source field mapping → deduplication → match rules → merge policies. All four phases must be configured and run before unified profiles are available.
- **Segments**: evaluated against unified customer profiles. Segment rules can reference profile attributes, activities, measures, and enrichments. Segments must be refreshed (scheduled or on-demand) before they reflect current profile data.
- **Measures**: calculated KPIs based on unified profile data. Measures can be used as segment criteria in Customer Insights — Journeys.

Consent model (Customer Insights — Journeys):
- Consent is captured and enforced at the **contact point level** (email address, phone number, or custom channel address), not at the contact/lead record level.
- The consent hierarchy: compliance profile → purpose → topic. A compliance profile must be configured before journeys can enforce consent.
- **Double opt-in**: must be explicitly enabled per compliance profile. Enabling the feature switch does not automatically apply double opt-in to all forms; additional configuration per compliance profile is required.
- The `DoNotEmail`, `DoNotBulkEmail`, and `DoNotTrack` fields on the contact record are **not** evaluated by default in real-time journeys. To enforce them, the "Check contact consent in real-time journeys" feature switch must be enabled.
- Consent migration from outbound marketing or external systems requires manual import or the Load Consent function; contact point consent records are not automatically created from legacy field values.

Journey orchestration:
- Real-time journeys support segment-based entry (scheduled) and trigger-based entry (event-driven).
- Journey branches, wait conditions, and channel steps must be tested before production publish. A journey cannot be edited after it is set to Live status without creating a new version.
- Production journey publish is irreversible without stopping the journey; stopping a live journey affects all in-progress participants.

Certification anchor:
- MB-260 (Customer Insights Data Specialist) was retired 2026-06-17 per latest available information; MB-220 (Marketing) was also retired. Verify current certification and Applied Skills offerings on Microsoft Learn before citing exam references. (E4: verify before citing.)
