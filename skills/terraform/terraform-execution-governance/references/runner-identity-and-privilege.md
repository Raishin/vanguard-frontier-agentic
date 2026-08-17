# Runner Identity And Privilege

Why the pipeline principal is a production identity, and how the plan stage becomes an execution path.

- The identity that applies infrastructure changes can generally recreate or destroy everything the configuration manages, which makes it the most privileged automated principal in most estates and a production identity rather than build infrastructure.
- Plan and apply require different privileges: plan needs read access to describe current state, while only apply needs mutating permission. Running both with one identity means the read-only stage carries write authority it never uses and cannot be prevented from using.
- A plan is not an inert operation — providers, modules, and external data sources execute during it — so a plan stage holding mutating credentials is an arbitrary-code-execution path with write access, reachable by anything that can cause a plan to run.
- Static long-lived cloud credentials stored in CI cannot be attributed to a run, expire only when someone rotates them, and appear identically in every audit trail; short-lived credentials issued per run through workload identity remove all three problems at once.
- Permission scope should match what the configuration manages rather than what the team owns; a runner permitted across an entire account applies changes to resources no configuration in that repository has ever described.
- A self-hosted runner shared between infrastructure and application pipelines extends the infrastructure identity to every other job on that host, so the trust boundary is the host rather than the workflow.
- Runner-side CLI configuration lives outside the repository and can redirect provider installation or supply credentials with no diff anywhere; an execution path certified without the runner image and its configuration has been assessed on incomplete evidence.
