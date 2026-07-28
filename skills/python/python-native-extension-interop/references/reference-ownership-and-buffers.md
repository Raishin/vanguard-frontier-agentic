# Reference Ownership And Buffers

The C-API's owned/borrowed reference contract and buffer-protocol acquire/release discipline.

- The C-API documents each function as returning a new (owned) or borrowed reference; owned references must be `Py_DECREF`'d on every path, borrowed references must not.
- A mismatch (missing/extra DECREF, borrowed-after-free) is a memory-safety bug.
- `PyObject_GetBuffer` must be paired with `PyBuffer_Release`, and buffer format/contiguity must be validated.

## Sources

- https://docs.python.org/3/c-api/refcounting.html
- https://docs.python.org/3/c-api/buffer.html
