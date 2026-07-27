# Native-Extension Review Checklist

The per-concern checklist applied to every native-extension review.

- Ownership: every owned reference is `Py_DECREF`'d on every path (including errors); no borrowed reference is `Py_DECREF`'d or stored past its owner's lifetime.
- Buffers: every `PyObject_GetBuffer` is paired with `PyBuffer_Release`; contiguity/format is validated, not assumed.
- Exceptions: every C/Rust error path sets a Python exception and returns the correct sentinel (NULL/-1); no dangling error state.
- Stable ABI: a module claiming abi3/Py_LIMITED_API uses only the limited API surface.
- Threads: no Python-object access inside a GIL-released region; free-threaded builds declare `Py_mod_gil` and protect shared state.
- Wrappers: PyO3 functions return `PyResult`; Cython `nogil` blocks never touch Python objects.
