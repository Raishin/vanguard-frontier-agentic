# Dependency Matrix and Proof

Everything outside Snowflake that decides whether a promotion is a recovery, and how to turn a claimed RPO/RTO into a proven one. Load for any readiness assessment or drill design.

## The dependency matrix

- **Identity provider** — can users and services authenticate against the secondary, and are the roles and grants present there?
- **DNS and connection strings** — which names must change, who changes them, and what the TTL is.
- **Secrets and credentials** — are key pairs, tokens, and integration credentials valid for the secondary, and are any of them region-scoped?
- **Orchestration and ETL** — does the scheduler point at the secondary, and does it need to be told the primary changed?
- **External stages and cloud storage** — is the storage reachable and permitted from the secondary region, and does it exist there at all?
- **Streaming producers** — can producers reach and write to the promoted account, and what happens to in-flight offsets?
- **External functions and external access integrations** — do the outbound paths exist from the secondary?
- **BI tools and applications** — how do they reconnect, and who reconfigures them?
- **Native Apps and shares** — do consumers follow the promotion, or do they need action?
- **Downstream exports and consumers** — who receives data from this account and how are they told the source moved?
- Mark each entry inventoried, tested, or proven. Three states, because 'we listed it' and 'we tested it' are as different as 'we replicate' and 'we recovered'.

## Turning claimed into proven

- A drill proves only what it exercised. Record: what was executed, what was simulated, what was skipped, who participated, how long each phase took, and what surprised them.
- The most valuable drill output is the list of things that did not work. A drill with no findings usually tested less than it claims.
- Time the RTO from the business's perspective — from the decision to the point where the business can work — not from the promotion command to the promotion completing. The gap between those two is where recovery plans fail.
- Measure RPO empirically at drill time from the last successful refresh, and compare it to the requested value. If they differ, the requested value is a commitment the organization cannot currently meet, and that is an escalation, not a footnote.
- Drill evidence ages. Every new integration, consumer, or external dependency added since the last drill is untested, so record the estate delta alongside the drill date.
- **Failback** is tested separately and almost never is. Without it, a successful failover leaves production in a region that was sized, priced, and connected as a secondary, indefinitely.

## Before any promotion

- Require, in writing and before anything else: an incident or drill declaration, a named incident or DR owner, the approved target, the replication freshness and the resulting estimated data-loss window, current RPO and RTO status, dependency readiness, the client redirection plan, integration readiness, the failback strategy, and business acknowledgement where the data-loss window is material.
- State the data-loss window as a number of minutes and a description of what those minutes contain. 'Some recent transactions' is not an approvable statement.
- Promotion without dependency readiness is not disaster recovery. It is an outage moved to another region, and it is harder to reverse than the one it replaced.
- Urgency is the condition under which this list is most likely to be skipped and most costly to skip. The gate exists for exactly this moment.
