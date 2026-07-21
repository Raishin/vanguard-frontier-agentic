# Spring WebFlux Coroutine Handlers

What WebFlux actually supports for suspend handlers, and where the real readiness-relevant boundary lives.

- Spring WebFlux has supported `suspend` function handlers on `@RestController` methods since Spring Framework 5.2 — a claim that WebFlux does not support coroutines or requires a reactive-type return value for suspend handlers is stale and should be flagged as a documentation defect, not treated as a real constraint.
- The genuine hazard in a Kotlin-on-Spring suspend handler is that imperative `@Transactional` binds its transaction to a ThreadLocal; when the annotated method body suspends across a dispatcher switch, the ThreadLocal-bound context can be lost, silently splitting the transaction — this is a coroutine-context defect whose root cause belongs to coroutine-reliability review, not to a production-readiness verdict.
- This agent's own scope in a suspend-handler review is limited to the readiness surface around it — health/actuator reachability through the coroutine handler and centralized exception mapping equivalent to StatusPages — not the coroutine-context correctness itself.

## Sources

- https://docs.spring.io/spring-framework/reference/languages/kotlin/coroutines.html
