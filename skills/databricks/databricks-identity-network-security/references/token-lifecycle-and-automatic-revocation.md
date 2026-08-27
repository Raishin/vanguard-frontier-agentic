# Token Lifecycle And Automatic Revocation

PAT and OAuth lifecycle, inactivity revocation, and token-scope inheritance.

- Personal access token (PAT) default max lifetime is 730 days; automatic revocation after 90 days of inactivity is a hard default and is not configurable.
- Account admins can enforce a shorter max lifetime for PAT across the account; this admin-enforced limit applies to all new tokens and overrides the 730-day default.
- New tokens receive inferred scopes; existing tokens show backfill_scopes. A newly-issued token is narrower in scope than legacy tokens, making it safer but potentially incompatible with old code expecting broader scopes.
- Automatic revocation after 90 days of inactivity applies to all PAT, regardless of whether the token is in active use elsewhere (e.g., a monthly batch job will encounter revocation).
- OAuth is Databricks' recommended authentication path over PAT because OAuth carries no inactivity revocation risk and is more suitable for service-account delegation.
