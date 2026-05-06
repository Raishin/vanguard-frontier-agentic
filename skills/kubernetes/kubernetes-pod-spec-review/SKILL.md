---
name: kubernetes-pod-spec-review
description: Use this skill when reviewing a Kubernetes Pod spec, Deployment spec, or StatefulSet spec for correctness, security posture, and production-readiness. Trigger on any request to audit, validate, or score a workload manifest.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-05"
  category: platform
---

# Kubernetes Pod Spec Review

## Purpose

Review Kubernetes Pod, Deployment, and StatefulSet specifications for probe correctness, resource QoS configuration, securityContext posture, image pull policy safety, secret consumption patterns, topology spread, and termination grace period alignment. Output a structured findings list with severity, evidence, and safe remediation steps — aligned with CKAD domain knowledge and production-readiness standards.

## Lean operating rules

- Check both `livenessProbe` and `readinessProbe`; flag missing probes as HIGH for Deployments receiving traffic. Flag aggressive `livenessProbe.failureThreshold` (<=2) that kills pods during GC pauses.
- Review `resources.requests` and `resources.limits`; flag missing requests (unschedulable under pressure) as MEDIUM and flag CPU limits without requests as Burstable QoS risk.
- Audit `securityContext` at both pod level (`runAsNonRoot`, `seccompProfile`) and container level (`allowPrivilegeEscalation: false`, `readOnlyRootFilesystem: true`, `capabilities.drop: [ALL]`).
- Flag `latest` image tag combined with `imagePullPolicy: IfNotPresent` as HIGH — image is never refreshed after first pull.
- Flag Secrets consumed via `envFrom.secretRef` (bulk-mount exposes all keys) as MEDIUM; recommend volume mounts or specific `env.valueFrom.secretKeyRef`.
- Check `topologySpreadConstraints` for multi-replica Deployments; flag absence as MEDIUM (single AZ failure = full outage).
- Review `terminationGracePeriodSeconds` against application drain time; flag default 30s for gRPC or database workloads as MEDIUM.
- Label all findings as live evidence, documentation-based, or inference.

## References

Load these only when needed:

- [Workflow and output contract](references/workflow-and-output.md)

## Response minimum

- Severity-labeled findings list (CRITICAL / HIGH / MEDIUM / LOW)
- Evidence source for each finding
- Specific field path that caused the finding (e.g., `spec.containers[0].livenessProbe`)
- Recommended remediation with example YAML snippet
- Overall production-readiness verdict
