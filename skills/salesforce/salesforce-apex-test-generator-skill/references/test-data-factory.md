# TestDataFactory Patterns Reference

Adapted from forcedotcom/sf-skills generating-apex-test references (Apache-2.0).

## Core Design Rules

1. **Always accept a `doInsert` flag** — lets callers modify records before insert
2. **Append loop index to fields in Duplicate Rules** — prevents `DUPLICATES_DETECTED`
3. **Single-record methods delegate to bulk** — `createAccount(doInsert)` calls `createAccounts(1, doInsert)[0]`
4. **Return created records** — enables chaining and further manipulation
5. **Set all required fields** — include validation-rule-required fields, not just schema-required

## Base Class Structure

```apex
@isTest
public class TestDataFactory {

    // Account
    public static Account createAccount(Boolean doInsert) {
        return createAccounts(1, doInsert)[0];
    }

    public static List<Account> createAccounts(Integer count, Boolean doInsert) {
        List<Account> accounts = new List<Account>;
        for (Integer i = 0; i < count; i++) {
            accounts.add(new Account(
                Name = 'Test Account ' + i,
                Industry = 'Technology',
                BillingCity = 'San Francisco'
            ));
        }
        if (doInsert) insert accounts;
        return accounts;
    }

    // Contact
    public static Contact createContact(Id accountId, Boolean doInsert) {
        return createContacts(accountId, 1, doInsert)[0];
    }

    public static List<Contact> createContacts(Id accountId, Integer count, Boolean doInsert) {
        List<Contact> contacts = new List<Contact>;
        for (Integer i = 0; i < count; i++) {
            contacts.add(new Contact(
                FirstName = 'Test',
                LastName = 'Contact ' + i,
                Email = 'testcontact' + i + '@example.com',
                AccountId = accountId
            ));
        }
        if (doInsert) insert contacts;
        return contacts;
    }
}
```

## Field Override Pattern

Allows callers to customize records without creating new factory methods:

```apex
public static Account createAccount(Map<String, Object> fieldOverrides, Boolean doInsert) {
    Account acc = new Account(
        Name = 'Test Account',
        Industry = 'Technology'
    );
    for (String fieldName : fieldOverrides.keySet) {
        acc.put(fieldName, fieldOverrides.get(fieldName));
    }
    if (doInsert) insert acc;
    return acc;
}

// Usage:
Account acc = TestDataFactory.createAccount(
    new Map<String, Object>{'Name' => 'Acme Corp', 'Industry' => 'Healthcare'},
    true
);
```

## @TestSetup vs Inline Factory

**Use @TestSetup when:**
- Multiple test methods need the same base data
- Setup is expensive (many records, related objects)

```apex
@isTest
private class AccountServiceTest {

    @TestSetup
    static void setup {
        List<Account> accounts = TestDataFactory.createAccounts(10, true);
        TestDataFactory.createContacts(accounts[0].Id, 5, true);
    }

    @isTest
    static void testGetActiveAccounts {
        // data from @TestSetup is available; each test gets a fresh copy
        List<Account> accounts = [SELECT Id FROM Account];
        Assert.areEqual(10, accounts.size, 'Expected 10 accounts from setup');
    }
}
```

**Important @TestSetup notes:**
- DML in @TestSetup is rolled back between test methods — each test gets its own
  isolated copy of the setup data
- @TestSetup cannot be used in test classes that use `SeeAllData=true`
- @TestSetup runs once per class, not once per method

## Bulk Factory Pattern (200+ Records)

For bulkification tests, always create at least 201 records to cross the trigger batch
boundary:

```apex
@isTest
static void testBulkAccountUpdate {
    List<Account> accounts = TestDataFactory.createAccounts(201, true);
    // test bulk update path
    for (Account acc : accounts) {
        acc.Industry = 'Finance';
    }
    Test.startTest;
    update accounts;
    Test.stopTest;

    List<Account> updated = [SELECT Industry FROM Account WHERE Id IN :accounts];
    Assert.areEqual(201, updated.size, 'All 201 accounts should be updated');
    for (Account acc : updated) {
        Assert.areEqual('Finance', acc.Industry, 'Industry should be Finance for all records');
    }
}
```

## Record Type Support

```apex
public static Account createAccountByRecordType(String recordTypeDeveloperName, Boolean doInsert) {
    Id recordTypeId = Schema.SObjectType.Account
        .getRecordTypeInfosByDeveloperName
        .get(recordTypeDeveloperName)
        .getRecordTypeId;

    Account acc = new Account(
        Name = 'Test ' + recordTypeDeveloperName + ' Account',
        RecordTypeId = recordTypeId
    );
    if (doInsert) insert acc;
    return acc;
}
```

## Handling Duplicate Rules

When Duplicate Rules are active in the org, use `Database.insert` with `AllOrNone=false`
or `DuplicateRuleHeader` in test setup:

```apex
public static List<Account> createAccountsAllowDuplicates(Integer count, Boolean doInsert) {
    List<Account> accounts = new List<Account>;
    for (Integer i = 0; i < count; i++) {
        accounts.add(new Account(Name = 'Duplicate Test ' + i));
    }
    if (doInsert) {
        Database.DMLOptions opts = new Database.DMLOptions;
        opts.DuplicateRuleHeader.AllowSave = true;
        Database.insert(accounts, opts);
    }
    return accounts;
}
```
