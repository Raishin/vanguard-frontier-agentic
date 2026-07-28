# Review Workflow And Output Contract

The native-extension review workflow and the required output shape.

## Workflow

1. Identify the extension toolchain (raw C-API, Cython, PyO3/Rust), the target Python versions, and whether abi3/Py_LIMITED_API is claimed.
2. Trace every reference for owned-vs-borrowed correctness on every code path, including error paths.
3. Check every `PyObject_GetBuffer` call is paired with `PyBuffer_Release` and that contiguity/format assumptions are validated.
4. Check every C/Rust error path sets a Python exception and returns the correct sentinel, with no dangling error state.
5. Check GIL-release regions and free-threaded `Py_mod_gil` declarations, and record every claim needing compilation/execution to confirm.

## Evidence labels

Label every claim: confirmed (source provided) > inference (partial source) > assumption (source absent) > unknown. Never present an assumption as confirmed.

## Output contract

- A verdict (pass / pass-with-conditions / block) and the extension toolchain, target versions, and ABI assumed.
- Reference-ownership, buffer-protocol, exception-translation, and stable-ABI/thread-GIL findings.
- A severity-labelled finding list, each with an evidence-basis label, plus safe remediations and any crash/leak claim the user must confirm by compiling and running the extension.
