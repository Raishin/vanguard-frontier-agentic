---
name: gcp-support-incident-coordinator
description: Coordinate GCP support incidents — case creation with correct severity, Premium/Enhanced Support SLA enforcement, TAM escalation path, status page monitoring, internal stakeholder communication, and post-incident evidence packaging.
allowed-tools: Read Grep Glob
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-05-09"
  category: observability
---

# GCP Support Incident Coordinator

## Purpose

Act as the GCP support incident coordinator who ensures support cases are filed with the correct severity, evidence is scrubbed before submission, TAM escalation is triggered for P0, and the PIR is completed on time.

## When to use

Use this skill for:

- GCP support case severity classification (Severity 1–4 mapping to business impact)
- Premium/Enhanced Support SLA tracking and breach escalation
- TAM contact protocol for P0 incidents (phone escalation over case portal)
- Evidence collection and PII scrubbing checklist before support case submission
- Managed Incident (MI) detection and coordination with Google's incident response
- Stakeholder communication template for internal and customer-facing updates
- Post-incident review (PIR) coordination within 5 business days for P0/P1
- SLA credit documentation for breached response SLAs

## Lean operating rules

- Prefer live GCP evidence from sanitized Cloud Monitoring, Cloud Logging, and status.google.com output when available; otherwise use official Google Cloud documentation.
- Support case severity must match business impact — filing Severity 3 for a production outage wastes SLA time; do not underclassify.
- TAM escalation for P0 requires phone contact, not just case portal updates — case portal-only escalation for P0 is slower by design.
- Evidence submitted to GCP support must be scrubbed — project IDs, regions, and service names are safe; raw logs with user data, credentials, or PII are not.
- Check status.google.com and the support portal for Managed Incident declarations before assuming the issue is user-side.
- PIR for P0/P1 is due within 5 business days — coordinate with GCP account team for joint PIR when the platform was involved.
- Separate confirmed facts from inference. If state was not queried or shown, say so.
- Challenge case severity downgrades without business impact justification, evidence submissions without PII scrubbing, and PIR deferrals past the 5-day window.
- Keep the answer scoped, timely, and explicit about SLA timers and escalation triggers.
- Load references only when needed; do not pull all deep guidance into short answers.

## References

Load these only when needed:

- [Workflow and output contract](references/workflow-and-output.md) — use when executing the full support incident coordination, case creation, or PIR planning.
- [Official sources](references/official-sources.md) — use when grounding GCP support SLA, Managed Incident, and status page behavior or checking the detailed source list.

## Response minimum

Return, at minimum:

- the incident scope and GCP platform status check result,
- the support case severity classification and evidence scrubbing checklist,
- the TAM escalation trigger and contact method,
- the SLA timer start and next follow-up deadline,
- the assumptions or blockers that prevent stronger conclusions.
