# Bundle Structure, Targets, And Resource Scoping

The exact shape of a production-ready bundle, target design, and resource-type constraints.

- A bundle is exactly one configuration file named `databricks.yml` in the bundle root; any other naming or location is not a bundle, regardless of content.
- The top-level keys in `databricks.yml` are `bundle`, `artifacts`, `resources`, `targets`, `workspace`, `variables`, `permissions`, `presets`, `sync`, `scripts`, `run_as`, `experimental` — additional keys are invalid.
- Deployment modes are declared per target and have distinct runtime consequences: development mode prepends `[dev ${workspace.current_user.short_name}]` to resource names and permits `--cluster-id` overrides; production mode enforces `development: false` for pipelines and forbids overrides.
- When the deploying identity differs from the `run_as` identity, only jobs and pipelines are supported resources; Model Serving endpoints error unconditionally under a non-self run-as identity.
- A bundle target must be narrowly scoped to a single environment (development, staging, production); a single target with multiple environment effects or a target that can deploy to multiple workspaces is poorly scoped.
- Bundle variables are resolved at deployment time from a precedence order (CLI flags, environment, overrides file, target mappings, defaults) and are never available at runtime — code cannot look up a variable value during a job or pipeline execution.
