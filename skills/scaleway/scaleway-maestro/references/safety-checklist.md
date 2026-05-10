# Safety checklist

Use this reference before routing any request that may involve privileged access, live mutations, or compliance-impacting Scaleway operations.

## Non-negotiables

- Never route to `scaleway-live-kapsule-rollout-guard-agent` based on vague or implicit mutation intent. Explicit user confirmation of live mutation intent is required.
- Do not invent cluster IDs, project IDs, organization IDs, resource names, zones, or Scaleway service quotas.
- Do not ask users to provide `SCW_ACCESS_KEY`, `SCW_SECRET_KEY`, or any raw credential material during classification.
- Do not guess the domain if classification signals are genuinely ambiguous — ask one focused clarifying question instead.
- Keep the routing verdict minimal: domain, specialist, rationale. Do not answer the specialist's question.

## Stress checks

- Is the request advisory-only or does it involve a live system change?
- Does the request span multiple domains — and if so, is the primary domain correctly identified?
- Is the live-guard agent being triggered by a vague reference to Kubernetes, or by explicit mutation intent?
- Is any credential or sensitive identifier visible in the request that should not be forwarded?
- Is the routing verdict traceable to actual signals in the request, or is it inferred from incomplete context?

## Evidence labels

Use `confirmed signal` (present in the request), `inferred signal` (deduced from context), or `ambiguous` (insufficient to classify). Routing on ambiguous signals without asking is a defect.
