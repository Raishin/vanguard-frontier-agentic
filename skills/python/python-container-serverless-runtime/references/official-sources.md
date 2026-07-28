# Official Sources

Primary Python, gunicorn, uvicorn, and Docker documentation for container/serverless runtime review.

Primary sources, verified 2026-07-26 against official upstream documentation and cross-checked via the Context7 MCP where a version-sensitive or security-sensitive claim was encoded. Blogs are used only for explanation, never as the sole source for normative behaviour.

## Source register

- https://docs.gunicorn.org/en/stable/signals.html
- https://www.uvicorn.org/deployment/
- https://docs.python.org/3/library/signal.html
- https://docs.docker.com/reference/dockerfile/

## Provenance notes

- docs.python.org/3/library/signal, docs.gunicorn.org (signals), www.uvicorn.org (deployment), and docs.docker.com (Dockerfile reference) are the authoritative upstreams for the PID 1/signal, worker-model, and container-runtime claims in this skill.
- Context7 NOT separately used — PID 1/signal semantics (docs.python.org signal), gunicorn signal handling, and Dockerfile ENTRYPOINT forms are quoted from those primary upstreams.

## Grounding rule

Documentation explains language, library, and platform behaviour in general. It does not prove the interpreter build, installed package versions, target configuration, or runtime the user actually ships. Treat any claim that depends on the user's specific versions or runtime as `assumption` until the source, lockfile, or build files confirm it.
