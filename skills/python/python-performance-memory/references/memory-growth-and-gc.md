# Memory Growth And Garbage Collection

Localizing memory growth with tracemalloc and CPython's reference-counting plus cyclic collector.

- tracemalloc traces allocation origins and diffs snapshots to localize growth.
- CPython uses reference counting plus a cyclic garbage collector; reference cycles are reclaimed by gc, and `__del__` on a cycle can delay collection.
- Unbounded caches/globals/closures are the common leak, evidenced by a growing tracemalloc snapshot.

## Sources

- https://docs.python.org/3/library/tracemalloc.html
- https://docs.python.org/3/library/gc.html
