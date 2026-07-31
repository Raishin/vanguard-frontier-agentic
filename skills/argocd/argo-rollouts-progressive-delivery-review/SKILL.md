---
name: argo-rollouts-progressive-delivery-review
description: Use this skill when reviewing Argo Rollouts progressive delivery configuration. Trigger when the user asks about canary or blue-green Rollout strategy correctness, AnalysisTemplate success/failure conditions, traffic weighting provider alignment, canaryService isolation, PDB deadlock risk with Rollout maxSurge settings, automated rollback posture, or manual vs automated promotion configuration.
allowed-tools: Read Grep Glob
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-05-05"
  category: delivery
---

# Argo Rollouts Progressive Delivery Review

## Purpose

Review Argo Rollouts canary and blue-green strategy configuration, AnalysisTemplate success and failure condition correctness, traffic management provider alignment, canaryService vs stableService isolation, PDB compatibility with Rollout surge settings, and automated rollback posture. Argo Rollouts' safety depends entirely on AnalysisTemplate conditions that actually fail — an always-true successCondition means automated rollback never fires, regardless of actual error rates.

## Lean operating rules

- Prefer live evidence (`kubectl get rollout -A -o yaml`, `kubectl get analysistemplate -A -o yaml`, `kubectl argo rollouts status <name>`) when the active client exposes it; otherwise fall back to official Argo Rollouts documentation and sanitized YAML from the user.
- Separate confirmed facts from inference. If AnalysisTemplate metric query results, traffic provider actual behavior, or PDB state was not directly queried, say so.
- Treat an AnalysisTemplate with a successCondition that always evaluates to true (e.g., `result >= 0`, `true`) as a critical finding — automated rollback can never fire.
- Treat a Rollout with no separate `canaryService` from `stableService` as a high finding — canary traffic isolation is broken.
- Treat a production Rollout using `pause: {}` (manual promotion) with no AnalysisTemplate as a high finding — there is no automated quality gate.
- Treat a traffic provider in `spec.strategy.canary.trafficRouting` that does not match the actual ingress controller installed in the cluster as a high finding — weight changes are silently ignored.
- Treat `failureLimit: 100` or higher on an error-rate metric as a medium finding — the analysis tolerates far too many errors before marking Degraded.
- Keep the answer scoped, evidence-labeled, and explicit about what was not queried.

## References

Load these only when needed:
- [Workflow and output contract](references/workflow-and-output.md)

## Response minimum

Return, at minimum:
- the scoped target (Rollout name, AnalysisTemplate name, or traffic provider config) and evidence level,
- the deployment strategy (canary with steps vs canary without steps, blue-green) and whether steps include AnalysisRun gates,
- AnalysisTemplate successCondition and failureCondition correctness,
- canaryService vs stableService isolation posture,
- traffic provider alignment with the actual cluster ingress,
- PDB compatibility with Rollout maxSurge/maxUnavailable,
- the safest next actions and any assumptions or blockers.
