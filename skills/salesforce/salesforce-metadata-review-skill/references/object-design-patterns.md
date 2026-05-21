# Object Design Patterns Reference

Reference for Salesforce custom object design including relationship types,
junction objects, OWD considerations, and sharing model design.

---

## Relationship Type Selection

### Master-Detail vs Lookup

| Dimension | Master-Detail | Lookup |
|-----------|--------------|--------|
| Required on child | Yes (always) | Optional |
| Cascade delete | Yes — child deleted when master deleted | No — lookup becomes null |
| Roll-up summary fields | Available on master | Not available |
| Sharing inherited | Child inherits master's sharing | Independent |
| OWD override | Cannot have OWD on detail; uses master's | Own OWD |
| Record ownership | Child has no owner; master's owner controls | Child has own owner |
| Max per object | 2 master-detail | 40 lookups |

### When to Use Master-Detail

Use master-detail when:
- The child record cannot exist independently of the parent.
- Roll-up summaries (SUM, COUNT, MIN, MAX) on the parent are required.
- Child records should be automatically deleted when parent is deleted.

Examples:
- `OrderLineItem` is master-detail to `Order`.
- `OpportunityLineItem` is master-detail to `Opportunity`.
- Custom `ProjectTask__c` is master-detail to `Project__c`.

### When to Use Lookup

Use lookup when:
- The child may exist without a parent (nullable foreign key).
- The child record has its own sharing model independent of the parent.
- The relationship is many-to-many (junction object pattern).

---

## Junction Object Pattern

A junction object implements a many-to-many relationship by having two
master-detail relationships (or two lookups) pointing to the two related objects.

### Design Example: Contact — Event (Many-to-Many)

```
Contact (1) <--- EventAttendee__c (junction) ---> (1) Event__c
```

```apex
// EventAttendee__c fields:
//   Contact__c   : Master-Detail to Contact
//   Event__c     : Master-Detail to Event__c (second master-detail)
//   Status__c    : Picklist (Registered, Attended, Cancelled)
//   RegistrationDate__c : DateTime

// Query all events a contact attended:
List<EventAttendee__c> attended = [
    SELECT Event__c, Event__r.Name, Event__r.StartDate__c, Status__c
    FROM EventAttendee__c
    WHERE Contact__c = :contactId
      AND Status__c = 'Attended'
    ORDER BY Event__r.StartDate__c DESC
];
```

### Junction Object Sharing Considerations

When both sides of a junction object are master-detail:
- The junction object's sharing is controlled by the object that is most
  restrictive (intersection of both masters' record access).
- If Contact is Private OWD and Event is Public Read Only, the junction
  record is accessible only to users who can access BOTH the Contact AND
  the Event.

When lookup relationships are used instead:
- The junction object has its own OWD and sharing rules.
- More flexible but requires explicit sharing configuration.

---

## Organization-Wide Defaults (OWD)

OWD is the baseline access level for records when no sharing rule or explicit
ownership grants access.

### OWD Options by Object Type

| OWD Setting | Meaning |
|-------------|---------|
| Private | Only record owner and users above in role hierarchy can access |
| Public Read Only | All users can view; only owner and hierarchy can edit |
| Public Read/Write | All users can view and edit |
| Public Read/Write/Transfer | All users can view, edit, and transfer ownership |
| Controlled by Parent | Detail object inherits access from master object |

### OWD Recommendations by Data Sensitivity

| Object | Recommended OWD |
|--------|----------------|
| Account (B2B) | Private (territory-based visibility) |
| Account (B2C) | Private |
| Contact | Controlled by Parent (if linked to Account) or Private |
| Opportunity | Private |
| Case | Private (customer service owns case visibility) |
| Lead | Private |
| Product (Pricebook2/Product2) | Public Read Only |
| Campaign | Public Read/Write |
| Custom operational object | Private (safest default; relax via sharing rules) |

### Tightening OWD Without Breaking Functionality

Moving from Public to Private OWD requires:
1. Identify all users who currently have access and should retain it.
2. Create Sharing Rules to restore needed access.
3. Identify Apex code using `without sharing` — review whether it over-exposes.
4. Test in a full sandbox with representative user profiles before production.

---

## Sharing Model Design

Salesforce sharing works in layers. Access is granted by ANY of the following:

```
1. OWD (baseline)
2. Role Hierarchy (managers see subordinates' records)
3. Sharing Rules (criteria-based or ownership-based)
4. Manual Sharing (user grants access to individual record)
5. Apex Managed Sharing (programmatic, custom ShareObject records)
6. Teams (Account Teams, Opportunity Teams)
7. Territory Management (if enabled)
```

### Sharing Rule Types

| Type | How It Works |
|------|-------------|
| Owner-based | Records owned by users in group A are shared with group B |
| Criteria-based | Records matching field criteria are shared with a group |
| Guest User sharing | Special rules for Experience Cloud guest users |

```sql
-- Query sharing rules (Metadata API recommended for full detail)
-- List Group-Based Sharing rules via Tooling API
SELECT DeveloperName, SharingGroupId, AccessLevel, UserOrGroupId
FROM AccountShare
WHERE RowCause = 'SharingCriteriaRule'
LIMIT 200
```

### Apex Managed Sharing

For complex sharing requirements that criteria-based rules cannot express:

```apex
// Share a custom object record with a specific user
CustomObject__Share share = new CustomObject__Share();
share.ParentId = recordId;
share.UserOrGroupId = userId;
share.AccessLevel = 'Read';
share.RowCause = Schema.CustomObject__Share.rowCause.Manual;  // or a custom reason

Database.SaveResult result = Database.insert(share, false);
```

Apex managed sharing persists across record changes but must be explicitly
removed when access should be revoked.

---

## Custom Object Design Checklist

- [ ] Relationship type (master-detail vs lookup) documented with rationale.
- [ ] OWD for new object set to Private as a starting point.
- [ ] Sharing rules defined for all required access scenarios.
- [ ] Roll-up summaries identified and implemented where master-detail exists.
- [ ] Custom object has a meaningful naming convention and namespace consideration.
- [ ] Object description field populated (visible in Salesforce documentation).
- [ ] Record types defined if multiple business processes share the object.
- [ ] Junction objects used for genuine many-to-many relationships (not a lookup array).
- [ ] External ID field defined if records will be upserted from external systems.
- [ ] Audit fields (CreatedDate, LastModifiedDate) expected behavior documented.
