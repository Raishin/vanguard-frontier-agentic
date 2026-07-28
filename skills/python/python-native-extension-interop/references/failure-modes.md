# High-Severity Failure Modes

The production incidents each finding class maps to, for severity calibration.

- A missing `Py_DECREF` on an error path leaks memory that only surfaces as an OOM days into a long-running production process.
- An extra `Py_DECREF` on what was actually a borrowed reference frees an object still in use elsewhere, corrupting memory or crashing intermittently.
- A `Py_buffer` acquired and never released via `PyBuffer_Release` pins memory that should have been freed, growing resident memory over the process lifetime.
- A C function that fails without setting a Python exception returns NULL, and the caller crashes on a nonsensical value instead of seeing the real error.
- A module built for the free-threaded interpreter but not declaring `Py_mod_gil` support silently runs with the GIL re-enabled, and the expected speedup never appears.
