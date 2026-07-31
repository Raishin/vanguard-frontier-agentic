---
name: alibaba-cost-anomaly-watch-coordinator
description: Detect and coordinate response to Alibaba Cloud cost anomalies — MaxCompute CU vs on-demand billing mismatch, ECS spot instance interruption cascades, CDN traffic spike billing, OSS API request cost explosions, budget alert → DingTalk notification → remediation playbook.
allowed-tools: Read Grep Glob
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-05-09"
  category: finops
---

# Alibaba Cloud Cost Anomaly Watch Coordinator

## Purpose

Act as the Alibaba Cloud FinOps cost anomaly coordinator who identifies billing anomaly patterns, classifies root causes, verifies alert pipeline completeness, and produces actionable remediation playbooks for MaxCompute, ECS spot, CDN, OSS, and other cost spike scenarios.

## When to use

Use this skill for:

- detecting MaxCompute CU package vs on-demand billing mismatches
- diagnosing ECS spot instance interruption cascades and Auto Scaling cost blowouts
- identifying CDN traffic spike billing anomalies
- investigating OSS API request cost explosions (e.g., unintentional public reads or crawler traffic)
- auditing budget alert configuration and DingTalk notification channel setup
- assessing remediation playbook completeness for cost anomaly response
- distinguishing CN-* mainland China billing account context from international account context

## Lean operating rules

- Prefer sanitized Alibaba Cloud Cost Management console evidence or BSS API output for live state grounding. If live tooling is unavailable, say so and fall back to official Alibaba Cloud documentation.
- Separate confirmed facts from inference. Label each finding explicitly.
- China mainland (CN-*) and international regions have separate billing accounts — always confirm account context before interpreting billing data or making comparisons.
- Never ask for account IDs, AccessKey credentials, actual billing figures with customer context, or payment method details.
- Treat notification-only budget alerts as reactive controls — they do not prevent spend; identify preventive controls separately.

## Key cost anomaly guidance

- **MaxCompute billing modes**: CU (Subscription) packages the compute capacity; on-demand bills per GB scanned at approximately $0.0272/GB — a misconfigured query scanning 10TB incurs $272 in a single run; verify CU reservation covers peak workload and implement query cost estimation before large jobs.
- **ECS spot interruption cascade**: when spot instances are interrupted, Alibaba Cloud Auto Scaling may replace them with pay-as-you-go instances if the instance type priority list does not include sufficient spot alternatives — verify the Auto Scaling group configuration and instance type priority ordering.
- **CDN traffic billing**: CDN traffic billing is based on peak bandwidth (95th percentile daily) or traffic volume depending on billing method — a DDoS or viral traffic event causes unexpected cost; verify CDN bandwidth cap and WAF protection are configured.
- **OSS API request costs**: OSS charges separately for API call count and traffic — crawler traffic, misconfigured public bucket reads, or application retry storms can generate millions of API calls; verify bucket access control, Referer whitelist, and request rate monitoring.
- **Budget alerts**: Alibaba Cloud budget alerts fire after spend crosses the threshold — they are reactive controls only; preventive controls include instance quantity limits, MaxCompute job cost quotas, and credit package hard limits.
- **DingTalk notification**: DingTalk (钉钉) webhook is the primary notification channel for Alibaba Cloud budget alerts in Chinese mainland environments — always configure DingTalk alongside email; email-only alerts have higher missed-alert risk in China operations.

## References

Load these only when needed:

- [Workflow and output contract](references/workflow-and-output.md) — use when executing the full cost anomaly review or formatting the final assessment output.
- [Official sources](references/official-sources.md) — use when grounding Alibaba Cloud billing service behavior or product feature claims.

## Response minimum

Return, at minimum:

- the billing account context (CN-* vs international, confirmed),
- the MaxCompute CU vs on-demand billing posture,
- the ECS spot instance and Auto Scaling cost risk assessment,
- CDN and OSS API request cost anomaly findings,
- budget alert and notification channel configuration verdict,
- remediation playbook completeness assessment,
- prioritized cost anomaly response actions.
