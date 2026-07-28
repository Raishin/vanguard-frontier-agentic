# High-Severity Failure Modes

The concrete production incidents each finding class maps to, for severity calibration.

- An unpickled cookie or cache entry yields remote code execution on the server that deserialized it.
- A `shell=True` command built from a filename field yields command execution and lateral movement.
- An SSRF against a URL parameter reaches the cloud instance-metadata endpoint and exfiltrates temporary credentials.
- A `zipfile.extractall` on an uploaded archive overwrites application code via a `../` member (zip-slip).
- A secret in a log line is replicated into the log pipeline and every downstream store and index.
- A bare `except:` around signature verification turns a forged request into an accepted one.
