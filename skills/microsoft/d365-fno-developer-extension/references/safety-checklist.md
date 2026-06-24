# Safety checklist

Use this reference before any recommendation involving production deployable package deployment, schema changes, release authorization, or upgrade-safety sign-off in Dynamics 365 Finance & Operations.

## Non-negotiables

- Never ask users to paste credentials, LCS project IDs, Azure DevOps PATs, environment URLs, tenant IDs, or source code containing secrets into chat.
- Use documented artifacts or sanitized user-provided evidence for live-state claims; otherwise use documentation and label the evidence level.
- Do not invent CoC correctness results, build pipeline outcomes, sandbox deployment results, test pass rates, or release manager approvals.
- Require explicit human approval before recommending production deployable package deployment or schema changes.
- All X++ and pipeline syntax guidance is advisory and static-review only; verify against current Microsoft Learn documentation before applying.
- Use current official Microsoft Learn documentation for Finance & Operations extensibility, deployable package creation, and ALM guidance.
- Keep recommendations least-change, reversible, and scoped to the extension or deployment in question.
- Production deployable package deployment and schema changes are live-guard gated. Always escalate to the implementation lead and named release manager before execution.

## Stress checks

- Does every Chain of Command wrapper method make an unconditional call to `next` (unless the method is explicitly marked `[Replaceable]`)?
- Are there any over-layering violations that modify base application objects directly rather than using extension points?
- Does the extension model have clean, non-circular dependencies on the base model packages it references?
- Has the deployable package been created by the Azure DevOps build pipeline (not from a developer machine) and uploaded to the LCS asset library?
- Has the package been deployed to a sandbox (UAT) environment and validated before production deployment is requested?
- Have SysTest unit tests and RSAT business process tests been executed and passed against the sandbox deployment?
- Is there a rollback plan with a named owner, rollback trigger criteria, and rollback validation steps?
- Has the release manager signed off on sandbox validation results and test evidence?
- Have all deprecated API usages and upgrade-risk patterns been identified and flagged for the next One Version service update cycle?

## Evidence labels

Use `live evidence`, `documented artifact`, `user-provided evidence`, `documentation-based`, or `inference`. Documentation alone never proves the user's actual extension correctness, package validation state, test coverage, or sandbox sign-off status.

## Live-guard gate

The following actions require explicit human confirmation and are out of scope for automated execution:

- Deploying a deployable package to the production environment
- Executing schema changes (new fields, table structure changes) in the production environment
- Marking a deployable package as a release candidate in the LCS asset library
- Authorizing a production service request with the Dynamics Service Engineering team
- Disabling or bypassing automated test gates in the release pipeline
- Approving production deployment without sandbox validation evidence and test results
- Signing off on upgrade safety or One Version compatibility on behalf of the implementation lead
