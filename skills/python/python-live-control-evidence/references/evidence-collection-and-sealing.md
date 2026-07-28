# Evidence Collection And Sealing

Integrity hashing, control mapping as candidate support, and retention/redaction requirements for control evidence.

- Hashing evidence (e.g. via `hashlib`) seals the bytes for integrity — it proves the evidence was not altered after capture, not that the underlying observation is accurate.
- Evidence mapped to a control_id is candidate support for that control, not proof the control is effective or a framework requirement is satisfied.
- Retention, legal-hold, and redaction/tokenization govern what is stored and for how long, and must be applied before sealing.

## Sources

- https://csrc.nist.gov/pubs/sp/800/53/r5/upd1/final
- https://www.aicpa-cima.com/resources/landing/system-and-organization-controls-soc-suite-of-services
- https://docs.python.org/3/library/hashlib.html
