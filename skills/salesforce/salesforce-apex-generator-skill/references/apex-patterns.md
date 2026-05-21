# Apex Patterns Reference

Adapted from forcedotcom/sf-skills generating-apex references (Apache-2.0).

## Service-Selector-Domain Pattern

The canonical layering for enterprise Apex. Each layer has a single responsibility.

```
Trigger
  └── Domain (trigger-context orchestration)
        └── Service (business logic, cross-object coordination)
              └── Selector (data access, SOQL)
```

### Selector Layer

- Owns all SOQL for a given SObject.
- Returns strongly-typed `List<SObject>` or `Map<Id, SObject>`.
- `with sharing` by default.
- No business logic — pure data retrieval.
- Method naming: `get{Purpose}By{Filter}` (e.g., `getAccountsByOwnerIds`).

```apex
public with sharing class AccountSelector {
    public List<Account> getAccountsByOwnerIds(Set<Id> ownerIds) {
        return [
            SELECT Id, Name, Industry, OwnerId
            FROM Account
            WHERE OwnerId IN :ownerIds
            WITH USER_MODE
        ];
    }
}
```

### Service Layer

- Owns business logic.
- Calls Selector for data; calls Domain for trigger-context operations.
- `with sharing` by default.
- Accepts `Set<Id>` or typed collections; never a single Id in production paths.
- Method naming: verb-noun (`processOrderUpdates`, `calculateDiscounts`).

```apex
public with sharing class AccountService {
    private static final AccountSelector SELECTOR = new AccountSelector();

    public void updateIndustryForOwners(Set<Id> ownerIds, String newIndustry) {
        List<Account> accounts = SELECTOR.getAccountsByOwnerIds(ownerIds);
        List<Account> toUpdate = new List<Account>();
        for (Account acc : accounts) {
            toUpdate.add(new Account(Id = acc.Id, Industry = newIndustry));
        }
        update toUpdate;
    }
}
```

### Domain Layer

- Owns trigger-context orchestration.
- Receives trigger lists/maps from the trigger handler.
- Delegates to Service for business logic.
- `with sharing` by default (unless the trigger requires bypassed sharing for system operations).

```apex
public with sharing class AccountDomain {
    public void onBeforeUpdate(List<Account> newList, Map<Id, Account> oldMap) {
        Set<Id> changedOwners = new Set<Id>();
        for (Account acc : newList) {
            if (acc.OwnerId != oldMap.get(acc.Id).OwnerId) {
                changedOwners.add(acc.Id);
            }
        }
        if (!changedOwners.isEmpty()) {
            AccountService.getInstance().handleOwnerChange(changedOwners);
        }
    }
}
```

---

## Sharing Models

| Model | When to use |
|---|---|
| `with sharing` | Default. Enforces record-level sharing rules. Use for all user-visible data access. |
| `without sharing` | Only when the class must bypass sharing for a documented system operation (e.g., background data sync, admin utility). Document the reason with an inline comment. |
| `inherited sharing` | Use for library/utility classes called by both `with sharing` and `without sharing` callers. Inherits the sharing context of the caller. |

**Default decision tree:**

1. Is this class directly invoked from a user-facing context (trigger, VF, LWC, API)?
   → `with sharing`
2. Is this a utility/library called by multiple contexts?
   → `inherited sharing`
3. Is this a background process that intentionally bypasses user sharing (with documented justification)?
   → `without sharing`

---

## Async Patterns

### Queueable (preferred over @future)

Use Queueable for all async operations. Supports: chaining, non-primitive parameters,
`System.Finalizer` for error handling, and callouts with `Database.AllowsCallouts`.

```apex
public with sharing class OrderEventQueueable implements Queueable, Database.AllowsCallouts {
    private final List<Id> orderIds;

    public OrderEventQueueable(List<Id> orderIds) {
        this.orderIds = orderIds;
    }

    public void execute(QueueableContext ctx) {
        // business logic here
    }
}

// Enqueue
System.enqueueJob(new OrderEventQueueable(orderIds));
```

**Never use `@future`.** Reasons:
- Cannot chain
- Cannot be called from Batch context
- Cannot accept non-primitive types
- Cannot use `System.Finalizer`

### Batchable

Use for large-volume data processing (> 10,000 records or > governor-limit scope).

```apex
public with sharing class AccountDeduplicationBatch
    implements Database.Batchable<SObject>, Database.Stateful {

    private Integer processedCount = 0; // only if stateful tracking required

    public Database.QueryLocator start(Database.BatchableContext ctx) {
        return Database.getQueryLocator([
            SELECT Id, Name FROM Account WHERE IsDeleted = false WITH USER_MODE
        ]);
    }

    public void execute(Database.BatchableContext ctx, List<Account> scope) {
        // process each batch
        this.processedCount += scope.size();
    }

    public void finish(Database.BatchableContext ctx) {
        // post-processing
    }
}

// Execute
Database.executeBatch(new AccountDeduplicationBatch(), 200);
```

`Database.Stateful` only when inter-batch state accumulation is required. Use sparingly —
it increases heap usage and slows batch jobs.

### Schedulable

Thin wrapper that delegates to a Queueable or Batch. No business logic in Schedulable.

```apex
public with sharing class NightlyCleanupScheduler implements Schedulable {
    public void execute(SchedulableContext ctx) {
        System.enqueueJob(new CleanupQueueable());
    }
}

// Schedule
System.schedule('Nightly Cleanup', '0 0 2 * * ?', new NightlyCleanupScheduler());
```

---

## REST Resource Pattern

```apex
@RestResource(urlMapping='/orders/v1/*')
global without sharing class OrderRestResource {

    @HttpGet
    global static OrderResponse doGet() {
        RestRequest req = RestContext.request;
        String orderId = req.requestURI.substringAfterLast('/');

        // validate input
        if (String.isBlank(orderId) || orderId.length() < 15) {
            RestContext.response.statusCode = 400;
            return new OrderResponse(null, 'Invalid order ID');
        }

        List<Order> orders = [
            SELECT Id, OrderNumber, Status FROM Order
            WHERE Id = :orderId WITH USER_MODE LIMIT 1
        ];

        if (orders.isEmpty()) {
            RestContext.response.statusCode = 404;
            return new OrderResponse(null, 'Order not found');
        }

        return new OrderResponse(orders[0], null);
    }

    global class OrderResponse {
        public Order order;
        public String error;
        public OrderResponse(Order o, String e) { this.order = o; this.error = e; }
    }
}
```

Note: REST resources require `global` access and typically `without sharing` because
the platform-enforced authentication controls access; the sharing bypass is intentional
and should be documented. Always validate input and apply `WITH USER_MODE` on SOQL.
