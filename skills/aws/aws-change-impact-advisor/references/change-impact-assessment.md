# Change Impact Assessment Guide

Use this reference when assessing planned AWS changes, rollback readiness, blast radius, maintenance windows, change calendars, dependency risk, and go/no-go recommendations.

## What people get wrong

The lazy story is:

> If the deployment tool accepts the change, impact is understood.

Wrong. Tool acceptance is not impact analysis. A valid change can still replace stateful resources, breach a freeze window, break downstream consumers, or leave the business without a rollback decision.

Common bad assumptions:

- A CloudFormation change set, Terraform plan, or pipeline diff is automatically business-safe.
- Low line-count equals low blast radius.
- Rollback exists because Git can revert the commit.
- Change calendars and maintenance windows are scheduling details, not controls.
- A service owner approval covers dependent teams.
- Read-only evidence from one Region proves global readiness.

## Change-specific failure modes

- Change set or plan hides replacement, delete, interruption, or data-retention risk.
- Dependency owners are missing for shared VPCs, IAM roles, KMS keys, event buses, queues, DNS, or certificates.
- Rollback requires data migration reversal, alias retargeting, previous image digests, parameter restoration, or manual state repair.
- Maintenance window conflicts with business freeze, AWS Health event, support case, or downstream release.
- Monitoring and alarms do not cover the changed path.
- Approval is given by the implementer instead of an accountable service owner.

## Minimum safe workflow

1. Identify the change type, target account/Region/environment, owner, and requested execution window.
2. Gather repository diff, change set/plan if available, affected services, dependencies, and rollback artifact.
3. Classify blast radius: customer-facing, data-bearing, identity/network, cost-affecting, or control-plane only.
4. Check scheduling controls: change calendar, maintenance window, freeze period, AWS Health context, and stakeholder availability.
5. Separate confirmed evidence from assumptions; do not fill unknowns with optimism.
6. Return a go/no-go recommendation with blockers, conditions, rollback notes, and communication plan.
7. Keep the role advisory; do not approve or execute the change from this skill.

## Verification targets

- change set, Terraform plan, CDK diff, SAM/CloudFormation diff, or pipeline release notes
- resource replacement/delete/interruption indicators
- rollback artifact: previous template, alias target, image digest, parameter set, migration rollback, or restore point
- impacted AWS services, accounts, Regions, VPCs, IAM principals, data stores, queues, DNS records, and certificates
- maintenance window, Change Calendar/Change Manager request, approval record, and on-call coverage
- CloudWatch alarms, AWS Health events, support cases, and incident backlog relevant to the change window

## When to push back

Push back if the user asks to:

- declare low risk from a diff without blast-radius evidence
- ignore replacement/delete risk because validation passed
- proceed without owner approval or rollback artifact
- execute during a freeze, health event, or unresolved incident without risk acceptance
- treat advisory review as deployment approval
- hide unknowns from stakeholders
