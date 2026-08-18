# Recovery And State Surgery

The order of preference for changing the record, and what must exist before any of it.

- Configuration-level constructs are strictly preferable to state commands because they are reviewable, versioned, and reversible: a `moved` block records a rename in the repository, an `import` block records an adoption, and a `removed` block records a deliberate release — while `state mv` and `state rm` leave no trace anywhere except the state itself.
- `state rm` removes a resource from the record without touching the real infrastructure, which leaves it running and unmanaged; `terraform destroy` removes the infrastructure. Confusing the two in either direction is the most expensive single mistake available in an IaC estate.
- The `state` subcommands write a local backup before mutating, which protects against a malformed command but not against a corrupted, deleted, or overwritten remote backend — the local backup lives on the operator's machine and disappears with it.
- Backup existence and recovery capability are different properties. The measurable one is time-to-restore, and it is only known after someone has actually restored; an untested backup supports an assumption about recovery, never a control.
- A backend migration creates a window in which state exists in two locations, and an interruption inside that window leaves no unambiguous source of truth; the migration must be single-operator, gated, and must name which copy is authoritative at each step.
- Hand-editing a state file is defensible only when no command expresses the required change, and it requires the serial number and lineage to be understood before the edit — a state written with a stale serial or a mismatched lineage is rejected or, worse, silently supersedes a newer one.
- State surgery performed under incident pressure without a verified restorable copy converts a recoverable incident into an unrecoverable one, which is why the backup requirement tightens rather than relaxes when the situation is urgent.
