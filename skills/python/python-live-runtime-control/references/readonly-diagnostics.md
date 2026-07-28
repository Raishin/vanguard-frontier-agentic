# Read-Only Runtime Diagnostics

Read-only interpreter/process/memory diagnostics and the freshness of a captured snapshot.

- The sys, gc, and faulthandler modules expose interpreter, process, thread, and memory state through read-only introspection, without mutating application state.
- A diagnostic read is not itself a control action — it observes state and must be treated separately from a restart, kill, scale, or reconfigure operation.
- A captured diagnostic snapshot has a freshness window and must be labeled with the time it was taken, since live state can change immediately after capture.

## Sources

- https://docs.python.org/3/library/sys.html
- https://docs.python.org/3/library/gc.html
- https://docs.python.org/3/library/faulthandler.html
