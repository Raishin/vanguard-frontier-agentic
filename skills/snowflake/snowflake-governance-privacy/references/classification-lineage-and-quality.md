# Classification, Lineage, and Data Quality

How to treat discovery, dependency, and quality signals as evidence with known limits rather than as verdicts. Load when coverage or impact is being asserted.

## Classification is a candidate generator

- Automatic classification identifies candidate sensitive columns with categories. It is a starting point that a data owner confirms — reporting its raw output as the sensitive inventory overstates certainty in both directions.
- It does not reliably reach sensitive values inside free text, JSON or variant payloads, concatenated identifiers, or values encoded in a domain-specific way. Name these blind spots in every coverage claim.
- Track three numbers, not one: candidates found, candidates confirmed by an owner, candidates still unreviewed. A programme reporting only the first is reporting activity.

## Lineage has edges

- Lineage answers 'what depends on this' within the operations and object types it covers. Its value in an impact analysis is entirely bounded by where it stops.
- The usual edges: external tables and files, data leaving through shares or exports, consumer-side transformations, and anything assembled outside Snowflake.
- An impact analysis that treats a lineage boundary as 'no further dependencies' is wrong in the direction that causes incidents. State the boundary explicitly and label beyond it `UNKNOWN`.
- Lineage is also how unprotected copies of protected data are found. A protection review with no lineage step will miss the copies.

## Data quality monitoring is only a control if someone acts

- Data metric functions assert something about a table on a schedule. The assertion, the schedule, and the action on violation are three separate design decisions and all three must be stated.
- Using data quality monitoring requires the appropriate privileges — including an account-level privilege to execute data metric functions and access to the results — so a monitoring design that ignores the grant model does not run.
- Report detection and action as two numbers. A violation rate with no corresponding action rate is a cost centre.
- Data quality is not data correctness. A table can pass every metric and still be semantically wrong for the business question — that boundary belongs to the analytics agent.

## Sources

Primary sources for the claims above. Each line states what that page establishes — a URL with no claim attached is a bibliography, not a reference.

- https://docs.snowflake.com/en/user-guide/classify-intro — What Snowflake's classification identifies and the categories it assigns — the basis for treating it as candidate generation
- https://docs.snowflake.com/en/user-guide/data-quality-intro — Data metric functions, their scheduling, and the privileges required to execute them and read results
- https://docs.snowflake.com/en/user-guide/ui-snowsight-data-lineage — The object types and operations lineage covers — and therefore where it stops
