# Review Workflow And Output Contract

The free-threading review workflow and the required output shape.

## Workflow

1. Identify the interpreter build (standard vs free-threaded `t` suffix), every native dependency, and every piece of shared mutable state.
2. Check shared mutable state for GIL-dependent thread-safety assumptions and require explicit synchronization where the assumption no longer holds.
3. Check every native dependency for `Py_mod_gil`/`PyUnstable_Module_SetGIL` declaration and flag any that would silently re-enable the GIL.
4. Check extension code that iterates shared containers for critical sections, and check for unguarded read-modify-write or container mutation in application code.
5. Establish the workload profile (CPU-bound vs I/O-bound, contention) and deliver an adopt / pilot / defer verdict tied to the evidence.

## Evidence labels

Label every claim: confirmed (source provided) > inference (partial source) > assumption (source absent) > unknown. Never present an assumption as confirmed.

## Output contract

- A verdict (pass / pass-with-conditions / block) and the interpreter build, extensions, and workload profile assumed.
- GIL-assumption/shared-state, C-extension-compatibility, synchronization/critical-section, and adoption-readiness findings.
- A severity-labelled finding list, each with an evidence-basis label, plus a safe adopt/pilot/defer recommendation and any race or compatibility claim the user must confirm on a real free-threaded build.
