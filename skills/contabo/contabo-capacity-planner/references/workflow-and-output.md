# Workflow and output contract

Use this reference only when performing a full capacity planning review, deployment design, region evaluation, or instance tier comparison for a Contabo environment.

## Review domains

Check these areas before producing a capacity plan:

- Region selection: latency targets, compliance constraints, region availability, and redundancy requirements
- Instance tier fit: VPS (shared) vs. VDS (dedicated) vs. Storage VPS — CPU, RAM, storage, and network ceiling for the declared workload
- Contract period selection: 1/3/6/12-month obligations and their billing commitment at creation time
- Cloud-Init userData strategy: reproducibility, secret hygiene, and idempotency of the provisioning script
- SSH key strategy: Contabo secret IDs referenced in API payloads, never raw private key material
- Addon requirements: Private Networking (VLANs across instances), Additional IPs, Extra Storage, Custom Images
- Scaling topology: single-region vs. multi-region, horizontal scaling constraints, and network topology

## Safe workflow

1. **Frame scope**
   - Workload type and expected resource demand (CPU, RAM, storage IOPS, bandwidth):
   - Target region(s) and latency or compliance constraints:
   - Required contract period and budget envelope:
   - Cloud-Init or SSH key strategy requirements:
   - Explicit non-goals (e.g., no dedicated resources, no multi-region):

2. **Collect evidence**
   - Prefer user-provided workload data (utilization metrics, peak demand, team size, SLA targets).
   - Use read-only Contabo API calls for current instance inventory if live access is available.
   - Ground instance tier specs and region availability in official Contabo docs; label as `documentation-based` since specifications may change.
   - Label each claim as `live evidence`, `user-provided sanitized evidence`, `documentation-based`, or `inference`.

3. **Stress-test the plan**
   - Does the selected instance tier have headroom for peak load or burst?
   - Is the declared contract period the lowest lock-in option that still meets the cost target?
   - Does the region have all required instance types and addons available?
   - Is the Cloud-Init userData free of embedded secrets or unauthenticated remote execution?
   - What is the recovery path if provisioning fails mid-flight?

4. **Recommend the smallest safe plan**
   - Prefer the lowest contract period that meets the budget, unless a longer period is justified by stability and cost savings.
   - Always declare the contract period and billing obligation explicitly in the plan output.
   - If the safest action is to start with a single VPS and scale, say that plainly.

## Output contract

Return this structure:

```markdown
# Contabo Capacity Plan: <scope>
## Executive summary
- Status: READY TO DEPLOY / READY WITH CAVEATS / NEEDS EVIDENCE
- Selected region(s):
- Selected instance tier(s):
- Contract period: <1 | 3 | 6 | 12> months — billing obligation acknowledged
- Evidence level:
## Scope and assumptions
- Confirmed:
- Unknown:
- Out of scope:
## Instance plan
| Region | Instance tier | vCPUs | RAM | Storage | Contract | Monthly cost (est.) |
|---|---|---|---|---|---|---|
## Addon requirements
- <addon and justification or "none">
## Cloud-Init / SSH key strategy
- <strategy or "not required">
## Deployment sequence
1. <step> — validation: <check>
## Rollback or recovery path
- <path>
## Residual risk
- <risk or explicit none>
```
