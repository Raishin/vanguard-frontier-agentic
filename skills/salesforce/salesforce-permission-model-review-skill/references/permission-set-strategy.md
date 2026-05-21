# Permission Set Strategy Reference

Design principles and patterns for Salesforce permission set architecture
aligned with least-privilege and scalable access management.

---

## Profile vs Permission Set: Design Philosophy

In modern Salesforce security design, the profile should carry the minimum
viable baseline, and Permission Sets (and Permission Set Groups) should be
used to grant everything additional.

### Profile Responsibilities (Minimal)

| What Profile Should Control | Why |
|-----------------------------|-----|
| Login hours | Can only be set on profile |
| Login IP ranges | Can only be set on profile |
| Object-level CRUD defaults (read-only or no access) | Baseline denial |
| Page layout assignments | User experience baseline |
| License type | Determined by profile |

### What Profile Should NOT Control (Move to Permission Sets)

- Field-level security beyond the minimum.
- Specific object permissions for data access.
- Application permissions.
- System permissions (e.g., API Enabled, Export Reports).

This approach means users can be on a minimal baseline profile and receive
job-specific permissions through Permission Sets without needing a new profile
for every combination.

---

## Permission Set Group Design

Permission Set Groups aggregate multiple Permission Sets into a single
assignable unit. Assign the group, not individual permission sets.

### Example Architecture: Sales User

```
Permission Set Group: Sales_Representative
  |- Permission Set: Core_CRM_Access
  |    CRM Objects: Account (R), Contact (R/W), Opportunity (R/W)
  |    Fields: Standard fields only
  |
  |- Permission Set: Sales_Territory_Management
  |    Custom Fields: Territory__c, RegionOwner__c
  |
  |- Permission Set: Report_Export_Allowed
       System Permission: Export Reports

Profile (baseline): Minimum_Internal_User
  No object permissions
  Login hours: 6am-8pm Mon-Fri
  Login IP: [corporate VPN range]
```

### Role-Based vs Persona-Based

**Role-based** permission sets grant access based on the user's job function
within a team:

```
Sales_Manager_Access
  Apex Class: SalesDashboardController (exec)
  Object: Opportunity (View All)
  Reports: All Sales Reports folder (View)
```

**Persona-based** permission sets grant access based on a named user persona
that cuts across multiple roles:

```
Power_User_Data_Export
  System: Export Reports
  System: Mass Delete Records
  (granted only to named Power Users regardless of role)
```

Use role-based as the primary model. Persona-based for cross-cutting elevated
privileges that need tight control.

---

## Principle of Least Privilege in Practice

### Field-Level Security (FLS) Baseline

For sensitive fields, default FLS should be hidden to all profiles:

```
Setup > Object Manager > Contact > Fields and Relationships > SSN__c
  Field-Level Security:
    Default (all profiles): Read = false, Edit = false
    (Access granted via Permission Set for specific roles only)
```

Then grant via Permission Set:
```
Permission Set: PII_Data_Steward
  Object: Contact
  Fields: SSN__c (Read), SSN__c (Edit)
```

### Object Permission Baseline

Start new custom objects with no access in default profile:
```
Custom Object: PatientRecord__c
  Profile (Minimum_Internal): No CRUD access
  Permission Set: Clinical_Team_Access -> CRUD
  Permission Set: Billing_Team_Access -> Read only
```

### System Permission Least Privilege

| Permission | When to Grant | Avoid Granting To |
|------------|--------------|-------------------|
| API Enabled | Integration users, developers | All standard users |
| Modify All Data | System Administrator only | Any non-admin user |
| View All Data | Compliance/audit role only | General users |
| Manage Users | HR/IT admin team | Regular users |
| Export Reports | Named power users only | Default profile |
| ViewEncryptedData | Compliance team only | All users |
| Bulk API Hard Delete | Integration admin only | General users |

---

## Permission Set Lifecycle Management

### Provisioning Process

```
1. New employee joins:
   a. Assign role-based Permission Set Group for their function.
   b. Do NOT assign individual Permission Sets unless the Group doesn't cover.
   
2. User changes roles:
   a. Remove old Permission Set Group.
   b. Assign new Permission Set Group.
   c. Review any individually assigned Permission Sets; remove if no longer valid.

3. Employee offboards:
   a. Deactivate user account (removes all permission set assignments).
   b. Document any records they owned that need reassignment.
```

### Permission Set Audit Query

```sql
-- Find all active users and their permission set assignments
SELECT Assignee.Name, Assignee.Username, PermissionSet.Name,
       PermissionSet.IsOwnedByProfile, PermissionSet.ProfileId
FROM PermissionSetAssignment
WHERE Assignee.IsActive = true
  AND PermissionSet.IsOwnedByProfile = false  -- excludes profile-owned sets
ORDER BY Assignee.Name, PermissionSet.Name
```

### Orphaned Assignments (Users Deactivated but Assignments Not Cleaned)

```sql
SELECT Assignee.Name, Assignee.Username, PermissionSet.Name
FROM PermissionSetAssignment
WHERE Assignee.IsActive = false
  AND PermissionSet.IsOwnedByProfile = false
ORDER BY Assignee.Name
LIMIT 200
```
Deactivated users retain permission set assignments in the database. These
are not a live security risk (inactive users cannot log in) but represent
cleanup debt and can cause confusion during re-activation.

---

## Permission Set vs Permission Set Group: Decision Guide

| Scenario | Use |
|----------|-----|
| Single capability (e.g., "Export Reports") | Permission Set |
| Job function requiring multiple capabilities | Permission Set Group |
| Temporary elevated access for a project | Permission Set (time-limited manual assignment) |
| System-level access (admin equivalents) | Permission Set Group with approval workflow |
| Integration user access | Dedicated Permission Set matching integration's exact needs |

---

## Salesforce Recommended Security Patterns <!-- verify-before-merge:2026-05-21 -->

1. **One profile per user type** — minimize the number of profiles to reduce
   maintenance burden.
2. **Never grant system permissions at the profile level** — use Permission Sets.
3. **Use Permission Set Groups** for all job-function bundles.
4. **Name Permission Sets by capability**, not by user type:
   - GOOD: `Can_Export_Reports`, `Read_Financial_Data`, `API_Integration_Access`
   - BAD: `Sales_Rep_Set`, `Marketing_User`, `IT_Admin_Plus`
5. **Review all Permission Set assignments quarterly** for critical permissions.
6. **Use muting Permission Sets** in Permission Set Groups to selectively remove
   permissions granted by included sets without removing the entire set.
