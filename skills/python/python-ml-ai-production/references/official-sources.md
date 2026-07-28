# Official Sources

Primary scikit-learn and Python documentation for the model-persistence and leakage claims.

Primary sources, verified 2026-07-26 against official upstream documentation and cross-checked via the Context7 MCP where a version-sensitive or security-sensitive claim was encoded. Blogs are used only for explanation, never as the sole source for normative behaviour.

## Source register

- https://scikit-learn.org/stable/model_persistence.html
- https://scikit-learn.org/stable/common_pitfalls.html
- https://docs.python.org/3/library/pickle.html
- https://numpy.org/doc/stable/reference/random/generator.html

## Provenance notes

- scikit-learn.org and docs.python.org are the authoritative upstreams for the model-persistence, pickle, and leakage-pitfall claims here; framework-specific serialization (PyTorch, TensorFlow) must be confirmed against their own documentation when the code uses them.
- Context7 NOT separately used — the model-persistence/pickle and leakage-pitfall claims are quoted from the scikit-learn documentation and docs.python.org (primary upstreams); framework-specific serialization (torch/tensorflow) must be confirmed against their own docs.

## Grounding rule

Documentation explains language, library, and platform behaviour in general. It does not prove the interpreter build, installed package versions, target configuration, or runtime the user actually ships. Treat any claim that depends on the user's specific versions or runtime as `assumption` until the source, lockfile, or build files confirm it.
