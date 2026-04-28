# Workflow and Output Contract

## Workflow

1. Classify the operating context.
   - greenfield or brownfield,
   - single-subscription or multi-subscription,
   - enterprise platform or team-local setup,
   - who owns the platform: central platform team, security/governance team, app teams, or mixed.
2. State missing facts before pretending certainty.
   - regulated or not,
   - connectivity model,
   - identity source and privileged access model,
   - regional footprint,
   - shared-services expectations,
   - management/monitoring/SOC ownership,
   - recovery objectives.
3. Map the problem to landing-zone design areas.
   - identity and access,
   - resource organization,
   - network topology and connectivity,
   - security,
   - management,
   - governance,
   - platform automation.
4. Define the target platform model.
   - management-group hierarchy shape and why,
   - subscription segmentation and placement,
   - platform landing zones versus application landing zones,
   - shared services placement,
   - connectivity pattern and tradeoffs,
   - baseline governance and policy inheritance points.
5. Stress-test dependencies that are often skipped.
   - logging and monitoring foundation,
   - backup and disaster recovery assumptions,
   - identity administration and privileged access,
   - policy rollout sequencing and exceptions,
   - DNS, routing, and private-access consequences,
   - cost and operational ownership boundaries.
6. Challenge unsafe defaults.
   - one giant subscription,
   - flat management-group hierarchy by habit,
   - hub-spoke assumed without requirements,
   - broad admin access for convenience,
   - “we will add governance later”,
   - calling a landing zone complete without management and recovery posture.
   - inventing a separate AI landing zone by default when normal application landing zones already satisfy the governance boundary.
7. Route narrower follow-up work when needed.
   - RBAC specifics -> `azure-rbac-review` or a role-selection skill,
   - policy-detail design -> governance/policy skill,
   - network-depth review -> network topology skill,
   - automation delivery model -> platform automation skill.

## Output contract

Return all of the following:

- **Architecture verdict**: whether the current or proposed landing-zone model is sound, risky, or incomplete.
- **Target platform model**: recommended hierarchy, subscription model, and platform/application boundary.
- **Design-area decision table**: for each relevant design area, give current state, risk, recommendation, and dependency.
- **Unresolved risks**: explicit gaps, assumptions, and blocked decisions.
- **Next actions**: prioritized steps, sequenced so governance and operational dependencies are not deferred into fantasy.
- **Evidence used**: Microsoft Learn pages and any Azure MCP namespaces actually used.

## Eval gate

The skill output is not acceptable unless it:

1. distinguishes greenfield versus brownfield context,
2. identifies platform ownership assumptions,
3. covers every relevant landing-zone design area,
4. separates platform landing zones from application landing zones,
5. addresses management-group and subscription placement,
6. surfaces governance, security, management, and recovery dependencies,
7. states assumptions and unknowns explicitly,
8. avoids broad-admin or one-size-fits-all hierarchy advice.

Fail the response if any major design area is skipped or if the answer collapses into generic Azure architecture prose.

## Safety notes

- Do not recommend a single canonical management-group hierarchy without business and governance context.
- Do not recommend tenant-wide, management-group-wide, or subscription-wide admin access unless the user has justified the blast radius.
- Do not treat platform, connectivity, identity, and governance as separable afterthoughts; in landing-zone work they are coupled.
- Do not claim the landing zone is production-ready without management, monitoring, backup, and disaster-recovery posture.
- Be explicit when guidance is inference from incomplete tenant facts rather than confirmed current-state evidence.
