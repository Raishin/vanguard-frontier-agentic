# Workflow and output contract — SAP Release and Change Collision Review

Use this reference for all collision risk classification, governance gap assessment, retrofit evaluation, and output formatting.

## Collision risk dimension taxonomy

| Dimension | Description |
|-----------|-------------|
| `transport-sequencing` | Transport request order in the import queue; object-level dependency documentation; sequencing gap identification |
| `overtake-overwrite-risk` | Transports from parallel tracks modifying the same repository objects; last-write-wins collision at import; import queue sequence control |
| `parallel-project-conflict` | Concurrent SAP projects sharing development or quality systems; object ownership enforcement; cross-project collision detection gate |
| `retrofit-dual-landscape` | Retrofit transport creation and sequencing for dual landscape; retrofit completeness tracking; dual landscape object integrity |
| `downgrade-protection` | Downgrade protection configuration at system and client level; emergency bypass detection; bypass authorization and documentation |
| `change-governance-coverage` | Change record coverage for all production transports; approval workflow enforcement; urgent correction and emergency change procedure |

## Landscape topology classification

| Topology | Description | Collision risk profile |
|----------|-------------|----------------------|
| `single-track-three-tier` | Standard DEV → QAS → PRD; single development line | Lower inherent collision risk; risk arises from parallel workstreams within one DEV system |
| `dual-landscape` | Parallel maintenance (MDEV → MQAS → PRD) and main development (DEV → QAS → PRD) lines sharing production | High retrofit collision risk; requires controlled retrofit procedure to maintain object integrity |
| `multi-track-shared-target` | Multiple parallel development tracks (feature teams, parallel projects) converging on a shared QAS or PRD | Highest collision risk; requires object-level analysis at every promotion gate |
| `sandbox-pool` | Multiple sandbox or development tenants with periodic consolidation to a shared QAS | Requires consolidation governance and conflict resolution process at QAS import gate |

## Collision risk status values

Each dimension is assessed as: `MANAGED` / `PARTIALLY-MANAGED` / `UNMANAGED` / `NOT-YET-ASSESSED`

| Risk dimension | Managed criteria | Common unmanaged indicators |
|----------------|-----------------|----------------------------|
| `transport-sequencing` | Import queue sorted by dependency; object-level dependency documentation exists; sequencing verified before QAS import | Transports imported in creation date order without dependency review; no object-level analysis performed |
| `overtake-overwrite-risk` | Object-level analysis performed for all parallel tracks; confirmed no shared object modifications without coordination; import queue sequence controlled | Parallel tracks developed independently with no object ownership check; conflicts discovered at production import |
| `parallel-project-conflict` | Named object ownership per development object across all parallel tracks; collision detection gate at QAS import; conflict resolution procedure documented | No cross-project object ownership registry; conflicts discovered reactively at import failure |
| `retrofit-dual-landscape` | All productive changes retrofitted within defined SLA; retrofit transport list maintained and sequenced; dual landscape object comparison performed per release | Retrofit backlog exists with no catchup plan; retrofit transports created but not sequenced; object divergence between lines not tracked |
| `downgrade-protection` | Downgrade protection active in production client; bypass procedure requires named authorization; all bypasses documented and post-reviewed | Downgrade protection disabled or not configured; emergency transports bypass without documented authorization; no post-bypass audit |
| `change-governance-coverage` | 100% of production transports linked to approved change records; approval workflow enforced; urgent corrections have post-change review within defined SLA | Transports imported to production without change records; approval workflow bypassed for urgent corrections; no post-emergency-change review |

## Downgrade protection gap classification

| Gap type | Risk level | Description |
|----------|-----------|-------------|
| `protection-not-configured` | production-blocking | Downgrade protection not configured at system or client level in production |
| `protection-bypassed-without-authorization` | production-blocking | Emergency transport bypassed downgrade protection without named authorization and documentation |
| `bypass-not-post-reviewed` | high | Authorized downgrade protection bypass not reviewed after implementation to confirm risk was as expected |
| `protection-not-tested` | medium | Downgrade protection configured but never tested to confirm it detects retrograde object moves correctly |
| `bypass-procedure-undocumented` | medium | No documented procedure for authorized downgrade protection bypass — response in emergency is ad hoc |

## Retrofit assessment criteria

A governed retrofit process in a dual landscape must include:

- Retrofit trigger: definition of which productive changes require retrofit (typically all normal changes and urgent corrections that modify development objects)
- Retrofit SLA: maximum time allowed between productive import in maintenance landscape and retrofit completion in main development landscape
- Retrofit transport creation procedure: how retrofit transports are created (manual, ChaRM-assisted, or automated comparison tool)
- Retrofit sequencing: retrofit transports imported in the same relative order as their originating productive transports
- Dual landscape integrity check: object comparison between maintenance production baseline and main development line performed before main release
- Retrofit backlog governance: open retrofit items tracked and prioritized; backlog cleared before next main release

## Workflow

1. **Identify landscape topology** — classify as single-track-three-tier, dual-landscape, multi-track-shared-target, or sandbox-pool.
2. **Assess transport sequencing** — confirm dependency documentation and import queue sort order control.
3. **Evaluate overtake and overwrite risk** — determine whether object-level analysis is performed for parallel tracks before QAS and production import.
4. **Review parallel project collision risk** — assess object ownership governance and cross-project conflict detection gate.
5. **Assess retrofit governance** — if dual landscape, evaluate retrofit completeness, SLA, sequencing, and integrity check process.
6. **Evaluate downgrade protection** — confirm configuration status, bypass procedure, and post-bypass review.
7. **Review change governance coverage** — confirm change record coverage for all production transports, approval workflow enforcement, and urgent correction procedure.
8. **Assign overall collision risk posture** — `collision-risk-managed` (all dimensions MANAGED) / `conditional` (gaps with documented mitigation) / `collision-risk-unmanaged` (one or more dimensions UNMANAGED with production-blocking or high risk).
9. **Return output** per the output contract below.

## Output contract

Return:

1. Landscape topology classification and number of parallel tracks
2. Evidence label per dimension (documentation-based / user-provided evidence / inference)
3. Collision risk assessment table: dimension, status (MANAGED/PARTIALLY-MANAGED/UNMANAGED/NOT-YET-ASSESSED), gap description
4. Downgrade protection status and bypass governance assessment
5. Retrofit governance assessment (dual landscape only): completeness, SLA adherence, backlog status
6. Change governance coverage: percentage of production transports with linked change records (if quantifiable from user evidence)
7. Overall collision risk posture: collision-risk-managed / conditional / collision-risk-unmanaged
8. Risk level per dimension (production-blocking / high / medium / low)
9. Prioritized collision mitigation recommendations with STMS, ChaRM, or Cloud ALM documentation reference
10. Escalation trigger if ungoverned production transports, active downgrade protection bypass without authorization, or critical retrofit backlog is identified
11. Explicit advisory boundary reminder: this review does not import transports, modify change records, or authorize production releases
