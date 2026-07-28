# Index Trust And Dependency Confusion

Why mixing indexes is dangerous and how to configure a trusted resolution.

- With multiple configured indexes, pip considers candidates from all of them and selects the highest compatible version; it does not treat a private index as higher-priority than the public one.
- This means a public package registered under an internal package's name with a higher version number can be resolved and installed instead of the internal package (dependency confusion).
- The mitigations are to resolve from a single trusted index (a private mirror that proxies the public one under one namespace), to pin exact versions with hashes, and to reserve/namespace internal package names — not to rely on `--extra-index-url` ordering.

## Sources

- https://pip.pypa.io/en/stable/cli/pip_install/
- https://packaging.python.org/en/latest/guides/hosting-your-own-index/
