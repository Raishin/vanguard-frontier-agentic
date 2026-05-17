---
name: marketing-gpc-signal-honoring-review
description: Use this skill when reviewing the technical path by which a Global Privacy Control opt-out signal travels through the tag stack and CMP to determine whether ad tags, server-side forwarding, and conversion APIs actually cease firing. Trigger when a user provides a tag-manager container export, a CMP opt-out configuration, a server-side tag configuration, or asks whether their GPC implementation actually stops ad tags from firing, whether CPRA opt-out obligations are met technically, or whether the CMP acknowledges GPC but fails to suppress downstream tag execution.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-17"
  category: compliance
  lifecycle: experimental
---

# Marketing GPC Signal Honoring Review

## Purpose
This skill reviews the technical signal path by which a Global Privacy Control (GPC) opt-out travels from the browser header through the consent management platform (CMP) and tag manager to determine whether ad tags, server-side conversion forwarding, and conversion API calls actually cease firing. GPC is a legally recognized opt-out signal under CPRA (Cal. Civ. Code §1798.135) and the California CPPA enforcement sweeps of September 2025 confirmed that acknowledging GPC in the CMP UI while failing to suppress downstream tag execution constitutes a violation. The review distinguishes between cosmetic compliance (the CMP reads the GPC header and sets a cookie) and substantive compliance (the GPC state variable gates every ad tag firing rule and every server-side forwarding path). It also catches the pre-first-visit gap: users who set GPC before arriving for the first time receive no opt-out cookie and are therefore not suppressed. Artifact inputs: tag-manager container export and CMP opt-out configuration, annotated with which firing rules reference the GPC/opt-out variable.

## Lean operating rules
- Treat ad conversion tags that remain in active firing rules with no GPC-state condition as HIGH — if the CMP acknowledges the opt-out in the UI but the tag-manager container has no GPC variable guard on those rules, the opt-out is not honored technically and constitutes a CPRA violation per CPPA Sept 2025 enforcement guidance.
- Treat server-side conversion API events (Meta CAPI, Google Enhanced Conversions, TikTok Events API) forwarded from a first-party endpoint that bypasses the CMP entirely as HIGH — the first-party routing does not exempt the forwarding from opt-out obligations; the GPC state must be checked before forwarding occurs.
- Treat a CMP that sets an opt-out cookie on opt-out but does not suppress tags for users who set GPC before their first visit (no prior consent record) as HIGH — pre-first-visit GPC must suppress all non-essential tags on the first page load, not only after cookie creation.
- Treat CMP-acknowledged GPC that is not propagated as a boolean variable to the tag-manager firing rules as HIGH — CMP acknowledgment without tag-layer propagation leaves all existing rules unaffected.
- Treat Opt Me Out Act (AB 566, Oct 2025) obligations for opt-out link placement as MEDIUM when the GPC signal path is technically broken — surfacing the link is insufficient if the signal is not honored downstream.
- Flag ad tags that check a consent cookie but not the GPC header directly as MEDIUM — cookie-only checks fail for users who clear cookies but retain GPC, and for fresh sessions where no cookie yet exists.
- Flag the absence of a documented test procedure confirming GPC suppression across the full tag list as MEDIUM — attestation of compliance requires evidence, not assumption.
- Flag MEDIUM when server-side tag configurations do not log GPC-state at the time of forwarding — without logging, an enforcement sweep cannot demonstrate suppression.
- Do not recommend disabling all tags as the remediation — identify the specific firing-rule conditions missing a GPC variable guard and propose the minimal surgical fix.
- Label every finding with evidence basis: container provided, CMP config provided, documentation-based, or inference from missing config.

## References
Load these only when needed:
- [Workflow and output contract](references/workflow-and-output.md) — use when executing the full review or formatting the final answer.

## Response minimum
Return, at minimum:
- GPC variable propagation assessment (CMP to tag-manager variable layer)
- Firing-rule guard assessment (which ad tags lack a GPC-state condition)
- Server-side forwarding path assessment (CAPI, Enhanced Conversions, Events API bypass)
- Pre-first-visit suppression assessment (fresh session with GPC, no prior cookie)
- Opt Me Out Act link/signal consistency assessment
- Severity-labelled finding list (critical / high / medium / low)
- Safe next actions
