# LWC Security Reference

Security vulnerabilities specific to Lightning Web Components and the
Lightning Platform security model.

---

## 1. XSS in Lightning Web Components

### Lightning Locker and LWS

Lightning Web Security (LWS)
replaced Lightning Locker as the
default security model for LWC. LWS provides DOM isolation via a JavaScript
membrane rather than a full sandbox iframe.

Key properties:
- Third-party scripts that manipulate `window` or `document` directly may break
  under LWS — this is by design.
- `eval`, `new Function`, and dynamic script injection are blocked.
- Cross-namespace DOM access is restricted.

### XSS Risk: `innerHTML` Assignment

```javascript
// VULNERABLE: setting innerHTML from user-supplied data
this.template.querySelector('.content').innerHTML = this.userInput;

// SAFE: use textContent for text-only content
this.template.querySelector('.content').textContent = this.userInput;

// SAFE: use lwc:dom="manual" only when HTML structure is needed
// and sanitize with DOMPurify (self-hosted) before assignment
import DOMPurify from 'c/domPurify';  // if available in your org
const clean = DOMPurify.sanitize(this.userInput, { ALLOWED_TAGS: ['b', 'i', 'em'] });
this.template.querySelector('.content').innerHTML = clean;
```

### `lwc:dom="manual"` Risks

Using `lwc:dom="manual"` opts an element out of LWC's rendering pipeline and
puts DOM management responsibility on the developer.

```html
<!-- Requires careful handling -->
<div lwc:dom="manual" class="rich-content"></div>
```

Rules when using `lwc:dom="manual"`:
- Never assign `element.innerHTML = userControlledString` without sanitization.
- Sanitize all content before insertion.
- Remove event listeners when component is disconnected to prevent memory leaks.

---

## 2. SOQL Injection in LWC (via Apex)

LWC does not execute SOQL directly. The risk is when user input from an LWC
is passed to an Apex method that concatenates it into a SOQL string.

### Vulnerable Pattern
```apex
@AuraEnabled
public static List<Account> searchAccounts(String searchTerm) {
    // VULNERABLE: SOQL injection via string concatenation
    String query = 'SELECT Id, Name FROM Account WHERE Name LIKE \'%' + searchTerm + '%\'';
    return Database.query(query);
}
```

Adversarial input: `%' OR OwnerId != NULL OR Name LIKE '%`
This returns all accounts regardless of filter intent.

### Secure Pattern
```apex
@AuraEnabled
public static List<Account> searchAccounts(String searchTerm) {
    // SAFE: bind variable prevents injection
    String searchPattern = '%' + String.escapeSingleQuotes(searchTerm) + '%';
    return [SELECT Id, Name FROM Account WHERE Name LIKE :searchPattern WITH SECURITY_ENFORCED LIMIT 100];
}
```

Additional hardening:
- `String.escapeSingleQuotes` on any dynamic string used in SOQL.
- Use bind variables (`:variable`) rather than concatenation.
- `WITH SECURITY_ENFORCED` enforces FLS and object-level security at query time.
- `WITH USER_MODE` (API 57.0+)
enforces full user-context security
  including sharing rules, FLS, and CRUD.

---

## 3. Field-Level Security Enforcement in LWC Apex

### Problem
`@AuraEnabled` Apex methods with `with sharing` enforce row-level access but
NOT field-level security. Users can receive field values they lack FLS access
to if the Apex code does not explicitly check FLS.

### Detection
Review all `@AuraEnabled` methods that return SObjects or field values. Check
whether field values are stripped before returning.

### Solutions

**Option A: `WITH SECURITY_ENFORCED` in SOQL**
```apex
@AuraEnabled
public static List<Contact> getContacts {
    // Throws QueryException if FLS blocks any field in SELECT list
    return [SELECT Id, Name, Email, SSN__c FROM Contact WITH SECURITY_ENFORCED LIMIT 50];
}
```

**Option B: `Security.stripInaccessible`**
```apex
@AuraEnabled
public static List<Contact> getContacts {
    List<Contact> rawContacts = [SELECT Id, Name, Email, SSN__c FROM Contact LIMIT 50];
    SObjectAccessDecision decision = Security.stripInaccessible(AccessType.READABLE, rawContacts);
    return (List<Contact>) decision.getRecords;
    // SSN__c is automatically stripped if user lacks read FLS
}
```

---

## 4. Cross-Site Request Forgery (CSRF) in LWC

Salesforce's platform automatically includes anti-CSRF tokens on Visualforce
pages but LWC-to-Apex wire/imperative calls use session cookies that are
same-site by default.

**Risk areas:**
- Custom REST endpoints (`@RestResource`) called by LWC via `fetch`.
- Aura endpoints exposed to unauthenticated access.

**Mitigation:**
- For custom REST endpoints called from LWC, verify the `Origin` header server-side.
- Set `Samesite=Strict` or `Samesite=Lax` on session cookies (configured in
  Setup > Session Settings
).
- Do not expose `@AuraEnabled(cacheable=false)` methods to unauthenticated sites
  without additional CSRF protection.

---

## 5. Secure Wire Adapter Usage

The `@wire` decorator fetches data reactively. Misuse can cause excessive data
exposure or SOQL limit exhaustion.

```javascript
// SAFE: wire with specific ID, not open-ended query
import { LightningElement, wire, api } from 'lwc';
import { getRecord, getFieldValue } from 'lightning/uiRecordApi';
import NAME_FIELD from '@salesforce/schema/Account.Name';

export default class AccountDetail extends LightningElement {
    @api recordId;

    @wire(getRecord, { recordId: '$recordId', fields: [NAME_FIELD] })
    account;

    get name {
        return getFieldValue(this.account.data, NAME_FIELD);
    }
}
```

**Anti-patterns with @wire:**
- Using `@wire` to fetch unbounded lists (no LIMIT or filter) — use Apex with
  server-side pagination instead.
- Passing `recordId` from URL parameters directly to a wire without validating
  the ID format (18-character Salesforce ID: `[a-zA-Z0-9]{18}`).
- Exposing a wire result directly in the template without null checks.

---

## 6. Content Security Policy Compliance

Experience Cloud sites
and Embedded Service can apply CSP.
LWC components must comply:

- Do not load scripts from external CDNs inline; use Static Resources.
- Do not use `eval` or `setTimeout(string)`.
- If using third-party libraries, host them in Static Resources and declare
  their origin in CSP Trusted Sites (Setup > CSP Trusted Sites).
- Images fetched via `fetch` from external sources require the domain listed
  in CSP `img-src`.

---

## LWC Security Review Checklist

- [ ] No `innerHTML` assignment from user-controlled data without sanitization.
- [ ] No dynamic SOQL string concatenation in Apex methods called from LWC.
- [ ] All `@AuraEnabled` methods use `with sharing`.
- [ ] FLS is enforced via `WITH SECURITY_ENFORCED` or `Security.stripInaccessible`.
- [ ] No hardcoded Salesforce IDs in component JavaScript.
- [ ] `lwc:dom="manual"` usage documented and reviewed for XSS.
- [ ] External scripts hosted as Static Resources, not loaded from CDN inline.
- [ ] API responses from custom REST endpoints checked for CSRF exposure.
- [ ] Wire adapters include error handling (`account.error` checked alongside `account.data`).
