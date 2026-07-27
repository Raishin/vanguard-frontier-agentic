# Routing Taxonomy And Modes

The live-plane domain-to-specialist map, routing modes, and the live-guard-gate handoff rule.

- Single mode routes to exactly one read-only-runtime specialist; parallel (N) is capped at four and used only when a task genuinely spans that many read-only domains.
- runtime-evidence-gate routes a read-only action that requires captured evidence before a downstream decision.
- live-guard-gate NEVER auto-dispatches a mutating-runtime operator — it is surfaced only with an external signed approval bound to the target and plan digest, target-scoped JIT credentials, and a pre-approved rollback, gated to a named human owner.
- unclassified is returned, and the smallest sufficient applicability-input set is requested, whenever org, jurisdiction, data class, environment, financial/PCI/health/personal scope, or AI-system role is missing for an R3+ action.
