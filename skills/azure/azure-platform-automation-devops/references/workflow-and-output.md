# Workflow and Output Contract

## Safe Workflow

1. **Classify the automation boundary**
   - platform landing zone, shared platform service, workload infrastructure, or app release;
   - bootstrap-only, steady-state run, or both.
2. **Separate ownership and blast radius**
   - who owns management-group or subscription bootstrap,
   - who owns platform baselines,
   - who owns workload delivery,
   - whether app teams can trigger infra mutations.
3. **Choose the IaC control model**
   - use Microsoft’s recommended Azure landing zone IaC accelerator when landing-zone delivery needs structured bootstrap and continuous delivery;
   - choose Bicep when Azure-native schema fidelity, ARM alignment, and Azure-first modularity are primary;
   - choose Terraform when cross-team standardization, existing Terraform operating model, or broader multi-environment consistency is the real driver;
   - reject tool choice by fashion.
4. **Split bootstrap from run**
   - bootstrap phase: initial credentials, repository wiring, runner/service-connection setup, state or deployment control prerequisites, management-group/subscription onboarding;
   - run phase: repeatable policy, networking, identity, platform service, and workload-safe deployment flows.
5. **Separate infrastructure and application pipelines**
   - infrastructure pipeline handles landing-zone and shared-platform changes,
   - application pipeline consumes approved platform contracts,
   - production application deployment should not bypass platform guardrails by mutating foundational infrastructure directly.
6. **Lock down secret handling**
   - no secrets in repo,
   - no secrets in pipeline variables unless the platform’s approved secret store and identity model justify it,
   - prefer managed identity, workload identity, or equivalent non-secret trust paths where the platform supports them,
   - minimize human-held credentials in bootstrap and rotate any unavoidable temporary credentials.
7. **Define validation gates**
   - schema/lint validation,
   - dependency and scope review,
   - plan or what-if style preview before mutation,
   - nonproduction deployment,
   - smoke or health validation,
   - explicit approval before production-impacting rollout,
   - documented rollback or reverse-deploy path.
8. **Choose safe release mechanics**
   - prefer staged rollout patterns for platform-impacting change,
   - for App Service, prefer nonproduction slot validation and swap-based promotion where applicable,
   - avoid direct-to-production deployment when staging, warm-up, and rollback are available.
9. **Return a go/no-go verdict**
   - include blockers, residual risks, required evidence, and the next safest step.

## Role-Specific Stress Checks

- Reject any design that mixes management-group bootstrap, platform governance rollout, and workload deployment into one opaque pipeline with shared credentials.
- Reject any “Bicep vs Terraform” answer that does not name the operating reason. Tool tribalism is not architecture.
- Reject pipelines that let application release jobs modify shared platform baselines without separate controls.
- Reject any secret-handling model that depends on pasting secrets into YAML, repo variables, markdown, or chat.
- Challenge any claim that “continuous deployment to production” is safe if preview, slotting, smoke validation, or approvals were skipped.
- Challenge bootstrap designs that require standing global admin or subscription-owner credentials long after initial setup.
- Distinguish `bootstrap convenience` from `steady-state safety`; they are not the same phase and should not keep the same privileges.
- Do not treat Azure MCP presence as proof that mutation should be automated. Capability is not authorization.

## Output Template

```markdown
# Azure Platform Automation Review: <scope>

## Verdict
- Status: READY / READY WITH RISKS / NOT READY
- Biggest risk:
- Evidence level: live evidence / documentation-based / sanitized evidence / inference

## Scope
- Platform boundary:
- Environment(s):
- Ownership model:
- Requested change:

## Delivery model
- IaC approach:
- Why this approach:
- Bootstrap phase:
- Run phase:
- Infra/app pipeline split:

## Control and safety findings
| Area | Finding | Severity | Evidence | Recommendation | Owner |
|---|---|---|---|---|---|

## Validation gates
| Gate | Required | Why | Blocking if absent |
|---|---|---|---|

## Rollout path
1.
2.
3.

## Rollback path
1.
2.
3.

## Open questions
- 
```

## Red Flags

- The plan deploys foundational Azure changes straight to production with no preview, no stage validation, and no rollback.
- The same pipeline identity can bootstrap tenant or subscription foundations and also run routine app releases.
- The answer says “use Terraform” or “use Bicep” without naming the actual operating constraints.
- The design stores secrets in source control, static pipeline YAML, copied variables, or chat.
- The pipeline mixes platform baselines, workload infrastructure, and application code promotion without separate control points.
- The recommendation assumes Azure MCP deploy or Bicep tooling exists in the runtime without live confirmation.
- The release strategy ignores App Service slot-based safety even when the target platform supports it.
