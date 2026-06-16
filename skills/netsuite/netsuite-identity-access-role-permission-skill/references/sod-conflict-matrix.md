# Sod Conflict Matrix

Reference conflict pairs for common NetSuite financial and administrative function combinations

Scope: Role structure, permission levels, and SoD conflict detection in NetSuite. Covers standard role baselines, custom role derivation, permission catalog lookup against the 684-code SDF catalog, and multi-role SoD conflict matrices.

- Standard role review: baseline permissions, intended profile, and principle of least privilege alignment (evidence rows 7a, 7b, 7c)
- Custom role derivation: confirm roles are copies of standard roles, not Administrator or blank; validate permkey/permlevel XML in SDF customrole objects
- Permission catalog lookup: resolve permission codes (ADMI_, LIST_, REGT_, REPO_, TRAN_ prefixes) against the upstream netsuite-sdf-roles-and-permissions catalog of 684 verified codes
- Segregation-of-Duties analysis: flag roles that combine conflicting functions (e.g., AP entry + AP approval, GL journal + period close)
- Integration role review: validate script run-as configurations and integration-record role assignments for least-privilege alignment
- 2FA requirement mapping: identify which permissions and roles trigger mandatory 2FA per evidence rows 5a–5d; flag roles missing the designation
