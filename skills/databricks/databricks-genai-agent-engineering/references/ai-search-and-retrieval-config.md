# Databricks AI Search Index And Retrieval Configuration

Index variants, sync modes, query types, and pagination semantics.

- Databricks AI Search (formerly Databricks Vector Search) offers four index variants: Delta Sync with Databricks-managed embeddings (Databricks computes embeddings), Delta Sync with self-managed embeddings (caller provides vectors), Direct Vector Access (external vector source), and full-text search (BETA, keyword-only, storage-optimized endpoints only).
- Sync modes differ by variant: continuous sync updates the index on every Delta write (not supported on storage-optimized endpoints); triggered sync updates on-demand or on a schedule (required for full-text indexes on storage-optimized endpoints); manual sync (Direct Vector Access only, no automatic updates).
- Query types include `"ann"` (approximate nearest neighbor, default, vector-only), `"hybrid"` (vector + keyword using reciprocal rank fusion), and `"FULL_TEXT"` (BETA, keyword-only). Hybrid queries are more expensive than ANN but more robust to keyword-heavy requests.
- Query API via Python: `similarity_search(query_text, query_vector, columns, num_results, query_type, filters, reranker)`. REST API: `POST /api/2.0/vector-search/indexes/{index_name}/query` with pagination via `query-next-page` and `page_token`.
- Result pagination is capped at 1,000 results per query; unbounded result retrieval requires multiple queries or acceptance of the 1,000-result limit.
- The `filters` parameter enables predicates on metadata columns; `reranker` allows post-retrieval re-ranking by a separate model.
- Storage-optimized endpoints do not support continuous sync or ANN query types; they support triggered sync and full-text search only.
- Full-text search is BETA and storage-optimized endpoints only; production reliance on this feature requires explicit risk acknowledgment.
