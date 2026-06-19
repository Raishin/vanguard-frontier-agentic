# Official sources

Use this reference only when you need source grounding for Dynamics 365 Finance & Operations extensibility, X++ Chain of Command, extension model design, deployable package creation, or ALM guidance.

## Microsoft Learn documentation

Use these as starting points, not as proof of the user's live extension quality, package state, or deployment readiness:

- https://learn.microsoft.com/dynamics365/fin-ops-core/dev-itpro/extensibility/method-wrapping-coc
- https://learn.microsoft.com/dynamics365/fin-ops-core/dev-itpro/dev-tools/pipeline-create-deployable-package
- https://learn.microsoft.com/dynamics365/fin-ops-core/dev-itpro/dev-tools/hosted-build-automation
- https://learn.microsoft.com/dynamics365/guidance/implementation-guide/application-lifecycle-management-product
- https://learn.microsoft.com/power-platform/admin/unified-experience/tutorial-release-pipeline-azure-devops
- https://learn.microsoft.com/dynamics365/fin-ops-core/dev-itpro/extensibility/extensibility-changes-73
- https://learn.microsoft.com/power-platform/developer/unified-experience/finance-operations-pipelines
- https://learn.microsoft.com/training/modules/develop-object-oriented-code-finance-operations/

## Grounding rule

Official documentation explains Dynamics 365 Finance & Operations extensibility mechanics, Chain of Command behavior, deployable package creation steps, and ALM best practices. It does not prove the user's actual extension correctness, package validation state, test coverage, or sandbox sign-off status. Prefer documented artifacts (code review records, build pipeline results, sandbox deployment logs, test execution reports, release manager sign-off) over inference.

All X++ and pipeline syntax shown here is advisory and for static review only. Verify against current Microsoft Learn documentation before applying in a real project, as syntax and tooling evolve with One Version updates.

## Service facts (verified 2026-06-17)

X++ extension patterns:
- **Extension-only**: Dynamics 365 Finance & Operations requires extension-based customization. Over-layering (directly modifying base application objects) is not supported and creates upgrade blockers. Use extension classes, table extensions, form extensions, and enum extensions instead.
- **Chain of Command (CoC)**: Allows wrapping public and protected methods on classes, tables, data entities, and forms via extension classes. The `[ExtensionOf(...)]` attribute binds the extension. Wrapper methods must always call `next` to invoke the next method in the chain; failure to call `next` is a defect that breaks base application behavior.
- **Extension class rules**: Extension classes must be `final`. They must belong to a package that references the model containing the augmented class. CoC is available from Platform update 9 onward; both the extension and the base package must be compiled on a compatible platform version.
- **Wrappable restriction**: Methods marked `[Wrappable(false)]` cannot be wrapped via CoC. Methods marked `[Replaceable]` allow conditional `next` calls (the compiler does not enforce unconditional `next` for Replaceable methods).

Extension model and package design:
- **Extension model**: A model is a logical grouping of elements (X++ source, metadata, resources) that belongs to a package. Extension models must reference the base model they extend without circular dependencies.
- **Deployable package**: A zip file containing compiled binaries and metadata, created by the Azure DevOps build pipeline using the Dynamics 365 Finance and Operations Tools extension. Production deployment requires the package to be uploaded to the Lifecycle Services asset library and marked as a release candidate.
- **Build pipeline**: Microsoft-hosted agents with the Dynamics 365 Finance and Operations Tools Azure DevOps extension can compile X++ and create deployable packages without a dedicated build virtual machine. The pipeline must install NuGet packages, update model versions, build the solution, and create and publish the deployable package.

ALM and deployment:
- **Branch strategy**: Each developer uses an isolated development environment; code is checked into Azure DevOps source control and built on a build agent. Build definitions should be created per branch with appropriate triggers (continuous integration, gated check-in, or scheduled).
- **Lifecycle Services (LCS)**: The asset library in LCS holds deployable packages. A package must be marked as a release candidate before it can be deployed to production. Production deployment is a self-service action requiring service request scheduling with the Dynamics Service Engineering team.
- **Sandbox-first**: All packages must be deployed to and validated in a sandbox (UAT) environment before production deployment. Automated regression testing (RSAT) should be executed after sandbox deployment.
- **One Version**: All customers are on the same service update track. Customizations that reference deprecated APIs, use over-layering, or have hard-coded version assumptions are upgrade risks on each service update cycle.

Certification anchor:
- MB-500 (Finance and Operations Apps Developer) — verify current exam status and objectives on Microsoft Learn before citing. (E4: verify before citing.)
