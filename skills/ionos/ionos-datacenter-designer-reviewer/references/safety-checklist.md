# Safety checklist

Use this reference before advising any structural IONOS DCD topology change, NIC reconfiguration, volume modification, or LAN restructure.

## Non-negotiables

- Never assess or advise on a topology change without a current DCD topology snapshot or IaC export as evidence — verbal descriptions alone are insufficient.
- Datacenter-level blast radius is the default assumption: a failed DCD structural change can disrupt all servers, LANs, and volumes within that datacenter simultaneously.
- GDPR data residency is a hard blocker — if the datacenter region does not match the declared processing location, flag this before any topology finding.
- Do not advise a structural change without a documented rollback path.
- Do not recommend removing private LAN isolation between application tiers without an explicit security review.
- Stay advisory — do not call DCD API endpoints or run Terraform apply.
- Do not invent resource UUIDs, LAN IDs, server names, or live configuration state.
- Label all claims: `live evidence`, `user-provided sanitized evidence`, `documentation-based`, or `inference`.

## Stress checks

- What resources share this datacenter and are affected by the proposed change?
- What can disrupt server connectivity if a LAN or NIC change fails mid-execution?
- What can make volume data inaccessible if a detach or volume move fails?
- Does a documented rollback path exist, and does it require downtime?
- Is the IaC state synchronized with the live DCD topology — or has drift occurred?
- Does the datacenter region match the declared GDPR processing location?
- Are any NICs publicly exposed that should be LAN-only?

## Evidence labels

Use `live evidence`, `user-provided sanitized evidence`, `documentation-based`, or `inference`. A verbal description of topology without a DCD export or IaC snapshot is `inference` — do not give a topology verdict on inference alone.
