# Dashboard Limits, Permissions, And Data Security

Dashboard rendering limits, 'Individual data' versus 'Share data' permission model and its security consequences, and the effect of this setting on row-level security.

- Dashboard limits: 15 pages, 100 datasets, 100 widgets per page, 10,000 rows for most charts and 100,000 for table visualizations, 100,000 distinct filter values, 9 MB email attachment cap. Exceeding row-rendering caps engages backend processing and causes slowdown.
- 'Individual data' permission: each query runs per viewer under the viewer's identity; Unity Catalog row filters and column masks apply per user.
- 'Share data' permission: the query runs once under the publisher's identity; row filters and column masks are COMPLETELY BYPASSED, and all viewers see unfiltered data under the publisher's credentials. This is a critical security boundary.
- Switching from 'Individual data' to 'Share data' flips the security model and removes all per-viewer row-level security enforcement. This requires explicit approval and security review.
- Dashboard caching provides a best-effort 24-hour cache on initial load; stale values can be shown after underlying data changes. Disabling the cache or reducing the window is needed for real-time dashboards.
- Cross-geo Genie agent use requires admin approval for data-residency and compliance.

## Sources

- https://docs.databricks.com/aws/en/dashboards/limits
- https://docs.databricks.com/aws/en/ai-bi/admin
- https://docs.databricks.com/aws/en/genie-agents/monitor
