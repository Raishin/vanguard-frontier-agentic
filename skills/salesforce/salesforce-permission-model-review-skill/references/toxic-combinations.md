# Toxic Combinations Reference

Permission combinations in Salesforce that create disproportionate security
risk when held by the same user or profile. Each combination requires
documented justification or remediation.

---

## Definition: Toxic Combination

A toxic combination is two or more permissions that, when held together, provide
a level of access significantly more dangerous than either permission alone.

---

## Tier 1: Critical Combinations (Zero Tolerance in Production)

### 1.1 ModifyAllData + API Enabled

**Why Critical:**
A user with ModifyAllData can update, delete, or create any record in the org.
Combined with API Enabled, this access is available programmatically — scripts,
bots, or attackers who compromise the credentials can automate mass data
destruction or exfiltration without any UI friction.

**Detection:**
```sql
SELECT Id, Name, Profile.Name,
       Profile.PermissionsModifyAllData,
       Profile.PermissionsApiEnabled
FROM User
WHERE IsActive = true
  AND Profile.PermissionsModifyAllData = true
  AND Profile.PermissionsApiEnabled = true
  AND Profile.Name != 'System Administrator'
```

**Acceptable:** System Administrator profile only, with < 5 named users,
no shared logins, IP restrictions enforced.

### 1.2 ModifyAllData + ManageUsers

**Why Critical:**
A user who can modify all data AND manage users can create new admin users,
reset passwords, and potentially create a persistent backdoor even if their
original account is revoked.

**Detection:**
```sql
SELECT Id, Name, Profile.Name
FROM User
WHERE IsActive = true
  AND Profile.PermissionsModifyAllData = true
  AND Profile.PermissionsManageUsers = true
  AND Profile.Name != 'System Administrator'
```

### 1.3 ViewEncryptedData + API Enabled

**Why Critical:**
ViewEncryptedData allows a user to see Shield Platform Encryption <!-- verify-before-merge:2026-05-21 -->
encrypted field values in plaintext. Combined with API Enabled, encrypted
data (SSNs, financial accounts, health data) can be bulk-exported via API
without any additional UI barrier.

**Detection:**
```sql
SELECT Id, Name, Profile.Name, Profile.PermissionsViewEncryptedData
FROM User
WHERE IsActive = true
  AND Profile.PermissionsViewEncryptedData = true
  AND Profile.PermissionsApiEnabled = true
LIMIT 100
```

**Expected:** Only users with documented business need for decrypted field
access (e.g., compliance team, specific data stewards). Count should be < 10.

---

## Tier 2: High-Risk Combinations (Require Documented Justification)

### 2.1 ViewAllData + Export Reports

A user who can view all records AND export report results can bulk-extract
any data in the org into a spreadsheet. Without Export Reports, ViewAllData
is limited to on-screen viewing.

**Detection:**
```sql
SELECT Id, Name, Profile.Name,
       Profile.PermissionsViewAllData
FROM User
WHERE IsActive = true
  AND Profile.PermissionsViewAllData = true
LIMIT 200
-- Then separately check if profile has Export Reports permission
-- (PermissionsExportReport not available in simple SOQL — use Metadata API)
```

### 2.2 ManageDataCategories + Manage Knowledge

A user who controls Knowledge Data Categories AND manages Knowledge Articles
can restructure the knowledge base in ways that affect Agentforce grounding,
search relevance, and customer-facing help content simultaneously.

**Finding:** No justification needed if role is explicitly Knowledge Manager —
document the role and ensure the account has MFA.

### 2.3 AuthorApex + ManageFlowMigrateConnections

**Why Risky:**
A developer who can write Apex AND migrate Flow connections between objects
can potentially combine automation layers to bypass business logic or access
controls in non-obvious ways.

### 2.4 PermissionsManageIPAddresses + PermissionsManageUsers

Managing IP restrictions AND managing users allows an attacker (or insider
threat) to whitelist their own IP and then create accounts for persistent access.

---

## Tier 3: Elevated-Risk Profiles for Regular Review

The following profiles/permission set combinations should be reviewed quarterly
regardless of whether a specific toxic combination is present:

| Profile / Permission Set | Review Frequency | Why |
|--------------------------|-----------------|-----|
| System Administrator | Quarterly | Highest privilege; any user is a critical risk |
| Profiles with ModifyAllData | Quarterly | Mass data write capability |
| Profiles with ViewAllData | Semi-annually | Mass data read capability |
| Integration user profiles | Quarterly | API access; no interactive MFA |
| Guest User profile | Monthly | Public-facing; controls unauthenticated access |

---

## Guest User Profile: Special Handling

The Salesforce Guest User profile controls access for unauthenticated visitors
to Experience Cloud sites <!-- verify-before-merge:2026-05-21 --> and Salesforce Embedded Service.

### Required Restrictions

```
Guest User profile must NOT have:
  [ ] API Enabled
  [ ] View All Data
  [ ] Modify All Data
  [ ] Export Reports
  [ ] Access to any custom object with sensitive data (PHI, financial, PII)

Guest User profile OWD access:
  [ ] Guest User visibility settings reviewed annually
  [ ] Objects accessible by Guest User limited to genuinely public content
```

### Query for Guest User Profile Users
```sql
SELECT Id, Name, Profile.Name, Profile.UserLicense.Name
FROM User
WHERE Profile.UserLicense.Name = 'Guest User'
  AND IsActive = true
```

---

## Broad Profile Assignments

Beyond specific permission combinations, a finding category is "excessive
users on high-privilege profiles."

### Thresholds

| Profile Name | Acceptable User Count | Action if Exceeded |
|-------------|----------------------|-------------------|
| System Administrator | < 5 | Immediate remediation |
| Standard User with ModifyAllData | 0 (this is System Admin) | Report as misconfiguration |
| Custom admin-equivalent profile | < 10 | Review and justify each |
| Read-only integration profile | No limit | Ensure no write permissions |

**Query:**
```sql
SELECT Profile.Name, COUNT(Id) userCount
FROM User
WHERE IsActive = true
GROUP BY Profile.Name
ORDER BY COUNT(Id) DESC
LIMIT 30
```

---

## Toxic Combination Detection Script (Anonymous Apex)

```apex
// Report users with both ModifyAllData and ManageUsers (non-sys-admin)
List<User> riskyUsers = [
    SELECT Id, Name, Username, Profile.Name,
           Profile.PermissionsModifyAllData,
           Profile.PermissionsManageUsers,
           Profile.PermissionsApiEnabled,
           Profile.PermissionsViewAllData
    FROM User
    WHERE IsActive = true
      AND Profile.Name != 'System Administrator'
      AND (
        (Profile.PermissionsModifyAllData = true AND Profile.PermissionsManageUsers = true)
        OR
        (Profile.PermissionsModifyAllData = true AND Profile.PermissionsApiEnabled = true)
        OR
        (Profile.PermissionsViewAllData = true AND Profile.PermissionsApiEnabled = true)
      )
];

for (User u : riskyUsers) {
    System.debug(
        'RISK FINDING - User: ' + u.Name +
        ' | Profile: ' + u.Profile.Name +
        ' | ModAll: ' + u.Profile.PermissionsModifyAllData +
        ' | ManageUsers: ' + u.Profile.PermissionsManageUsers +
        ' | API: ' + u.Profile.PermissionsApiEnabled
    );
}
System.debug('Total risky users found: ' + riskyUsers.size());
```
