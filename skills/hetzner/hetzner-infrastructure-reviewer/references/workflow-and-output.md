# Workflow and output contract

Use this reference only when performing a full infrastructure review, security posture audit, architecture assessment, or producing a structured review response.

## Review domains

Check these areas before giving a verdict:

- **Firewall rules** — inbound and outbound rules, protocol/port/CIDR scope, broad 0.0.0.0/0 exposure, management port exposure (SSH 22, RDP 3389), rule completeness
- **Firewall attachment** — which servers and Label groups are attached, unattached Firewalls (provide zero protection), attachment coverage gaps
- **Load Balancer config** — health check type (HTTP/HTTPS/TCP), health check path and interval, target pool size and server health, algorithm selection
- **Private Network topology** — subnet segmentation, server placement in Networks, routes and Network attachment coverage
- **IP exposure** — Floating IPs and Primary IPs in use, unnecessary public IP assignments, servers without Firewall coverage on public interfaces
- **Region distribution** — placement across fsn1 (Falkenstein DE), nbg1 (Nuremberg DE), hel1 (Helsinki FI), single-region risk for production workloads
- **Placement Groups** — spread vs packed strategy, groups assigned to servers with availability requirements

## Safe review workflow

1. **Frame scope**
   - Hetzner project, environment (prod / staging / dev):
   - Resource types in scope (Firewalls, Servers, LBs, Networks):
   - Business criticality and compliance driver:
   - Required outcome (audit report, remediation plan, specific control check):
   - Explicit non-goals:

2. **Collect evidence**
   - Prefer live Hetzner MCP read-only evidence if available.
   - Otherwise inspect user-provided sanitized API output, repository config, or official Hetzner docs.
   - Label each finding as `live evidence`, `user-provided evidence`, `documentation-based`, or `inference`.

3. **Stress-test risk**
   - What inbound paths expose services to the public internet without a Firewall?
   - What Firewalls exist but are unattached, providing zero protection?
   - What management ports are reachable from 0.0.0.0/0?
   - What Load Balancer targets are unhealthy or in single-region?
   - What evidence is missing that would change the security verdict?

4. **Recommend the smallest safe action**
   - Prefer narrow firewall rule tightening over broad rewrites.
   - Do not recommend live firewall mutations here — route those to `hetzner-live-firewall-rule-guard`.
   - If the safest action is to gather more evidence, say that plainly.

## Output contract

Return this structure:

```markdown
# Hetzner Cloud Infrastructure Review: <scope>
## Executive verdict
- Status: SECURE / SECURE WITH RISKS / GAPS FOUND / NEEDS EVIDENCE
- Biggest risk:
- Evidence level:
## Scope and assumptions
- Confirmed:
- Unknown:
- Out of scope:
## Findings
| Severity | Finding | Evidence | Why it matters | Minimum safe action |
|---|---|---|---|---|
## Recommended actions
1. <action> — owner: <owner>, validation: <check>, rollback: <rollback>
## Validation
- Commands or checks:
- Expected result:
## Residual risk
- <risk or explicit none>
```
