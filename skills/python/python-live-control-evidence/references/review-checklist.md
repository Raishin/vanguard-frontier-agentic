# Control-Evidence Review Checklist

The per-concern checklist applied to every evidence-collection request.

- Quality dimensions: every piece of evidence records its source, integrity, freshness, completeness, independence, and control stage.
- Hashing: evidence bytes are hashed (e.g. `hashlib`) before sealing.
- Timestamp: a trusted timestamp is attached at sealing time.
- Destination: evidence is sealed only to an approved, access-controlled, retention-managed destination.
- Redaction: secrets are never persisted, and sensitive/personal fields are redacted or tokenized before sealing.
- Mapping: evidence maps to control_ids as candidate support only, never as an assertion the control is effective or a framework is satisfied.
