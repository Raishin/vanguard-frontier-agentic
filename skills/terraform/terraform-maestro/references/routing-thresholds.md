# Routing Thresholds And Coordination Cost

The written thresholds that decide when a second, third, or fourth specialist is added — and the far more common case where one owns the change alone.

- Default mode is `single`. A board of eight specialists dispatched three at a time on every change costs more in coordination and reconciliation than the marginal finding is worth, and it destroys ownership: when three agents review a plan, none of them owns the verdict.
- Blast radius threshold — cross it when the plan contains a replace, a destroy, or a `-target` flag, or when a `lifecycle` block is edited. An additive plan (creates only, no replacements) does not cross it and does not need the specialist.
- State threshold — cross it when a `backend` or `cloud` block, a workspace, a lock, or a `state` subcommand is involved, or when a replaced resource stores data. Every plan reads state; that alone is not a signal.
- Supply-chain threshold — cross it on a change to a provider source address, a module `source`, a registry host, a mirror configuration, or `.terraform.lock.hcl`. A version bump within an already-trusted source is a compatibility question, not a provenance one, and routes to the compatibility specialist instead.
- Compatibility threshold — cross it on a core version constraint change, a provider major version change, or an engine change between Terraform and OpenTofu.
- Policy threshold — cross it only when the change touches public exposure, encryption, retention, logging, or an IAM/RBAC grant, or the repository declares policy-as-code. Tagging, naming, and formatting changes do not cross it, and routing them to policy trains reviewers to ignore policy output.
- Execution threshold — cross it when the pipeline, runner identity, remote execution backend, or plan-artifact handoff changes. A configuration change that merely happens to run in CI does not cross it.
- Cost is never a threshold on this board. A material spend change is handed to `finops-cloud-price-advisor-agent`, which owns live public pricing; duplicating that here would produce two answers with different price data and no owner.
- Four specialists is a hard ceiling, not a target. A change that appears to need five is a change that has not been decomposed — say so and ask for the split rather than dispatching five.
