# Safety checklist

Use this reference before any recommendation involving production channel configuration, pricing-engine setup, Commerce Scale Unit deployment, or POS register settings in Dynamics 365 Commerce.

## Non-negotiables

- Never ask users to paste credentials, tenant IDs, environment URLs, connection strings, certificates, or customer transaction data into chat.
- Use exported channel sales reports or sanitized user-provided evidence for current-state claims; otherwise use documentation and label the evidence level.
- Do not invent pricing attainment figures, POS transaction volumes, CDX sync latency metrics, or discount margin impacts.
- Require explicit human approval before recommending any production channel, pricing, assortment, or Commerce Scale Unit change.
- Use current official Microsoft Learn documentation for Commerce channel setup, pricing engine behavior, Commerce Scale Unit architecture, and Store Commerce POS capabilities.
- Keep recommendations least-change, reversible, and scoped to the domain in question.

## Stress checks

- Are all active channels assigned to the correct price groups, or are there channels operating without consistent pricing?
- Are discount priority and concurrency rules configured to prevent unintended stacking and margin leakage?
- Is the Commerce Scale Unit healthy, and are CDX (Commerce Data Exchange) jobs completing on schedule without data-sync errors?
- Does the Store Commerce app have a validated offline-mode configuration, and has it been tested for the expected failover scenario?
- What rollback exists if a channel pricing or Commerce Scale Unit change causes pricing errors or POS outages in a live retail environment?

## Evidence labels

Use `documented artifact`, `user-provided evidence`, `documentation-based`, or `inference`. Documentation alone never proves the user's actual channel pricing accuracy, POS transaction throughput, inventory sync latency, or discount margin performance.

## Live-guard gate

The following actions require explicit human confirmation and are out of scope for automated execution:

- Modifying production channel configuration, organization hierarchy assignments, or payment method setup
- Changing production price groups, price adjustments, or discount configurations that affect live channels or online storefronts
- Publishing or withdrawing production product assortments or catalogs
- Deploying, redeploying, or scaling a Commerce Scale Unit in a production environment
- Running or re-running CDX (Commerce Data Exchange) full-sync jobs in production that overwrite live channel databases
- Modifying production POS register settings, hardware station profiles, or payment connector configurations
