# Proportionate Verification

How to size verification to blast radius, and the isolation requirement that apply-mode testing imposes.

- Verification should be sized to what the change can break, not to how much code it contains: a three-line module provisioning a database deserves more verification than a large module composing already-verified components.
- The properties worth asserting are the ones that would cause the incident — reachability, encryption, retention, identity scope — rather than the ones that are convenient to express.
- Apply-mode tests create real infrastructure with real credentials, so they require an isolated account whose blast radius is bounded by construction; a workspace named non-production inside an account that also holds production resources provides naming, not isolation.
- Test infrastructure is destroyed at the end of a run, which means a failed or interrupted run can leave resources behind — the isolation requirement covers the failure case, not only the success case.
- A suite that asserts only that the plan succeeded will pass for every misconfiguration that is syntactically valid, which is nearly all of them; at least one assertion per test must fail when the module is misconfigured in the way it exists to prevent.
- When an existing suite passed but a failure still occurred, the gap is almost always an assertion that checked a property adjacent to the one that mattered — naming that adjacency is more useful than adding more tests.
- A failing test is the control working. Deleting, skipping, or weakening it to reach green removes the only signal that the defect exists, and the fix is always the defect rather than the test.
