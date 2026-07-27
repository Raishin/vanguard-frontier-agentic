# Review Workflow And Output Contract

The evidence-collection review workflow and the required output shape.

## Workflow

1. Collect the evidence and its quality dimensions (source, integrity, freshness, completeness, independence, control stage).
2. Hash the evidence (e.g. via `hashlib`) and attach a trusted timestamp.
3. Apply redaction/tokenization to sensitive or personal fields and honor retention/legal-hold requirements.
4. Seal the evidence to an approved, access-controlled, retention-managed destination.
5. Map the sealed evidence to the relevant control_ids as candidate support only, never as an effectiveness or compliance assertion.

## Evidence labels

Label every claim: confirmed (independently observed) > inference (partial) > assumption (self-reported / not observed) > unknown, AND tag the evidence quality dimensions. Never present an assumption as confirmed, or evidence as proof.

## Output contract

- A verdict (approved / blocked / needs-review) and the evidence/control-mapping particulars.
- Collection/hashing, sealing/retention, and control-mapping findings.
- Control results, the audit event emitted, and safe next actions/open questions including any effectiveness testing or independent assessment the user must obtain.
