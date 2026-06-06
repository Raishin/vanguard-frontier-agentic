# Landing Zone Governance Controls Guide

Use this reference for AWS landing zone, Control Tower, Organizations, OU, account vending, guardrail, delegated admin, centralized logging, and audit/security account reviews.

## What people get wrong

The lazy story is:

> Control Tower deployed successfully, so the landing zone is governed.

Wrong. Landing-zone governance fails through account sprawl, OU drift, missing delegated-admin scope, weak SCPs, logging gaps, and unclear account lifecycle ownership.

Common bad assumptions:

- OUs mirror the org chart safely.
- SCPs are complete guardrails rather than coarse permission boundaries.
- Centralized logging exists because an audit account exists.
- Account vending is safe without lifecycle, budget, and ownership controls.
- Sandbox accounts are low risk.
- Control Tower drift is only an operational nuisance.

## Landing-zone failure modes

- Workloads are placed in OUs with wrong controls, data residency, or production isolation.
- SCPs block break-glass or deployments, or fail to prevent risky services/Regions.
- CloudTrail, Config, Security Hub, GuardDuty, Macie, IAM Access Analyzer, and log archive coverage differ across accounts/Regions.
- Account factory/vending lacks owner, cost center, environment, network, identity, and decommission metadata.
- Delegated administrator roles create hidden privilege concentration.
- Shared network/log/security accounts become single points of governance failure.

## Minimum safe workflow

1. Identify organization scope, management account constraints, Control Tower state, OUs, Regions, and account inventory.
2. Classify accounts by environment, data sensitivity, workload criticality, ownership, and lifecycle stage.
3. Review preventive controls: SCPs, Control Tower controls, Region restrictions, IAM boundaries, and break-glass path.
4. Review detective controls: organization trails, Config aggregators, Security Hub, GuardDuty, Macie, log archive, and alert routing.
5. Check account vending and decommission workflow for owner, budget, network, identity, tagging, and closure criteria.
6. Return governance gaps with blast radius, evidence level, owner, and staged remediation.
7. Do not recommend management-account or SCP changes without explicit approval and rollback plan.

## Verification targets

- AWS Organizations OUs, accounts, delegated admins, SCPs, tag policies, backup policies, and Region restrictions
- AWS Control Tower landing-zone version, enabled controls, drift status, Account Factory/account vending process
- log archive/audit/security account design, CloudTrail organization trail, Config aggregator, and S3/KMS log protection
- identity center/federation, break-glass, permission sets, and management-account access controls
- shared networking accounts, VPC sharing, Transit Gateway ownership, and account baseline templates
- account owner metadata, budgets, tags, support model, and decommission evidence

## When to push back

Push back if the user asks to:

- flatten OUs for convenience without blast-radius analysis
- deploy SCPs without testing break-glass and pipeline impact
- call Control Tower deployment proof of full governance
- skip centralized logs or delegated-admin coverage checks
- create accounts without owner, budget, lifecycle, and data classification
- mutate management-account controls from advisory review alone
