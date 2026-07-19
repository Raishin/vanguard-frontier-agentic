# Dual-Write, Outbox, and Saga Patterns

> Static review only. Every conclusion below requires the actual write-then-publish (or write-then-call) call site as evidence — a consistency claim without seeing both operations, and whether they share a transaction, is `inference (partial source)` or `assumption (source absent)`. Sources: the transactional outbox and saga patterns as documented on microservices.io (Chris Richardson's pattern catalog, the commonly cited reference for these two patterns); Jakarta Transactions (JTA) specification for the XA/2PC model these patterns exist to avoid at the cross-service level. Broker-specific delivery-semantics details (Kafka's idempotent producer, `transactional.id`, consumer `isolation.level`) are explicitly out of scope here — see the escalation section.

## The dual-write problem

```java
@Service
public class OrderService {

    @Transactional
    public void placeOrder(Order order) {
        orderRepository.save(order);        // resource #1: the database
        eventPublisher.publish(              // resource #2: the broker
            new OrderPlacedEvent(order.getId())
        );
        // BAD: these two writes are to two independent resources.
        // The @Transactional boundary only covers the database. If the
        // process crashes, the broker is unreachable, or the publish
        // call throws AFTER the DB transaction has already committed
        // (or even mid-commit, depending on where publish() sits
        // relative to the transaction's actual commit point), the
        // event is silently lost. If publish() is retried on a
        // transient failure after the DB already committed, the event
        // can be silently duplicated instead.
    }
}
```

There is no ordering of `save()` and `publish()` inside a single `@Transactional` method that makes this atomic — a relational database commit and a message-broker publish are two separate resource managers with no shared commit protocol in this design. This holds whether `publish()` runs before or after `save()`, and whether it runs inside or outside the annotated boundary; moving it does not fix the fundamental problem, it only changes which side is more likely to lose the race.

## The remedy: transactional outbox

```java
@Service
public class OrderService {

    @Transactional
    public void placeOrder(Order order) {
        orderRepository.save(order);
        // GOOD: write the intent to publish into an outbox table, in
        // the SAME database transaction as the order write. Either
        // both rows exist after commit, or neither does — ordinary
        // single-resource ACID, no distributed coordination needed.
        outboxRepository.save(new OutboxEvent(
            "OrderPlaced", order.getId(), toJson(order)
        ));
    }
}

// A SEPARATE process — a polling relay or a CDC-based reader
// (e.g. reading the outbox table's change stream) — reads unpublished
// outbox rows and publishes them, marking each processed only after
// a successful publish acknowledgment. This relay's own delivery
// guarantees (retry, dedup, ordering) are what actually need
// verifying — not the original save() call site.
```

The outbox pattern converts a two-resource atomicity problem into a single-resource one (the DB write of the business row *and* the outbox row happen together, ordinarily) plus a separately-verifiable relay problem (does the relay eventually publish every unprocessed row, and does it avoid publishing the same row twice in a way consumers can't tolerate). Flag a design that writes directly to the broker from the request-handling transaction as needing an outbox; flag an outbox implementation that lacks a described relay (a table that fills up with rows nothing ever reads is not a working outbox).

## Post-commit side effects that aren't publish-to-a-broker

The same class of bug shows up for any post-commit side effect, not just messaging — calling another service synchronously, invalidating a cache, sending a notification:

```java
@Transactional
public void approveOrder(Order order) {
    order.setStatus(APPROVED);
    orderRepository.save(order);
    // BAD: if the transaction rolls back after this line runs (e.g. a
    // later step in the same method throws), the notification has
    // already gone out for an approval that never actually committed.
    notificationService.sendApprovalEmail(order);
}
```

```java
@Service
public class OrderService {

    @Transactional
    public void approveOrder(Order order) {
        order.setStatus(APPROVED);
        orderRepository.save(order);
        // GOOD: defer the side effect until commit is guaranteed.
        TransactionSynchronizationManager.registerSynchronization(
            new TransactionSynchronization() {
                @Override
                public void afterCommit() {
                    notificationService.sendApprovalEmail(order);
                }
            }
        );
    }
}

// Or, declaratively, with an application event:
@TransactionalEventListener(phase = TransactionPhase.AFTER_COMMIT)
public void onOrderApproved(OrderApprovedEvent event) {
    notificationService.sendApprovalEmail(event.getOrder());
}
```

`registerSynchronization`/`@TransactionalEventListener(phase = AFTER_COMMIT)` only guarantee the callback runs after the *current* transaction manager's commit — they do not make the side effect itself durable or exactly-once. If the process crashes between commit and the callback executing, the side effect is still lost; for a side effect that must survive that gap, it needs the same outbox treatment as messaging, not just an `AFTER_COMMIT` listener. Flag an `AFTER_COMMIT` listener presented as a complete fix for a side effect that must not be lost — it closes the pre-commit race but not the post-commit crash window.

## Cross-service atomicity: XA/2PC versus saga

A distributed transaction manager (XA, two-phase commit) can, in principle, make a database write and another XA-capable resource commit atomically. Across genuinely independent, separately-deployed services this is usually the wrong tool:

- It requires every participant to support and correctly implement the XA protocol — most modern message brokers and NoSQL/managed data stores do not.
- The coordinator becomes a single point of blocking: participants hold locks (and, for a DB participant, connections) until the coordinator resolves the transaction, which couples the availability of every service to the coordinator and to each other.
- It does not compose across organizational/deployment boundaries the way a single-service ACID transaction does.

The alternative for cross-service consistency is a **saga**: a sequence of local transactions, each in its own service, where every forward step that has an externally-visible effect is paired with a **compensating action** that semantically undoes it if a later step fails. Sagas can be **choreographed** (each service reacts to the previous service's event) or **orchestrated** (a coordinator service explicitly sequences the steps); either way, two properties are non-negotiable:

1. **Every forward step's side effect has a compensation.** A step that reserves inventory needs a "release reservation" compensation; a step that charges a card needs a "refund" compensation — not just a log entry saying it should have one.
2. **Every step (forward and compensating) is idempotent and safe to invoke on a step that never actually ran**, because retries and at-least-once delivery mean a step or its compensation can be invoked more than once, or a compensation can be invoked for a step that failed before doing anything.

Flag a design that assumes a distributed transaction manager makes cross-service calls atomic; flag a saga missing a compensation for any forward step with an externally-visible effect; flag a saga whose steps are not clearly idempotent.

## Escalation conditions

- The finding is really about the broker client's own delivery guarantees — idempotent producer configuration, `transactional.id`, consumer `isolation.level`, offset-commit strategy — rather than whether a dual write exists at all → hand to `java-kafka-reliability-agent`; this agent's job stops at "this needs an outbox / transactional wiring," not designing that wiring.
- The finding is about `@Transactional` propagation/isolation/rollback on the database side alone, with no second resource involved → see `propagation-isolation-and-proxy-pitfalls.md`; it is not a consistency finding by itself.
- The user asks to verify actual delivery/exactly-once behavior against a running broker → out of scope for static review; describe what to test and who runs it.
