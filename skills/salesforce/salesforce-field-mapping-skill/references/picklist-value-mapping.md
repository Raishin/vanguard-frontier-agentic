# Picklist Value Mapping Reference

## Key Rules

1. **Case-sensitive:** `"Closed Won"` ≠ `"closed won"` in Data Loader
2. **API value vs. display label:** These can differ in custom picklists;
   always use the API value (also called the "internal value")
3. **Multi-select separator:** Semicolon (`;`) in Data Loader imports
4. **Inactive values:** Existing records with inactive picklist values retain
   them, but new records cannot be assigned inactive values

---

## Standard Picklist Values by Field

### LeadSource (Contact, Lead, Opportunity)

```
Cold Call
Web
Phone Inquiry
Partner Referral
Purchased List
Other
Word of mouth
Employee
Internal
Partner
```

### StageName (Opportunity)

Default Salesforce values (orgs commonly customize these):
```
Prospecting
Qualification
Needs Analysis
Value Proposition
Id. Decision Makers
Perception Analysis
Proposal/Price Quote
Negotiation/Review
Closed Won
Closed Lost
```

### Industry (Account, Lead)

```
Agriculture
Apparel
Banking
Biotechnology
Chemicals
Communications
Construction
Consulting
Education
Electronics
Energy
Engineering
Entertainment
Environmental
Finance
Food & Beverage
Government
Healthcare
Hospitality
Insurance
Machinery
Manufacturing
Media
Not For Profit
Recreation
Retail
Shipping
Technology
Telecommunications
Transportation
Utilities
Other
```

### Rating (Lead, Account)

```
Hot
Warm
Cold
```

### Lead Status (Lead)

```
Open - Not Contacted
Working - Contacted
Closed - Converted
Closed - Not Converted
```

### Account Type (Account)

```
Prospect
Customer - Direct
Customer - Channel
Channel Partner / Reseller
Installation Partner
Technology Partner
Other
```

### Case Status (Case)

```
New
Working
Escalated
Closed
```

### Case Priority (Case)

```
Low
Medium
High
```

### Case Origin (Case)

```
Phone
Email
Web
```

### Salutation (Contact, Lead)

```
Mr.
Ms.
Mrs.
Dr.
Prof.
```

---

## Dependent Picklist Concepts

A **dependent picklist** shows only the values valid for the currently
selected controlling field value. When mapping:

1. Identify if the source system has both controlling and dependent values
2. Map controlling field first, then dependent field
3. Verify that source dependent values are valid for the controlling value
   in the target org — invalid combinations are silently blanked or rejected

**Example:** Type = "Partner" controls Subtype picklist. If source has
Type = "Reseller" (valid in source, invalid in target), the Subtype
value will also be invalid.

---

## Multi-Select Picklist Value Normalization

### Separator Conversion

| Source separator | Action |
|---|---|
| Comma (`,`) | Replace with semicolon (`;`) |
| Pipe (`\|`) | Replace with semicolon (`;`) |
| Semicolon (`;`) | Already correct for Data Loader |
| Space-separated | Ambiguous — check if values contain spaces; use manual review |

### Value Case Normalization Table (example)

| Source value | Salesforce API value |
|---|---|
| `technology` | `Technology` |
| `TECHNOLOGY` | `Technology` |
| `Financial Services` | May be `Financial Services` — verify against org picklist |
| `fin-serv` | Unknown — requires business mapping decision |

---

## Mapping Source CRM Values to Salesforce Picklists

### HubSpot Lead Status → Salesforce Lead Status

| HubSpot | Salesforce |
|---|---|
| `new` | `Open - Not Contacted` |
| `open` | `Open - Not Contacted` |
| `in_progress` | `Working - Contacted` |
| `qualified` | `Working - Contacted` |
| `unqualified` | `Closed - Not Converted` |
| `connected` | `Working - Contacted` |
| `bad_timing` | `Closed - Not Converted` |

### Pipedrive Stage → Salesforce Stage

Pipedrive stages are org-specific — no universal mapping. Establish a
manual mapping table from the Pipedrive admin's stage list to Salesforce
`StageName` values. Flag this as a REQUIRED business decision in the output.

### Pipedrive Priority → Salesforce Priority/Rating

| Pipedrive | Salesforce |
|---|---|
| `3` (high) | `Hot` |
| `2` (medium) | `Warm` |
| `1` (low) | `Cold` |

---

## Inactive Picklist Values

When source data contains values that exist in Salesforce but are marked
inactive:

1. Flag the inactive values in the mapping output
2. Options:
   - Reactivate the value before import
   - Map to a currently active equivalent value
   - Import with a custom `Other` or catch-all value and note for review

Inactive picklist values will cause Data Loader validation errors on
insert/upsert operations if the field has "Restrict picklist to the values
defined in the value set" enabled.

---

## Custom Picklist Field Values

For custom picklist fields, the admin must provide the exact API values
from Setup → Object Manager → Fields → Picklist Options. Labels and API
values may differ.

When custom picklist values are unknown:
1. Note in `unmapped_source_columns` with reason `"custom picklist values unknown"`
2. Recommend using `salesforce-metadata-fetcher-skill` to retrieve the
   picklist value set from a live org
3. Provide a placeholder mapping with a `<!-- verify-values -->` annotation
