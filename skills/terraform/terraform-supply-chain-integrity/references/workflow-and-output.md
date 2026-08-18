# Workflow And Output

Assessment sequence and output contract for supply-chain review.

## Workflow

1. Establish which engine resolves the configuration, since default registry resolution differs and changes what an unqualified reference means.
2. Enumerate every provider with its explicit source address and verify each namespace against the provider's own documentation.
3. Assess the lock file: whether it is committed, which platforms it records hashes for, which platforms will actually run `init`, and which hash schemes are present.
4. Request and review the CLI configuration whenever a mirror or air-gapped path is involved; treat installation redirection as invisible until it is supplied.
5. Check whether `dev_overrides` can reach any non-developer environment.
6. Enumerate module sources including transitive ones, and flag every reference to a mutable branch or tag.
7. State what is actually verified at install time versus what is merely assumed, and name the exact remediation for each gap.

## Evidence labels

Label every claim: confirmed (artifact provided) > inference (partial artifact) > assumption (artifact absent) > unknown. Never present an assumption as confirmed, and never let a documentation-based claim stand in for live evidence of the user's actual infrastructure.

## Output contract

- A verdict (pass / pass-with-conditions / block) and which engine resolves the sources under review.
- Per-provider source address, namespace verification status, and version constraint.
- Lock file assessment naming the platforms covered and, explicitly, the platforms not covered.
- Any installation path that bypasses verification (mirror, `dev_overrides`, uncommitted lock), stated as an unverified path.
- Module source provenance including transitive sources, with mutable references flagged and the exact remediation named.
