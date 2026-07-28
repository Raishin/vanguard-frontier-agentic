# PID 1, Signals, And Shutdown

PID 1 signal semantics, entrypoint forms, and gunicorn's graceful-shutdown handling.

- PID 1 has no default signal dispositions and must explicitly handle SIGTERM, or use an init (`tini`/exec form), else the orchestrator SIGKILLs after the grace period.
- The exec/JSON-array `ENTRYPOINT` delivers signals to the app while the shell form does not forward them.
- gunicorn treats SIGTERM as a graceful shutdown and the master forwards signals to workers.

## Sources

- https://docs.python.org/3/library/signal.html
- https://docs.gunicorn.org/en/stable/signals.html
