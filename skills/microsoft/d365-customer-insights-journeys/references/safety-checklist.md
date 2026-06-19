# Safety checklist

Use this reference before any recommendation involving production journey publish authorization, consent-model changes, bulk outreach approval, or compliance-impacting configuration changes in Dynamics 365 Customer Insights — Data or Customer Insights — Journeys.

## Non-negotiables

- Never ask users to paste credentials, API keys, tenant IDs, environment URLs, client secrets, customer PII, or consent data exports into chat.
- Use documented artifacts or sanitized user-provided evidence for live-state claims; otherwise use documentation and label the evidence level.
- Do not invent unified profile counts, segment membership sizes, consent record states, journey test results, or compliance profile configurations.
- Require explicit human approval before recommending production journey publish, consent-model changes, or bulk outreach execution.
- Use current official Microsoft Learn documentation for Dynamics 365 Customer Insights — Data and Customer Insights — Journeys behavior.
- Keep recommendations least-change, reversible, and scoped to the workload in question.
- Production journey publish, consent-model changes, and segment-based bulk outreach are live-guard gated. Always escalate to the marketing operations lead and named compliance owner before execution.

## Stress checks

- Have all data sources been ingested and mapped before unification runs were executed?
- Has the unified profile configuration been validated with sample profile results showing correct identity resolution?
- Do segment rules reference validated profile attributes with a refresh schedule aligned to journey entry cadence?
- Has at least one compliance profile been configured with purpose and topic hierarchy before any journey is published?
- Is double opt-in configured on all compliance profiles where required by applicable regulation (GDPR, CAN-SPAM, CASL)?
- Has legacy consent data been fully migrated from outbound marketing or external systems before bulk outreach begins?
- Have all journey branches — including suppression, exit, error, and no-match paths — been tested in a non-production environment?
- Has the compliance owner signed off on the consent model and unsubscribe path?
- Is there a post-launch monitoring plan for engagement metrics, consent opt-out rates, and journey error rates?

## Evidence labels

Use `live evidence`, `documented artifact`, `user-provided evidence`, `documentation-based`, or `inference`. Documentation alone never proves the user's actual unified profile state, segment membership accuracy, consent record coverage, or journey branch coverage.

## Live-guard gate

The following actions require explicit human confirmation and are out of scope for automated execution:

- Publishing a journey to Live status in the production environment
- Modifying compliance profiles, purposes, or topics in the production consent model
- Executing segment-based bulk email, SMS, or push notification outreach in production
- Migrating or bulk-loading consent records into the production consent center
- Enabling or disabling double opt-in on a compliance profile in production
- Authorizing suppression list changes that affect production outreach
- Signing off on regulatory compliance posture on behalf of the compliance owner
