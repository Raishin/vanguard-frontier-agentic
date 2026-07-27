# Review Workflow And Output Contract

The system-inventory discovery workflow and the required output shape.

## Workflow

1. Identify the read-only discovery queries available (e.g. importlib.metadata) and the scope of the environment to inventory.
2. Enumerate runtimes, services, jobs, notebooks, and packages using only allowlisted list/get/describe queries.
3. Confirm each discovered asset carries a named owner, environment, deployment revision, and service identity; flag unowned or orphaned assets.
4. Classify criticality and data class per asset.
5. Confirm no raw credential, secret value, or token was retrieved, and record the evidence with its quality dimensions.

## Evidence labels

Label every claim: confirmed (independently observed) > inference (partial) > assumption (self-reported / not observed) > unknown, AND tag the evidence quality dimensions. Never present an assumption as confirmed, or evidence as proof.

## Output contract

- A verdict (approved / blocked / needs-review), the blockers (named conditions preventing execution; empty if approved), and the evidence level and quality dimensions of the discovery.
- Asset-discovery, ownership, and criticality/data-class findings.
- Control results, the audit event emitted, and safe next actions or open questions, including any authority the user must obtain.
