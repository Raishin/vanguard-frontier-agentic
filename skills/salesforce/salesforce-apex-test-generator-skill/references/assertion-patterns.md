# Apex Assertion Patterns Reference

Adapted from forcedotcom/sf-skills generating-apex-test references (Apache-2.0).

## Assert Class (Apex API v55.0+)

Always use the `Assert` class instead of bare `System.assert()`. The `Assert` class
provides meaningful failure messages by default and aligns with modern Apex best practices.

## Core Assert Methods

```apex
// Assert equality (most common)
Assert.areEqual(expected, actual, 'Optional failure message');

// Assert inequality
Assert.areNotEqual(unexpected, actual, 'Optional failure message');

// Assert boolean true
Assert.isTrue(condition, 'Optional failure message');

// Assert boolean false
Assert.isFalse(condition, 'Optional failure message');

// Assert not null
Assert.isNotNull(value, 'Optional failure message');

// Assert null
Assert.isNull(value, 'Optional failure message');

// Assert instanceof (type check)
Assert.isInstanceOfType(obj, MyClass.class, 'Optional failure message');

// Explicit failure (use in exception testing)
Assert.fail('Should have thrown an exception before reaching this line');
```

## Error Messages

Always provide a descriptive failure message. Bare assertions without messages produce
"Assertion failed" with no context — useless when multiple assertions fail in a test run.

```apex
// BAD — no context on failure
Assert.areEqual(5, results.size());

// GOOD — clear what failed and what was expected
Assert.areEqual(5, results.size(), 'Expected 5 accounts returned for active owners');
```

## Positive Path Assertions

```apex
@isTest
static void testGetAccountsByOwner_Success() {
    // Arrange
    User owner = TestDataFactory.createUser(true);
    List<Account> accounts = TestDataFactory.createAccounts(3, false);
    for (Account acc : accounts) acc.OwnerId = owner.Id;
    insert accounts;

    // Act
    Test.startTest();
    List<Account> results = AccountSelector.getAccountsByOwnerId(owner.Id);
    Test.stopTest();

    // Assert
    Assert.areEqual(3, results.size(), 'Expected 3 accounts for owner');
    Assert.areEqual(owner.Id, results[0].OwnerId, 'OwnerId should match');
}
```

## Negative Path / Exception Assertions

```apex
@isTest
static void testGetAccountsByOwner_ThrowsOnNullId() {
    // Arrange — no setup needed

    // Act + Assert
    Test.startTest();
    try {
        AccountSelector.getAccountsByOwnerId(null);
        Assert.fail('Expected IllegalArgumentException for null ownerId');
    } catch (IllegalArgumentException e) {
        Assert.isTrue(
            e.getMessage().contains('ownerId cannot be null'),
            'Exception message should mention ownerId: ' + e.getMessage()
        );
    }
    Test.stopTest();
}
```

## Bulk Assertions

Do not assert only on the first record. Assert on all records when testing bulk behavior.

```apex
@isTest
static void testUpdateIndustry_Bulk() {
    // Arrange
    List<Account> accounts = TestDataFactory.createAccounts(201, true);

    // Act
    Test.startTest();
    AccountService.updateIndustry(new Map<Id, Account>(accounts).keySet(), 'Finance');
    Test.stopTest();

    // Assert — check ALL records
    List<Account> updated = [SELECT Industry FROM Account WHERE Id IN :accounts];
    Assert.areEqual(201, updated.size(), 'All 201 accounts should be retrieved');
    for (Account acc : updated) {
        Assert.areEqual('Finance', acc.Industry,
            'Industry should be Finance for account: ' + acc.Id);
    }
}
```

## Multi-Assert vs Single Assert

Prefer multiple specific assertions over a single compound assertion. Each assertion
provides independent failure context:

```apex
// BAD — single compound assertion hides which field failed
Assert.isTrue(acc.Name == 'Acme' && acc.Industry == 'Tech' && acc.Active__c == true);

// GOOD — each assertion independently identifiable
Assert.areEqual('Acme', acc.Name, 'Account Name mismatch');
Assert.areEqual('Tech', acc.Industry, 'Account Industry mismatch');
Assert.isTrue(acc.Active__c, 'Account should be active');
```

## Asserting Exception Types

When testing that specific exception types are thrown, assert the type explicitly:

```apex
@isTest
static void testService_ThrowsCustomException() {
    Test.startTest();
    try {
        MyService.processWithInvalidData(null);
        Assert.fail('Expected MyCustomException was not thrown');
    } catch (MyCustomException e) {
        Assert.isTrue(
            e.getMessage().startsWith('Invalid'),
            'Exception message should start with Invalid: ' + e.getMessage()
        );
    } catch (Exception e) {
        Assert.fail('Unexpected exception type: ' + e.getTypeName() + ': ' + e.getMessage());
    }
    Test.stopTest();
}
```

## Deprecated Patterns to Avoid

```apex
// DEPRECATED — use Assert.isTrue instead
System.assert(condition);
System.assert(condition, 'message');

// DEPRECATED — use Assert.areEqual instead
System.assertEquals(expected, actual);
System.assertEquals(expected, actual, 'message');

// DEPRECATED — use Assert.areNotEqual instead
System.assertNotEquals(unexpected, actual);
```

These deprecated forms still compile and work but are not the modern Apex testing
standard. All new test code should use the `Assert` class.
