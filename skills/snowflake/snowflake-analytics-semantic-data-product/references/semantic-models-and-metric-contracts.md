# Semantic Models and Metric Contracts

How to specify a metric so it survives reimplementation, and what a semantic model must resolve before it drives natural-language querying. Load when defining metrics or building a semantic view.

## The metric contract

- **Definition** in business language, written by or agreed with the owner — not derived from the SQL after the fact.
- **Grain** — the level at which the metric is meaningful, and what aggregating above it means.
- **Filters** — every inclusion and exclusion, including the ones that feel obvious. 'Excluding returns' is the difference between two revenue definitions.
- **Time basis** — event time, effective time, or load time. Three different numbers, and the one nobody states is the one that causes the dispute.
- **Units and currency** — including the conversion source and the rate date, where applicable.
- **Null and unknown handling** — whether unknown-category rows are excluded, bucketed, or counted.
- **Restatement policy** — what happens to published history when a definition or a source changes.
- **Owner** — a named person or function. A metric with no owner has no definition, only an implementation.
- **Test** — a known input and expected output, or an independent reconciliation. Without it the metric cannot be safely changed later.

## Semantic views as a specification

- A semantic view declares tables with keys, relationships between them, dimensions, and metrics — including derived metrics defined in terms of other metrics through a relationship. The declaration is a contract the data must actually satisfy.
- Relationships are the load-bearing part. A relationship whose key is not unique produces fan-out inside every metric that traverses it, and the semantic layer makes that invisible to the consumer.
- Derived metrics compose. An error in a base metric propagates into every derived metric silently, so base metrics need the strongest tests.
- A metric defined in a semantic view is the place to centralize a definition — which is only an advantage if the definition was agreed first. Centralizing a disputed definition makes the dispute harder to see, not easier.

## The natural-language boundary

- A semantic model exposed to natural-language querying cannot ask which of two similar measures the user meant. Every ambiguity in the model becomes a confidently generated wrong answer.
- Hunt for: measures with similar names and different filters; dimensions with overlapping meanings; metrics whose grain is implicit; synonyms that map to different objects; and time dimensions with more than one plausible basis.
- State the questions the model is *not* able to answer correctly, and put them in the model's documentation. A boundary that is written down is a boundary that can be tested.
- Correctness of the model is this agent's domain; whether exposing it is safe — what data it reaches, under whose identity, with what guardrails — is the Cortex AI security governor's. Both reviews are required before exposure, and neither substitutes for the other.

## Sources

Primary sources for the claims above. Each line states what that page establishes — a URL with no claim attached is a bibliography, not a reference.

- https://docs.snowflake.com/en/user-guide/views-semantic/sql — The semantic view grammar — tables with primary keys, relationships, dimensions, metrics, and derived metrics using the USING clause
- https://docs.snowflake.com/en/user-guide/snowflake-cortex/cortex-analyst — That Cortex Analyst answers natural-language questions from a semantic model, making model ambiguity a correctness risk at query time
