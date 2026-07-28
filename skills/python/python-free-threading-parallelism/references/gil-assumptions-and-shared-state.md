# GIL Assumptions And Shared State

How free-threaded builds invalidate GIL-dependent thread-safety assumptions.

- On a `Py_GIL_DISABLED` build the GIL no longer serializes bytecode, so previously-latent races on shared mutable state become active and need explicit locks.
- Read-modify-write on a shared counter and non-atomic container mutation are unsafe without synchronization.
- Reference counting is per-object with local and shared counts on free-threaded builds, so refcount contention shifts rather than disappears.

## Sources

- https://docs.python.org/3/howto/free-threading-python.html
- https://peps.python.org/pep-0703/
