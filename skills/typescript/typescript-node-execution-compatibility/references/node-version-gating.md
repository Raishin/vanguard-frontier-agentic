# Node Version And API Gating

How to establish the Node version and what changes across the supported lines.

- Node's release schedule is the authoritative source for which major is Current, Active LTS, or Maintenance at any point in time — a support-window claim must cite the schedule, not a remembered assumption.
- As of this review's evidence, v26 is Current, v24 is Active LTS, and v22 is Maintenance — a deployment target running an already-EOL major (such as v25) carries no security-patch guarantee, and any type-stripping or runtime-syntax claim for it should be flagged as unsupported.
- A CLI flag, API, or default behavior documented for one Node major is not automatically present or unchanged in another; every runtime claim must name the specific Node version it was verified against.
- The condition-ordering rules in `exports`/`imports` (`types` first, `default` last, most-specific-first) apply at the version documented; confirm the target Node major against current documentation rather than an older cached understanding.
