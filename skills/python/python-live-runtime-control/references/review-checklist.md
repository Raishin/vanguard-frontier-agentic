# Runtime-Control Review Checklist

The per-concern checklist applied to every runtime diagnostic read.

- Every diagnostic read uses only allowlisted sys/gc/faulthandler introspection — no mutating call is issued.
- Each captured snapshot is labeled with a freshness timestamp.
- No restart, kill, scale, or reconfigure action is performed by this agent.
- Leaked tasks, stuck workers, or abnormal memory growth are reported as findings, not silently remediated.
- A diagnostic read is clearly distinguished from a state-changing control action in the response.
- Findings needing a bounded restart or job operation are routed to the correct release/job operator rather than acted on directly.
