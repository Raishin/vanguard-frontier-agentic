---
name: terraform-verification-strategy
description: "Use this skill to decide what verification a Terraform or OpenTofu change actually needs and what each option proves: `validate` versus a plan, `terraform test` run blocks with `command = plan` versus `command = apply`, mock providers, and assertions on the properties that would cause an outage. Procedure only — it produces a verification plan, not a pass/fail verdict; the owning agent (`terraform-reviewer` for module contracts, `terraform-plan-blast-radius-agent` for change safety) issues the verdict."
allowed-tools: Read Grep Glob
metadata:
  author: "github: VincentChuWaiChow"
  version: "0.1.0"
  updated: "2026-08-17"
  category: delivery
  lifecycle: experimental
---

# terraform-verification-strategy

## Purpose

This skill exists because deciding what to verify is a repeatable procedure, not an act of judgment that needs its own owner. Most IaC verification gaps come from a confusion about what each tool proves: `validate` proves the configuration parses, a plan proves what the engine intends given current state, a plan-mode test proves logic without touching infrastructure, and only an apply-mode test proves the resources can actually be created. Choosing between them is a lookup; deciding whether the result is good enough belongs to the agent that owns the change.

## Trigger conditions

- A module or configuration change needs a verification plan proportionate to its blast radius.
- A user is deciding between `terraform test` with `command = plan` and `command = apply`, or whether mock providers fit the case.
- A user needs to know what `validate`, a plan, or an existing test suite actually proves about a change.
- An existing test suite passes but did not catch a failure, and the gap needs naming.
- A property would be better asserted continuously after apply than once at change time.

## When not to use

- A verdict is needed on whether the change is adequately verified — that belongs to `terraform-reviewer` for module contracts or `terraform-plan-blast-radius-agent` for change safety; this skill supplies the plan, not the judgment.
- The question is why a plan replaces or destroys a resource — that is blast-radius analysis, not verification design.
- The question is whether a control is satisfied and what evidence records it — route to `terraform-policy-evidence-agent`.
- The task requires executing tests to observe real behaviour — this skill designs verification and never runs it.

## Lean operating rules

- State what each artifact proves before recommending it: `validate` proves the configuration parses and is internally consistent, a plan proves what the engine intends given current state and provider versions, a plan-mode test proves module logic without touching infrastructure, and only an apply-mode test proves the resources can actually be created.
- Match verification to blast radius rather than to module size: a small module provisioning a stateful or internet-facing resource needs assertions on the properties that would cause the outage, while a large module composing already-verified pieces may need far less.
- Assert on the property that would cause the failure, not on the property that is easiest to assert. A test confirming a resource's name is well-formed proves nothing about whether it is publicly reachable.
- `command = apply` in a `run` block creates real infrastructure, so it belongs against an isolated non-production account with its own credentials — never against an account that also holds production resources, whatever the workspace is called.
- Mock providers make plan-mode tests possible for modules whose providers would otherwise require credentials, but a mocked provider proves the module's logic given assumed provider behaviour; it cannot prove the provider will actually accept the configuration.
- A test suite that only asserts on plan success is a parse check with extra steps; require at least one assertion per test that would fail if the resource were misconfigured in the way the module exists to prevent.
- Prefer a `validation` block at the module boundary over a test for any invariant about input: the validation rejects the bad value in every caller's plan, while the test only proves the module rejects it in the one case the test covers.
- Some properties are better asserted continuously than at change time; a `check` block observes an invariant after every apply without blocking, which suits properties that can drift after a correct change.
- Verify the test framework surface per engine rather than assuming it is shared, and name the engine in any recommendation that depends on a specific test feature.
- Never recommend deleting, skipping, or weakening a failing test to reach a passing state — a failing test is the control working, and the fix is the defect it found.

## References

Load these only when needed:

- [What Each Check Actually Proves](references/what-each-check-proves.md)
- [Proportionate Verification](references/proportionate-verification.md)
- [Official Sources](references/official-sources.md)

## Response minimum

- A verification plan naming each step, what it proves, and what it does not.
- The blast-radius rationale for the level of verification proposed.
- For any apply-mode test: the isolation requirement stated explicitly.
- Any invariant better placed in a `validation` block or a `check` block than in a test.
- The owning agent that must issue the adequacy verdict, since this skill does not.
