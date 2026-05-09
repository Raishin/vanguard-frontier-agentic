# Workflow and output contract

Use this reference only when executing the full live OSS bucket ACL and policy mutation gate.

## OSS bucket guard areas to check

- Bucket ACL: current setting (private/public-read/public-read-write); intended change; data sensitivity assessment
- Bucket policy: current policy document; intended change; affected principals and resources
- Cross-border replication: source region (CN-* or international); destination region; DSL Article 31 assessment status
- Blast radius: object count and sensitivity; dependent services and users; rollback baseline (prior ACL and policy captured)
- Operator authorization: identity confirmed; explicit written confirmation covering all required statements

## Safe workflow

1. **Frame scope** — confirm bucket identity, current ACL/policy, and the specific mutation requested
2. **Collect evidence** — capture current state before any change; label: `live evidence`, `user-provided`, `documentation-based`, `inference`
3. **Stress-test** — what data is exposed? what is the CN-* replication status? what is the rollback path?
4. **Gate** — require explicit written confirmation from the operator covering all required confirmation statements
5. **Execute and verify** — execute the minimum scoped change; confirm new state; monitor access logs

## Output contract

Return this structure:

```markdown
# Alibaba Cloud Live OSS Bucket Policy Guard: <scope>
## Bucket identity and current state
## Data sensitivity and blast radius assessment
## Cross-border replication check (if applicable)
## Confirmation received
## Execution result
## Post-change verification
```

Each section must include an evidence level label. Do not proceed past any step without the operator's explicit written confirmation.
