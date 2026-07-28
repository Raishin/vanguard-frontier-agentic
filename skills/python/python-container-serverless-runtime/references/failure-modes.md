# High-Severity Failure Modes

The production incidents each finding class maps to, for severity calibration.

- A container with no signal handling at PID 1 ignores SIGTERM entirely, and every deploy SIGKILLs in-flight requests once the grace period expires.
- A shell-form ENTRYPOINT swallows SIGTERM, so a rolling deploy drops requests that should have drained gracefully.
- A worker count sized without regard to CPU/memory causes the container to be OOM-killed or throttled under normal load.
- A service with no shutdown hook keeps serving new requests after SIGTERM and gets killed mid-response during every deploy.
- An arm64-built wheel shipped in an amd64 base image fails to import at container start, and the service never comes up.
