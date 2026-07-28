# System-Inventory Review Checklist

The per-concern checklist applied to every system-inventory discovery.

- Runtime, service, job, notebook, and package discovery uses only read-only list/get/describe queries — no mutation.
- Every discovered asset has a named owner; unowned or orphaned assets are flagged, not silently accepted.
- Every discovered asset records its environment and deployment revision.
- Every discovered asset records a service identity (not a shared/generic identity).
- Criticality and data class are classified per asset to scope downstream controls.
- No raw credential, secret value, keystore, or token is ever retrieved — only identity references are recorded.
