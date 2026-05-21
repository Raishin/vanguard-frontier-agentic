# API Name Normalization Reference

## How Salesforce Derives API Names from Labels

Salesforce applies the following transformations when you create a field
and it auto-generates the API name from the field label:

1. **Replace spaces with underscores:** `First Name` → `First_Name`
2. **Remove special characters:** Keep only `[A-Za-z0-9_]`
3. **Remove leading digits:** `2nd_Stage` → `nd_Stage` (add prefix manually)
4. **Collapse consecutive underscores:** `My__Field` → `My_Field`
5. **Truncate to 40 characters** (custom field base name, before `__c`)
6. **Append `__c`** suffix for custom fields
7. **Append namespace prefix** for managed package fields: `ns__Field__c`

## Reserved Words

These cannot be used as custom field API names (without modification):

| Reserved | Notes |
|---|---|
| `Id` | Always the record's 18-char Salesforce ID |
| `Name` | Standard name field on all objects |
| `OwnerId` | Always the owner User ID |
| `CreatedDate` | Auto-populated system field |
| `CreatedById` | Auto-populated system field |
| `LastModifiedDate` | Auto-populated system field |
| `LastModifiedById` | Auto-populated system field |
| `SystemModstamp` | Auto-populated system field |
| `IsDeleted` | Soft-delete flag |
| `RecordTypeId` | Record Type assignment |

If a source column matches a reserved word name, map directly to the
standard field — do not attempt to create a custom field with that name.

## Length Limits

| Component | Limit |
|---|---|
| Custom field base name | 40 characters (before `__c`) |
| Standard field API name | Varies; typically 30–40 characters |
| Namespace prefix + field name | 40 characters total base name |

If a derived API name exceeds 40 characters, truncate to 40 characters
from the left. Flag the truncation in the output.

## Common Source-to-API Name Mappings

### HubSpot to Salesforce Contact

| HubSpot Property | Salesforce API Name | Type |
|---|---|---|
| `firstname` | `FirstName` | Text |
| `lastname` | `LastName` | Text |
| `email` | `Email` | Email |
| `phone` | `Phone` | Phone |
| `mobilephone` | `MobilePhone` | Phone |
| `jobtitle` | `Title` | Text |
| `company` | `Account.Name` (via AccountId) | Lookup |
| `website` | Account `Website` | Text |
| `city` | `MailingCity` | Text |
| `state` | `MailingState` | Text |
| `zip` | `MailingPostalCode` | Text |
| `country` | `MailingCountry` | Text |
| `address` | `MailingStreet` | TextArea |
| `hs_lead_status` | `LeadSource` | Picklist |
| `hubspot_owner_id` | `OwnerId` (requires ID lookup) | Lookup |

### Pipedrive to Salesforce Lead

| Pipedrive Field | Salesforce API Name | Type |
|---|---|---|
| `name` | Requires split to `FirstName` + `LastName` | Text |
| `email[0].value` | `Email` | Email |
| `phone[0].value` | `Phone` | Phone |
| `org_name` | `Company` | Text |
| `title` | `Title` | Text |
| `stage_id` | `LeadSource` (mapped via picklist) | Picklist |
| `owner_name` | `OwnerId` (requires User lookup) | Lookup |

### Excel Export Common Headers

| Excel Header | Likely API Name | Notes |
|---|---|---|
| `First Name` | `FirstName` | Standard |
| `Last Name` | `LastName` | Standard |
| `Email Address` | `Email` | Normalize header |
| `Mobile` | `MobilePhone` | Not `Mobile` alone |
| `Company Name` | `Account.Name` | Requires AccountId lookup |
| `Date Created` | `CreatedDate` | Read-only, cannot import |
| `Modified Date` | `LastModifiedDate` | Read-only, cannot import |
| `Deal Value` | `Amount` | Opportunity only |
| `Expected Close` | `CloseDate` | Required on Opportunity |
| `Stage` | `StageName` | Required on Opportunity |

## Collision Detection

Two source columns collide if they normalize to the same API name.

**Examples:**
- `"Phone Number"` and `"Phone"` both normalize to → `Phone`
- `"First Name"` and `"FirstName"` both normalize to → `FirstName`
- `"My Field"` and `"My_Field"` both normalize to → `My_Field`

When collision is detected:
1. Flag both columns in the output
2. Ask user to clarify which column maps to which field
3. Do not silently drop one column

## Special Character Handling

Source columns with these characters in headers require cleanup before
use in Data Loader:

| Character | Risk | Action |
|---|---|---|
| `#` | Splits CSV parsing | Rename column before Data Loader import |
| `,` | Breaks CSV structure | Rename or quote column header |
| `"` | Breaks CSV quoting | Remove from column header |
| `(` `)` | Removed by normalization | Document the mapping |
| `/` | Path separator risk | Remove |
| `&` | HTML entity risk | Replace with `And` or `_` |
| `.` | Period | Remove or replace with underscore |

## Standard Object Required Fields

Imports will fail without these fields in the source (for insert operations):

| Object | Required Fields |
|---|---|
| Contact | `LastName` |
| Lead | `LastName`, `Company` |
| Account | `Name` |
| Opportunity | `Name`, `StageName`, `CloseDate` |
| Case | `Status` |
| Custom object | Object's required fields per field configuration |

For **upsert operations**, also require: the external ID field used as
the match key.

For **update operations**: `Id` or the external ID field.
