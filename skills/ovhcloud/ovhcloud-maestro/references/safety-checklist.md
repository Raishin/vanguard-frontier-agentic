# Safety checklist

Use this reference before any routing decision that may dispatch to a live-guard or privileged specialist.

## Non-negotiables

- Never attempt live OVHcloud API calls, Terraform mutations, or credential operations from the routing layer.
- Never ask users to paste API tokens, application keys, OAuth2 client secrets, or account passwords into chat.
- Do not invent account IDs, project IDs, region codes, resource URNs, or service names.
- Treat an ambiguous domain as a blocker — ask one clarifying question rather than routing to the wrong specialist.
- If the request contains signals for the live-guard KMS skill, confirm explicitly before routing. Key destruction is irreversible.
- Keep routing output minimal; do not speculate about specialist-layer findings before routing.

## Stress checks

- Is the domain actually clear, or is the classification an inference?
- Does the request contain any live-guard signals (key destruction, irreversible deletion, production mutation)?
- Is there enough scope information to give the specialist a useful handoff, or does the user need to provide more context first?
- Could routing to the wrong specialist expose privileged evidence or trigger an inappropriate action?

## Evidence labels

Use `live evidence`, `repo evidence`, `user-provided evidence`, `documentation-based`, or `inference` when describing classification signals. Inference alone never justifies routing to a live-guard specialist.
