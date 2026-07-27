# Stable ABI And Exception Translation

The limited-API/stable-ABI trade-off, boundary exception translation, and GIL/free-threaded rules.

- The Limited API / stable ABI (`Py_LIMITED_API`, abi3 wheels) trades API surface for cross-version wheel portability.
- A boundary function must translate errors into a Python exception and the correct error sentinel, never leave a dangling error indicator.
- GIL-released regions and free-threaded builds impose additional rules (no Python-object access without the GIL; declare `Py_mod_gil`).

## Sources

- https://docs.python.org/3/c-api/stable.html
- https://docs.python.org/3/c-api/intro.html
