# Wave Planning, Dual Run, and Rollback

How to sequence a migration, what dual running really costs, and when rollback stops being available. Load when writing or reviewing a migration plan.

## Sequencing

- Sequence by three factors together: dependency (what must move before what), risk (what fails worst), and value (what the business gets). Sequencing by ease alone produces early progress and late benefit.
- Place at least one genuinely valuable workload in an early wave. A programme that spends its first two quarters on low-value easy wins has no benefit story when it needs one.
- Identify the workloads that anchor others — shared dimensions, reference data, common extracts — and move them early because everything else waits on them.
- Classify each workload explicitly, including the ones that stay: migrate now, migrate later, redesign first, leave in place, retire. The 'retire' category is routinely the highest-return finding in the inventory and is routinely absent from migration plans.

## Dual running honestly

- Dual running means paying for both platforms plus the engineering to keep them consistent. That third term is the one omitted from plans and it is often the largest.
- State what ends the dual run for each wave, with a date and an owner. An undated dual run becomes a permanent second platform, and the migration's business case quietly inverts.
- Decide which platform is authoritative during the dual run and make that explicit to consumers. Two authoritative sources is how two versions of a number reach the same meeting.
- Use the dual run to gather the target cost baseline empirically. Pre-migration target cost is an estimate; dual-run metering is a measurement, and it is the moment the business case can be checked rather than believed.

## Cutover and the expiry of rollback

- Cutover is a sequence, not a moment: freeze source changes, run the final incremental load, reconcile, redirect consumers, verify, and release the freeze. Each step has an owner and a verification.
- Consumer redirection is the underestimated part. Enumerate every report, extract, scheduled job, spreadsheet connection, and downstream system, and name who redirects each one.
- Define the point of no return per wave and write down what makes it irreversible — usually the moment the source stops being fed and its data stops being current.
- State the date each wave's rollback expires. A rollback plan with no expiry date implies an option that no longer exists, and the plan will be relied upon precisely when it has lapsed.
- Where a control was lost or weakened in the target, cutover is the last moment it can be raised cheaply. Confirm every acknowledged security gap has an owner and a remediation date before the freeze lifts.
