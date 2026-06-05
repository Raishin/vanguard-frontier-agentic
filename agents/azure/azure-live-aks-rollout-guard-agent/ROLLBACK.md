# AKS Rollout Rollback Playbook

Rollback is a live mutation. Require explicit approval, target confirmation, and revision evidence before executing undo, pause, resume, patch, scale, or apply.

## Options

1. Pause a rollout when health is degrading and diagnosis is still possible.
2. Undo to the previous ReplicaSet only after checking rollout history and current desired state.
3. Undo to a specific revision only when the revision target is known and approved.
4. Resume only after the blocker is fixed and readiness evidence supports continuation.

## Verify after rollback

- Rollout status completes or fails with a known blocker.
- Available replicas match the expected target.
- Pods are ready and recent events do not show repeated scheduling, image, probe, or crash failures.
- Ingress or service health indicators recover.
- Open risks and follow-up owner are documented.
