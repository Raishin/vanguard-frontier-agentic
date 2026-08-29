# Masks, Filters, And ABAC UDF Cost

Row and column mask implementation, query-engine cost implications, ABAC cycle prevention.

- Row filters and column masks are implemented as SQL UDFs and are evaluated by the query engine at query time; the engine prioritises security over optimisation when protecting masked or filtered values, so query cost cannot be guaranteed under active policies.
- A row filter returns FALSE to exclude a row; a column mask applies one mask per column and transforms the value in the result set (not in storage). Neither operates on historical versions.
- Masks and filters cannot reference tables carrying active ABAC policies (cycle prevention). A column mask cannot reference another masked column on the same table (no mask chaining).
- Deterministic UDFs (marked DETERMINISTIC in the DDL) allow the query engine to optimise masked columns; non-deterministic UDFs prevent optimisation and incur full-table scan cost.
- String operations (SUBSTR, REGEX_REPLACE) are generally cheaper than full-table regex evaluation for masking PII; prefer string operations over regex when masking credit cards, SSNs, or phone numbers.
