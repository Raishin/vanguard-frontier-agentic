# Framework Async Model And Request Lifecycle

How ASGI frameworks run sync vs async endpoints and the shutdown lifecycle.

- In FastAPI, a path operation declared with plain `def` is run in an external threadpool and awaited so it does not block the server, while an `async def` runs directly on the event loop; the documented guidance is to use `def` when calling blocking (non-await) libraries and `async def` when using awaitable libraries.
- Because an `async def` endpoint runs on the loop, any synchronous blocking call inside it (a blocking DB/HTTP client, `time.sleep`, heavy CPU) blocks every other request on that worker until it returns.
- Graceful shutdown on an ASGI server means stopping acceptance of new connections and draining in-flight requests within a bounded grace period on SIGTERM; work that must survive a crash belongs in a durable task queue, not an in-process background task tied to the request lifecycle.

## Sources

- https://fastapi.tiangolo.com/async/
- https://www.starlette.io/
