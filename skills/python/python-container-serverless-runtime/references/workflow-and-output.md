# Review Workflow And Output Contract

The container/serverless runtime review workflow and the required output shape.

## Workflow

1. Identify the base image, entrypoint form, process/worker model, and target platform (container vs serverless).
2. Check whether PID 1 handles SIGTERM (init/exec form) and whether the entrypoint form actually forwards signals.
3. Check the worker class and count fit the workload and platform, and that the master forwards SIGTERM for graceful drain.
4. Check the shutdown hook stops new work and drains in-flight requests within the grace period, and check filesystem writes target an explicit writable/tmpfs mount.
5. Check cold-start/import cost and image architecture/libc compatibility, and record every claim needing a real build/run to confirm.

## Evidence labels

Label every claim: confirmed (source provided) > inference (partial source) > assumption (source absent) > unknown. Never present an assumption as confirmed.

## Output contract

- A verdict (pass / pass-with-conditions / block) and the base image, entrypoint form, and worker/server assumed.
- Signal-handling/PID-1, worker-model/shutdown, filesystem/cold-start, and architecture-compatibility findings.
- A severity-labelled finding list, each with an evidence-basis label, plus safe remediations and any signal/shutdown-timing claim the user must confirm by building and running the container.
