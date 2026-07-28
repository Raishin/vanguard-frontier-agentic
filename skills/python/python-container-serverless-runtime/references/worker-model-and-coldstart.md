# Worker Model And Cold Start

Matching the worker model to the workload, read-only-filesystem writes, and cold-start cost.

- The sync vs async worker class and worker count must match the workload and CPU/memory.
- A read-only root filesystem requires explicit writable mounts for any runtime writes.
- Cold-start latency and image size are driven by module-level import cost and dependency footprint.

## Sources

- https://www.uvicorn.org/deployment/
- https://docs.docker.com/reference/dockerfile/
