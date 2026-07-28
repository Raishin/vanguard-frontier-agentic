# Unsafe Deserialization And Dynamic Execution

Why pickle/yaml.load/eval/exec are code-execution sinks and what a sound control looks like.

- `pickle` (and `marshal`/`shelve`, which build on it) can execute arbitrary code during `load`/`loads` because the opcode stream can invoke callables; the official documentation warns it must never be unpickled from an untrusted or unauthenticated source.
- `yaml.load` with the default or `FullLoader` can construct arbitrary Python objects from tags; only `yaml.safe_load` (or `SafeLoader`) restricts construction to plain data types.
- `eval`/`exec`/`compile` evaluate arbitrary expressions or statements; a character or keyword blocklist is bypassable and is not a control — the only sound fix is to remove dynamic execution or replace it with a purpose-built parser over an explicit allowlist.

## Sources

- https://docs.python.org/3/library/pickle.html
- https://owasp.org/www-community/vulnerabilities/Deserialization_of_untrusted_data
- https://pyyaml.org/wiki/PyYAMLDocumentation
