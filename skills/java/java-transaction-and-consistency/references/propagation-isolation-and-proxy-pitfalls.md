# Propagation, Isolation, and Proxy Pitfalls

> Static review only. Every conclusion below requires the actual `@Transactional` attributes *and* the call site (who invokes the method, and from where) as evidence — a propagation or rollback claim without both is `inference (partial source)` or `assumption (source absent)`. Sources: Spring Framework Reference — Data Access / Transaction Management (declarative transaction management, propagation, and rollback rules); Jakarta Transactions (JTA) specification for the underlying resource-transaction model Spring's `PlatformTransactionManager` abstracts over. Spring's default propagation and rollback behavior has been stable across recent Spring Framework major lines, but always confirm annotation defaults against the specific Spring Framework version in use before treating a version-specific detail as settled — this reference does not hardcode a version number.

## Why propagation is the first thing to check

`@Transactional` attributes on a method only fully apply when that method actually **starts** the transaction. Spring's declarative transaction management is proxy-based: it wraps the bean in an AOP proxy that opens/joins/suspends a transaction *before* the target method runs, and commits/rolls back *after* it returns. Everything below follows from that one mechanical fact.

## Propagation semantics that get silently overridden

| Propagation | Behavior when called with no existing transaction | Behavior when called from within an existing transaction |
|---|---|---|
| `REQUIRED` (default) | Starts a new transaction | **Joins** the caller's transaction — the method's own `isolation`/`timeout` are silently ignored; `readOnly` is honored only if the caller also declared it |
| `REQUIRES_NEW` | Starts a new transaction | **Suspends** the caller's transaction, runs in a brand-new one, then resumes the caller's — the inner transaction can commit independently of the outer |
| `MANDATORY` | Throws `IllegalTransactionStateException` | Joins the caller's transaction |
| `SUPPORTS` | Runs non-transactionally | Joins the caller's transaction |
| `NOT_SUPPORTED` | Runs non-transactionally | Suspends the caller's transaction and runs non-transactionally |
| `NEVER` | Runs non-transactionally | Throws `IllegalTransactionStateException` |
| `NESTED` | Starts a new transaction | Starts a nested transaction using a savepoint (JDBC `RESOURCE_LOCAL` only; not all `PlatformTransactionManager` implementations support it) |

**The finding that matters most:** a `REQUIRED` method that declares `isolation = SERIALIZABLE`, `timeout = 5`, or similar, and is *also* called from another `@Transactional` method elsewhere in the codebase, will silently run at whatever isolation/timeout the outer transaction already established. If the call graph shows this method invoked both standalone and from inside another transaction, the inner attributes are unreliable by construction — flag it, don't just check the annotation in isolation.

`REQUIRES_NEW` avoids that problem but introduces a different one: the inner transaction can commit even if the outer later rolls back. That is sometimes exactly what's wanted (e.g., an audit-log write that must survive the caller's rollback) and sometimes a bug (a "just to be safe" `REQUIRES_NEW` that breaks atomicity the caller assumed). The evidence needed to tell them apart is *why* the isolation was chosen — ask if it isn't stated.

## Proxy self-invocation: the classic silent bypass

```java
@Service
public class OrderService {

    public void placeOrder(Order order) {
        // BAD: calling another @Transactional method on `this` goes
        // straight to the method body — it never passes through the
        // Spring-generated proxy, so @Transactional on saveOrder() below
        // is a complete no-op here: no transaction is started, no
        // rollback rule applies, no propagation semantics fire.
        this.saveOrder(order);
    }

    @Transactional
    public void saveOrder(Order order) {
        repository.save(order);
    }
}
```

This happens because Spring's default AOP is proxy-based (JDK dynamic proxies for interfaces, CGLIB subclassing for concrete classes): the proxy only intercepts calls that arrive from *outside* the bean. A call from one method to another on the same instance (`this.method()`, or simply calling a sibling method by its plain name) never goes through the proxy.

Correct alternatives, in order of preference for most codebases:

```java
@Service
public class OrderService {

    // 1) Split the transactional method into a separate bean and
    //    inject it — the cleanest fix, no proxy tricks required.
    private final OrderWriter orderWriter;

    public void placeOrder(Order order) {
        orderWriter.saveOrder(order); // goes through orderWriter's proxy
    }
}

@Component
class OrderWriter {
    @Transactional
    void saveOrder(Order order) { repository.save(order); }
}
```

Self-injection (`@Autowired private OrderService self;` then `self.saveOrder(order)`) or `AopContext.currentProxy()` (requires `exposeProxy = true` on the `@EnableAspectJAutoProxy`) both work but are easy to regress if a later edit reverts to `this.`; extraction to a separate bean is the more durable fix. Flag self-invocation regardless of which fix the codebase already uses elsewhere — check every `@Transactional` call site, not just the constructor-injected ones.

## rollbackFor: checked exceptions do not roll back by default

```java
@Transactional // BAD if InventoryException is checked and unhandled here
public void reserveStock(Order order) throws InventoryException {
    inventory.decrement(order.getSku(), order.getQty()); // throws checked InventoryException
    ledger.record(order); // never runs if the exception above propagates —
                           // but the transaction still COMMITS on the way out
}
```

Spring's default rollback rule is: roll back on unchecked `RuntimeException` and `Error`; **commit** on a checked exception unless told otherwise. This is the exact inverse of what most developers expect from "an exception happened, surely it rolled back." The fix is explicit:

```java
@Transactional(rollbackFor = InventoryException.class)
public void reserveStock(Order order) throws InventoryException { ... }
```

Flag every checked exception declared or thrown from a `@Transactional` method that lacks a matching `rollbackFor` (or a global convention enforced elsewhere, such as wrapping checked exceptions in an unchecked wrapper before they leave the method — verify that convention actually holds at the site under review rather than assuming it).

## readOnly: an optimization hint, not just documentation

`readOnly = true` is a hint to the underlying resource: Hibernate can skip dirty-checking/flush, and some `DataSource`/driver/pool combinations route read-only connections differently (e.g., to a read replica, or reject a write outright). **This behavior is infrastructure-dependent** — Spring and JPA do not guarantee a specific outcome for a write inside a `readOnly = true` transaction; it can silently no-op, silently succeed, or throw, depending on the driver and pool configuration in use. Treat a write inside a `readOnly = true` method as a defect regardless of which of those three outcomes the current stack happens to produce, and mark the exact runtime consequence as `unknown` unless the driver/pool configuration was provided.

## Isolation levels: name the anomaly, and mind vendor differences

| Level | Prevents |
|---|---|
| `READ_UNCOMMITTED` | Nothing extra beyond `DEFAULT`'s DB baseline |
| `READ_COMMITTED` | Dirty reads |
| `REPEATABLE_READ` | Dirty reads, non-repeatable reads |
| `SERIALIZABLE` | Dirty reads, non-repeatable reads, phantom reads |

Treat this table as the *conceptual* SQL-standard mapping, not a promise about every database's actual implementation: some vendors alias `READ_UNCOMMITTED` up to `READ_COMMITTED`, and `REPEATABLE_READ`/`SERIALIZABLE` are implemented differently across engines (locking vs. MVCC/snapshot). Without knowing the target database and version, treat any claim about which specific anomaly a chosen isolation level prevents *on that database* as `inference (partial source)` at best.

## Escalation conditions

- The consistency question is really about a broker's producer/consumer transactional configuration (idempotent producer, `transactional.id`, `isolation.level`) rather than the application's transaction boundary → hand to `java-kafka-reliability-agent`.
- The slow-query or N+1 symptom sits inside a correctly-bounded transaction → hand to `java-jpa-hibernate-performance-agent`; boundary correctness and query shape are separate defects.
- The user asks to actually run the code to observe commit/rollback behavior → out of scope for static review; describe what to instrument and who runs it.
