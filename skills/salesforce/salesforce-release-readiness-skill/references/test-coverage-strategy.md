# Test Coverage Strategy Reference

Patterns for achieving and maintaining meaningful Apex test coverage beyond
the Salesforce-mandated 75% threshold.

---

## Coverage Thresholds

| Threshold | Significance |
|-----------|-------------|
| 75% | Salesforce deployment minimum — cannot deploy to production below this |
| 80% | Internal quality gate (common industry standard) |
| 85% | Recommended target for business-critical code |
| 90%+ | Target for payment processing, HIPAA, or PCI-regulated code paths |

Note: Coverage percentage measures executed lines — not assertion quality.
High coverage with zero assertions is worse than 75% coverage with meaningful assertions.

---

## Test Class Organization

### Naming Convention

| Component Type | Test Class Name Pattern |
|----------------|------------------------|
| Apex Class: `AccountService.cls` | `AccountServiceTest.cls` |
| Apex Trigger: `AccountTrigger.trigger` | `AccountTriggerTest.cls` |
| Batch class: `AccountCleanupBatch.cls` | `AccountCleanupBatchTest.cls` |
| Queueable: `AccountSyncQueueable.cls` | `AccountSyncQueueableTest.cls` |

### File Structure

```
force-app/
  main/
    default/
      classes/
        AccountService.cls
        AccountService.cls-meta.xml
        AccountServiceTest.cls
        AccountServiceTest.cls-meta.xml
```

Tests must be in the same package/directory as the class they test for
proper coverage association.

---

## Test Data Strategy

### Anti-Pattern: SeeAllData=true

```apex
// WRONG: SeeAllData makes tests dependent on org data
@isTest(SeeAllData=true)
public class AccountServiceTest {
    @isTest
    static void testGetAccount() {
        // Relies on prod data existing in org — fails in clean sandboxes
        Account a = [SELECT Id FROM Account LIMIT 1];
        // ...
    }
}
```

**Acceptable uses of SeeAllData=true:**
- Reading Pricebook2 standard price book ID (legacy workaround).
- Accessing some Financial Services Cloud records in early development.

### Correct Pattern: @testSetup and Test Factory

```apex
@isTest
public class AccountServiceTest {

    @testSetup
    static void setup() {
        // Create all test data once for the entire test class
        Account testAccount = new Account(
            Name = 'Test Corp',
            BillingCity = 'San Francisco',
            BillingState = 'CA',
            Industry = 'Technology'
        );
        insert testAccount;

        Contact testContact = new Contact(
            AccountId = testAccount.Id,
            FirstName = 'Test',
            LastName = 'User',
            Email = 'test@example.test'
        );
        insert testContact;
    }

    @isTest
    static void testGetAccount_returnsAccount() {
        // Arrange
        Account testAccount = [SELECT Id FROM Account WHERE Name = 'Test Corp' LIMIT 1];

        // Act
        Test.startTest();
        Account result = AccountService.getAccount(testAccount.Id);
        Test.stopTest();

        // Assert
        System.assertNotEquals(null, result, 'Result should not be null');
        System.assertEquals('Test Corp', result.Name, 'Account name should match');
    }
}
```

### Test Factory Pattern

For orgs with complex objects and many test classes, a shared test factory
prevents code duplication and ensures consistent test data:

```apex
@isTest
public class TestDataFactory {

    public static Account createAccount(String name) {
        return createAccount(name, true);
    }

    public static Account createAccount(String name, Boolean doInsert) {
        Account a = new Account(
            Name = name,
            BillingStreet = '123 Test Street',
            BillingCity = 'San Francisco',
            BillingState = 'CA',
            BillingPostalCode = '94105',
            BillingCountry = 'US',
            Phone = '+1-555-0100'
        );
        if (doInsert) insert a;
        return a;
    }

    public static Contact createContact(Id accountId, String lastName) {
        Contact c = new Contact(
            AccountId = accountId,
            FirstName = 'Test',
            LastName = lastName,
            Email = lastName.toLowerCase() + '@example.test'
        );
        insert c;
        return c;
    }
}
```

---

## Coverage by Code Path Type

### Testing the Happy Path

```apex
@isTest
static void testCreateCase_success() {
    Account a = TestDataFactory.createAccount('ACME Corp');
    Contact c = TestDataFactory.createContact(a.Id, 'Jones');

    Test.startTest();
    Case result = CaseService.createCase(c.Id, 'Billing Issue', 'High');
    Test.stopTest();

    System.assertNotEquals(null, result.Id, 'Case should be created');
    System.assertEquals('Billing Issue', result.Subject, 'Subject should match');
    System.assertEquals('High', result.Priority, 'Priority should match');
}
```

### Testing Error Paths (Validation, Exception)

```apex
@isTest
static void testCreateCase_throwsException_whenContactNotFound() {
    Boolean exceptionThrown = false;
    try {
        Test.startTest();
        CaseService.createCase('003000000000000', 'Test', 'Low'); // bogus ID
        Test.stopTest();
    } catch (AuraHandledException ex) {
        exceptionThrown = true;
        System.assert(ex.getMessage().contains('Contact not found'),
            'Exception message should indicate contact not found');
    }
    System.assert(exceptionThrown, 'Exception should have been thrown');
}
```

### Testing Bulk Scenarios

```apex
@isTest
static void testProcessAccounts_bulk_200Records() {
    List<Account> accounts = new List<Account>();
    for (Integer i = 0; i < 200; i++) {
        accounts.add(new Account(Name = 'Bulk Test ' + i));
    }
    insert accounts;

    Test.startTest();
    AccountProcessor.processAll(accounts);
    Test.stopTest();

    // Verify all 200 processed without governor limit errors
    List<Account> processed = [SELECT Id, ProcessedDate__c FROM Account WHERE Name LIKE 'Bulk Test%'];
    System.assertEquals(200, processed.size(), 'All 200 accounts should be processed');
    for (Account a : processed) {
        System.assertNotEquals(null, a.ProcessedDate__c, 'All accounts should have ProcessedDate set');
    }
}
```

### Testing Async Apex

```apex
@isTest
static void testBatchApex() {
    List<Account> accounts = TestDataFactory.createAccountBatch(100);

    Test.startTest();
    AccountCleanupBatch batch = new AccountCleanupBatch();
    Id jobId = Database.executeBatch(batch, 200);
    Test.stopTest(); // Forces async execution to complete synchronously in test

    // Assert post-batch state
    List<Account> updated = [SELECT Id, Status__c FROM Account WHERE Id IN :accounts];
    for (Account a : updated) {
        System.assertEquals('Processed', a.Status__c, 'All accounts should be processed by batch');
    }
}
```

---

## CI Integration

### GitHub Actions: Apex Test on Pull Request

```yaml
name: Apex Test Coverage
on: [pull_request]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Install Salesforce CLI
        run: npm install -g @salesforce/cli
      - name: Authenticate to sandbox
        run: |
          echo "${{ secrets.SF_AUTH_URL }}" | sf org login sfdx-url \
            --sfdx-url-stdin \
            --alias ci-org \
            --set-default
      - name: Run tests and check coverage
        run: |
          sf apex run test \
            --target-org ci-org \
            --test-level RunLocalTests \
            --result-format tap \
            --code-coverage \
            --wait 30 | tee test-results.txt
          
          # Extract coverage percentage from results and fail if below threshold
          COVERAGE=$(grep "Org Wide Coverage" test-results.txt | grep -o '[0-9]*%' | tr -d '%')
          if [ "$COVERAGE" -lt 85 ]; then
            echo "Coverage $COVERAGE% is below 85% threshold"
            exit 1
          fi
```

---

## Coverage Reporting

### Querying Coverage Data via Tooling API

```bash
# Per-class coverage
sf data query \
  --query "SELECT ApexClassOrTrigger.Name, NumLinesCovered, NumLinesUncovered \
           FROM ApexCodeCoverageAggregate \
           ORDER BY NumLinesUncovered DESC \
           LIMIT 50" \
  --use-tooling-api \
  -o my-org

# Org-wide aggregate
sf data query \
  --query "SELECT PercentCovered FROM ApexOrgWideCoverage" \
  --use-tooling-api \
  -o my-org
```

---

## Test Coverage Anti-Patterns

| Anti-Pattern | Issue |
|-------------|-------|
| Test methods with no assertions | Coverage without validation |
| One giant test method testing everything | Hard to diagnose failures |
| `SeeAllData=true` | Tests break in clean sandboxes |
| Tests that only test governor limit compliance | No business logic verified |
| Copy-pasted tests for trivial variations | Maintenance burden |
| Testing framework/platform behavior (e.g., that DML works) | Waste of coverage budget |
| Tests that mutate shared state without @testSetup isolation | Flaky test ordering dependencies |
