# Dispatchers, Blocking, And Context Propagation

Dispatcher selection, blocking-call confinement, and what survives a dispatcher switch.

- `Dispatchers.Default` is a CPU-core-sized pool for CPU-bound work; `Dispatchers.IO` is for blocking I/O; `Main` is the UI thread — blocking work on Default or Main causes starvation or ANR.
- `runBlocking` blocks the calling thread until completion and is intended only as a main/test bridge, never in suspend functions or request handlers.
- ThreadLocal state (SLF4J MDC, security principal, imperative `@Transactional` context) is bound to the thread and is lost across a `withContext` dispatcher switch unless bridged with `ThreadLocal.asContextElement`, `MDCContext`, or the OpenTelemetry `Context.asContextElement` element.
- Trace context must be captured before an async dispatch and attached to the coroutine context so spans keep their parent across suspension.

## Sources

- https://kotlinlang.org/docs/coroutine-context-and-dispatchers.html
- https://kotlin.github.io/kotlinx.coroutines/kotlinx-coroutines-slf4j/
