# Workflow and output contract

Use this reference only when performing the full Dynamics 365 Commerce review or formatting the final answer.

## Review domains

Check these areas before giving a verdict:

- Channel management: retail store, call center, and online channel configuration; organization hierarchy; payment methods; price group assignment; assortment linkage
- Store Commerce POS: Store Commerce app deployment, hardware station, payment connector, offline mode, shift management, receipt profiles, POS permissions
- E-commerce storefront: online channel, site builder configuration, product discovery (Azure Cognitive Search), ratings and reviews, CDX sync to online channel
- Commerce Scale Unit (CSU): deployment health, CDX (Commerce Data Exchange) job schedules, channel database, offline-mode data availability, scale and performance
- Product catalogs and assortments: category hierarchy, assortment configuration, assortment publishing schedule, catalog management, product attribute setup
- Pricing and discounts: price groups linked to channels/catalogs/affiliations/loyalty; price adjustment validity periods; discount types (simple, quantity, mix-and-match, threshold, tender-based, shipping); discount concurrency and priority; coupon codes
- Inventory visibility: cross-channel inventory lookup, available-to-promise configuration, inventory sync latency, order fulfillment visibility at POS
- Store operations: clienteling, endless aisle, order processing and fulfillment workflows, loyalty program setup, gift card configuration
- KPIs: channel revenue consistency, pricing accuracy, POS transaction throughput, CDX sync latency, discount margin impact

## Safe workflow

1. **Frame scope**
   - Area in scope (channel / POS / e-commerce / CSU / pricing / assortments / inventory / store operations):
   - Channels in use (retail store / call center / online / all):
   - Required outcome (channel consistency / pricing accuracy / POS reliability / CSU health / inventory accuracy):
   - Explicit non-goals:

2. **Collect evidence**
   - Prefer exported reports: channel sales by price group, POS transaction logs, CDX job history, discount usage and margin, inventory sync reports.
   - Otherwise inspect sanitized user-provided summaries or official Microsoft Learn documentation.
   - Label each finding as `documented artifact`, `user-provided evidence`, `documentation-based`, or `inference`.

3. **Stress-test risk**
   - Are channels missing price group assignments, causing pricing gaps or incorrect retail prices?
   - Are discount priority and concurrency rules configured to prevent unintended stacking or margin leakage?
   - Is the Commerce Scale Unit healthy, and are CDX jobs completing without errors or data-sync lag?
   - Does the Store Commerce app have a tested offline-mode fallback, and is the channel database current?
   - What evidence is missing that would change the verdict?

4. **Recommend the smallest safe action**
   - Prefer configuration and process fixes over channel or pricing-engine changes.
   - Production channel configuration, pricing setup, and CSU deployment changes require live-guard escalation with a rollback plan.

## Output contract

Return this structure:

```markdown
# D365 Commerce Review: <scope>
## Executive verdict
- Status: HEALTHY / HEALTHY WITH RISKS / AT RISK / NEEDS EVIDENCE
- Biggest risk:
- Evidence level:
## Scope and assumptions
- Confirmed:
- Unknown:
- Out of scope:
## Findings
| Severity | Finding | Evidence | Why it matters | Minimum safe action |
|---|---|---|---|---|
## Recommended actions
1. <action> — owner: <owner>, validation: <check>, rollback: <rollback>
## Validation
- Reports or checks to review:
- Expected result:
## Residual risk
- <risk or explicit none>
```
