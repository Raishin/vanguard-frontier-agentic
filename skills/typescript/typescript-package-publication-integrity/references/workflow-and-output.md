# Workflow And Output

Diagnostic sequence and output contract for publication-integrity review.

## Workflow

1. Identify the publish authority: trusted publishing/OIDC or token, and which CI provider runs the release.
2. Confirm provenance is attached and the release process states the consumer verification step.
3. Check the release-automation trust path for fork-triggerability and branch/tag protection.
4. Diff the packed file list against the working tree to catch anything unintended.
5. Review shipped declarations and source maps for internal-structure exposure, and audit lifecycle scripts.

## Evidence labels

Label every claim: confirmed (source provided) > inference (partial source) > assumption (source absent) > unknown. Never present an assumption as confirmed.

## Output contract

- A verdict (pass / pass-with-conditions / block) and the release/publish pipeline assumed.
- Publish-identity, provenance, release-automation-trust, tarball-contents, and declaration/source-map findings, each with an evidence-basis label.
- A severity-labelled finding list plus safe next actions and open questions, including anything the sigstore board, security board, or `package-governance-agent` must confirm.
