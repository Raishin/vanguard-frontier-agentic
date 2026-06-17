# Safety checklist

Use this reference before any recommendation involving production master plan runs, coverage group reconfigurations, planned order firming, BOM or route activations, warehouse management parameter changes, or compliance-impacting supply chain configuration changes in Dynamics 365 Supply Chain Management.

## Non-negotiables

- Never ask users to paste credentials, tenant IDs, environment URLs, client secrets, certificates, or customer personally identifiable supply chain data into chat.
- Use exported planning logs, on-hand inventory reports, or sanitized user-provided evidence for live-state claims; otherwise use documentation and label the evidence level.
- Do not invent on-hand quantities, coverage settings, BOM version numbers, planned order quantities, lead times, or live environment state.
- Require explicit human approval before recommending any production master plan parameter change, coverage group reconfiguration, planned order mass firming, or BOM or route activation.
- Use current official Microsoft Learn documentation for Dynamics 365 Supply Chain Management master planning and production control behavior.
- Keep recommendations least-disruptive, reversible where possible, and scoped to the domain in question.
- Production master plan runs, coverage group changes, and BOM or route activations are live-guard gated. Always escalate to a qualified Dynamics 365 Supply Chain Management controller or system administrator with environment access before execution.

## Stress checks

- What items have missing or incorrect coverage settings that could create stockout or excess inventory upon the next planning run?
- What on-hand inventory records are negative, unvalidated, or based on unposted journals that would distort planning output?
- What planned orders are near the firming horizon without human review or approval, risking auto-firming of incorrect quantities?
- What BOM versions or route operations are inactive, incorrectly configured, or based on stale data that would create production errors?
- What rollback path exists if a master plan run produces incorrect planned orders that are auto-firmed before review?
- What audit evidence is missing that supply chain managers or operations auditors would require?

## Evidence labels

Use `live evidence`, `report evidence`, `user-provided evidence`, `documentation-based`, or `inference`. Documentation alone never proves the user's live Dynamics 365 Supply Chain Management inventory positions, coverage settings, active BOM versions, or production schedule state.

## Live-guard gate

The following actions require explicit human confirmation and are out of scope for automated execution:

- Running or canceling a master plan in any production environment
- Modifying master plan parameters or coverage group settings in a production environment
- Activating or deactivating BOM versions or route configurations in a production environment
- Mass-firming planned orders in a production environment
- Modifying warehouse configuration, location directives, or work templates in a production environment
- Changing item coverage settings or safety stock levels in a production environment
- Modifying procurement policies, purchase trade agreements, or vendor sourcing rules in a production environment
