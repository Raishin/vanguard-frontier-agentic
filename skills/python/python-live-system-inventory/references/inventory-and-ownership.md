# Inventory And Ownership

Read-only distribution discovery and the NIST CM-8 asset-inventory fields required per asset.

- importlib.metadata enumerates installed Python distributions read-only, without executing package code.
- An asset register needs owner, environment, deployment revision, service identity, and criticality per NIST SP 800-53 CM-8 (component inventory).
- An unowned or orphaned business-critical asset is a key-person and control risk that CM-8 inventory practices are designed to surface.

## Sources

- https://docs.python.org/3/library/importlib.metadata.html
- https://csrc.nist.gov/pubs/sp/800/53/r5/upd1/final
