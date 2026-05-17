---
name: email-sender-authentication-review
description: Use this skill when reviewing DNS sender-authentication records for a marketing domain to identify policy gaps exposing campaigns to rejection, spoofing, or inbox displacement. Trigger when a user provides DNS TXT record exports for SPF, DKIM, DMARC, or BIMI, or asks whether their email authentication posture meets Google/Yahoo bulk-sender requirements, DMARC enforcement standards, CISA BOD 18-01 obligations, PCI DSS v4.0 Req 5.3.3, or whether their transactional or marketing emails are at risk of spoofing or bulk-sender quarantine.
allowed-tools: Read Grep Glob
metadata:
  author: "github: Raishin"
  version: "0.1.0"
  updated: "2026-05-17"
  category: compliance
  lifecycle: experimental
---

# Email Sender Authentication Review

## Purpose
This skill reviews DNS sender-authentication records (SPF, DKIM, DMARC, BIMI) for a marketing domain and its ESP subdomains to identify policy gaps that expose email campaigns to rejection, spoofing, or inbox displacement. Email authentication failures have grown from a deliverability concern to a compliance obligation: Google and Yahoo bulk-sender requirements (enforced 2024) mandate DMARC alignment for senders exceeding 5,000 messages per day; CISA BOD 18-01 requires federal domains to reach DMARC `p=reject`; and PCI DSS v4.0 Requirement 5.3.3 requires anti-phishing controls for outbound email. A `p=none` DMARC policy with no roadmap to enforcement, a missing DKIM selector for a transactional ESP subdomain, or an SPF record exceeding the ten DNS-lookup limit all constitute policy gaps that range from HIGH spoofing exposure to deliverability failure. The review assesses the full authentication stack from a sanitized DNS record export and surfaces the gap, its severity, and the surgical fix.

## Lean operating rules
- Treat DMARC policy `p=none` with no enforcement on a domain sending bulk marketing email as HIGH — `p=none` provides monitoring only; spoofing is possible, and Google/Yahoo bulk-sender requirements treat senders without at least `p=none` plus DKIM alignment as quarantine candidates; the path to `p=quarantine` or `p=reject` must be explicit.
- Treat a missing DKIM selector for any active ESP or transactional subdomain as HIGH — emails sent through that path are unauthenticated, cannot pass DMARC alignment, and are treated as unsigned by receiving MTAs; automation and transactional flows are commonly the most impactful to revenue.
- Treat an SPF record that exceeds ten DNS lookup mechanisms (`include:`, `a:`, `mx:`, `ptr:`) as HIGH — RFC 7208 defines this as a permerror, which receiving MTAs treat as an SPF fail, blocking all mail from that domain that relies on SPF for DMARC alignment.
- Treat a DMARC record with `rua=` absent (no aggregate reporting URI) as MEDIUM — without aggregate reports, the operator cannot see what is aligning and what is failing; DMARC without visibility is unmanaged.
- Treat SPF records using `+all` (pass all) as HIGH — this negates SPF entirely by authorizing any sending source; the entire domain is open to spoofing regardless of which sources are explicitly listed.
- Treat DMARC `pct=` below 100 as MEDIUM when `p=quarantine` or `p=reject` is set — partial enforcement leaves a configured percentage of non-aligning mail unaffected by the policy and creates a false sense of full enforcement.
- Treat a BIMI record present without a corresponding VMC or CMC certificate as LOW — BIMI without a validated certificate is ignored by major mailbox providers that require certificate-backed BIMI.
- Flag the absence of DKIM key rotation documentation as MEDIUM — DKIM keys that have never been rotated accumulate risk; PCI DSS v4.0 Req 5.3.3 and general key-hygiene practice require rotation procedures to exist.
- Do not recommend removing an ESP's SPF include without first confirming a DKIM-only alignment path is available — SPF removal without DKIM coverage breaks DMARC alignment for that sending path.
- Label every finding with evidence basis: DNS record provided, documentation-based, or inference from absent record.

## References
Load these only when needed:
- [Workflow and output contract](references/workflow-and-output.md) — use when executing the full review or formatting the final answer.

## Response minimum
Return, at minimum:
- SPF mechanism count and permerror risk assessment
- DKIM selector coverage assessment for all active sending paths
- DMARC policy and reporting configuration assessment
- DMARC alignment mode assessment (strict vs relaxed)
- BIMI and certificate assessment
- Bulk-sender requirement compliance status (Google/Yahoo)
- Severity-labelled finding list (critical / high / medium / low)
- Safe next actions
