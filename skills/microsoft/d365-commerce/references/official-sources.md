# Official sources

Use this reference only when you need source grounding for Dynamics 365 Commerce behavior or omnichannel retail operations.

## Microsoft Learn documentation

Use these as starting points, not as proof of the user's live channel configuration, pricing setup, or Commerce Scale Unit health:

- https://learn.microsoft.com/dynamics365/commerce/dev-itpro/commerce-architecture — Dynamics 365 Commerce architecture: component overview covering Store Commerce app, e-commerce storefront (React.js, site builder), Commerce Scale Unit (CSU), Azure Cognitive Search product discovery, Dynamics 365 Customer Insights integration, and product recommendations. Supports the architecture and CSU review step.
- https://learn.microsoft.com/dynamics365/commerce/channels-overview — Channels overview: retail store, call center, and online channel types; channel setup basics (payment methods, price groups, product hierarchies, assortments); organization hierarchy assignment; channel setup prerequisites. Supports the channel-management review step.
- https://learn.microsoft.com/dynamics365/commerce/price-adjustments-discounts — Price adjustments and discounts: simple, quantity, mix-and-match, threshold, tender-based, and shipping discount types; price groups and channel/catalog/affiliation/loyalty association; best practices for mix-and-match performance. Supports the pricing and discount review step.
- https://learn.microsoft.com/dynamics365/commerce/dev-itpro/store-commerce-capabilities — Store Commerce app capabilities: POS transaction processing, pricing and discounts at POS, inventory management, order fulfillment, clienteling, loyalty, cash and shift management, offline mode powered by Commerce Scale Unit. Supports the POS and store-operations review step.
- https://learn.microsoft.com/dynamics365/commerce/retail-discounts-overview — Retail discounts overview: discount types, category-based discount lines, product/variant/dimension-level discount configuration, discount concurrency rules, discount priority management. Supports the discount-configuration review step.

## Certification alignment (verify currency)

- Exam MB-340: Microsoft Dynamics 365 Commerce Functional Consultant — verify current status and skills measured at learn.microsoft.com/certifications before citing. Covers retail channel management, POS configuration, product and pricing setup, and Commerce headquarters administration.

## Terminology note

The Store Commerce app replaces Modern POS (MPOS) as the primary in-store POS application. Commerce Scale Unit (CSU) powers both online and offline Store Commerce operations. Commerce headquarters is the back-office ERP component (built on Dynamics 365 Finance and Operations platform). Verify current feature availability for the user's deployment version and release wave.

## Grounding rule

Official documentation explains Commerce behavior. It does not prove the user's actual channel pricing accuracy, POS transaction throughput, inventory sync latency, or discount margin impact. Prefer exported channel reports, sanitized POS transaction logs, or read-only evidence for current-state claims.
