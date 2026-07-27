# Container-And-Serverless-Runtime Review Checklist

The per-concern checklist applied to every container/serverless runtime review.

- PID 1: the entrypoint uses an init (`tini`) or exec form with explicit SIGTERM handling; the process is not left as a signal-blind PID 1.
- Entrypoint form: `ENTRYPOINT`/`CMD` use the exec/JSON-array form, not the shell form, on every long-running service.
- Workers: the worker class and count match the workload and CPU/memory, and the master forwards SIGTERM for graceful drain.
- Shutdown: the app stops accepting new work on SIGTERM and drains in-flight requests within the grace period.
- Filesystem: writes target an explicit writable/tmpfs mount; nothing assumes a writable working directory or `/tmp` under a read-only root filesystem.
- Cold start / arch: module-level imports and dependency footprint are checked for cold-start cost, and the image architecture/libc matches the target runtime.
