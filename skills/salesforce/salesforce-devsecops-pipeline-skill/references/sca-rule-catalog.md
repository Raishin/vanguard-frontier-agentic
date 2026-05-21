# SCA Rule Catalog Reference

Static code analysis rules for Apex (PMD), Lightning Web Components (ESLint),
and JavaScript dependency scanning (RetireJS / npm audit) in Salesforce DevSecOps pipelines.

---

## PMD Rules for Apex

PMD is the standard static analysis tool for Apex. Rules are organized by
severity. The Salesforce DX CLI plugin `@salesforce/plugin-code-analyzer`
wraps PMD and can run these checks in pipeline steps.

### Critical Rules (block merge)

| Rule | Category | What It Catches |
|------|----------|----------------|
| `AvoidDmlStatementsInLoops` | Performance | DML inside for/while loops |
| `AvoidSoqlInLoops` | Performance | SOQL inside for/while loops |
| `ApexCRUDViolation` | Security | Missing object/field CRUD checks before DML |
| `ApexSharingViolations` | Security | Classes without `with sharing` declaration |
| `ApexSOQLInjection` | Security | Dynamic SOQL concatenation with user input |
| `ApexXSSFromURLParam` | Security | URL parameter inserted into page output |
| `ApexOpenRedirect` | Security | User-controlled redirect targets |

### High Rules (flag for review)

| Rule | Category | What It Catches |
|------|----------|----------------|
| `AvoidGlobalModifier` | Best Practice | Global scope on classes/methods |
| `ExcessiveClassLength` | Design | Classes > 1000 lines |
| `ExcessiveMethodLength` | Design | Methods > 100 lines |
| `CyclomaticComplexity` | Design | Complexity > 10 in a method |
| `NcssMethodCount` | Design | Non-comment source statements > 40 |
| `EmptyCatchBlock` | Error Prone | `catch(Exception e) {}` swallows errors |
| `EmptyStatementBlock` | Error Prone | Empty if/try/catch blocks |
| `AvoidHardcodingId` | Best Practice | 15/18-char Salesforce IDs in code |
| `UnusedLocalVariable` | Best Practice | Variables declared but never read |
| `OperationWithLimitsInLoop` | Performance | Limits checks inside loops |

### Medium Rules (advisory)

| Rule | Category | What It Catches |
|------|----------|----------------|
| `DebugsShouldUseLoggingLevel` | Best Practice | `System.debug` without LoggingLevel |
| `ApexDoc` | Documentation | Public methods without ApexDoc comments |
| `OneDeclarationPerLine` | Code Style | Multiple variable declarations on one line |
| `FieldDeclarationsShouldBeAtStart` | Code Style | Fields declared mid-class |

### Running PMD in CI

```bash
# Salesforce Code Analyzer CLI plugin
sf scanner run \
  --target "force-app/main/default/classes/**/*.cls" \
  --engine pmd \
  --ruleset "category/apex/security.xml,category/apex/performance.xml" \
  --severity-threshold 2 \
  --format csv \
  --outfile scan-results.csv

# Exit code > 0 if violations at or above threshold
```

Custom ruleset XML for project-specific exclusions:
```xml
<?xml version="1.0" encoding="UTF-8"?>
<ruleset name="Project Custom Ruleset"
  xmlns="http://pmd.sourceforge.net/ruleset/2.0.0">
  <description>Project-specific PMD configuration</description>
  <rule ref="category/apex/security.xml">
    <exclude name="ApexSuggestUsingNamedCredential"/> <!-- named creds not yet available -->
  </rule>
  <rule ref="category/apex/performance.xml"/>
</ruleset>
```

---

## ESLint Rules for LWC

The `@salesforce/eslint-config-lwc`
package provides recommended rules.

### Security-Critical ESLint Rules

| Rule | Severity | What It Catches |
|------|----------|----------------|
| `no-eval` | Error | Dynamic code execution via `eval` |
| `no-implied-eval` | Error | `setTimeout('string')` patterns |
| `no-inner-declarations` | Error | Functions declared inside blocks |
| `@locker/locker/distorted-xml-http-request-never-use-inner-html` | Error | XSS via innerHTML |
| `@salesforce/lwc/no-document-query` | Error | `document.querySelector` bypassing shadow DOM |

### LWC-Specific Rules

| Rule | Severity | What It Catches |
|------|----------|----------------|
| `@salesforce/lwc/no-api-reassignments` | Error | Mutating `@api` properties directly |
| `@salesforce/lwc/no-deprecated` | Warning | Deprecated LWC APIs |
| `@salesforce/lwc/no-leading-uppercase-api-name` | Error | API naming violations |
| `@salesforce/lwc/no-template-children` | Error | Accessing `this.template.children` |
| `@salesforce/lwc/consistent-component-name` | Warning | File name / class name mismatch |

### ESLint Configuration (`.eslintrc.json`)

```json
{
  "extends": ["@salesforce/eslint-config-lwc/recommended"],
  "rules": {
    "no-eval": "error",
    "no-implied-eval": "error",
    "@salesforce/lwc/no-document-query": "error"
  },
  "overrides": [
    {
      "files": ["**/__tests__/**"],
      "rules": {
        "@salesforce/lwc/no-document-query": "off"
      }
    }
  ]
}
```

### Running ESLint in CI

```bash
# Install
npm install --save-dev @salesforce/eslint-config-lwc

# Run
npx eslint force-app/main/default/lwc \
  --ext .js \
  --format json \
  --output-file eslint-results.json

# Non-zero exit if errors found
```

---

## RetireJS / npm audit CVE Patterns

### RetireJS

RetireJS scans JavaScript files and `package.json` for known vulnerable
library versions by comparing against a CVE database.

```bash
# Install
npm install -g retire

# Scan a project
retire --path force-app/main/default/staticresources \
  --outputformat json \
  --outputpath retire-results.json \
  --severity medium  # fail on medium and above
```

Common vulnerable patterns found in Salesforce static resources:

| Library | Common Issue | Safe Version Floor |
|---------|-------------|-------------------|
| jQuery | `$.parseHTML` XSS, `$` selector injection | >= 3.5.0 |
| Moment.js | ReDoS in `moment.format` | >= 2.29.4 (or migrate to Luxon) |
| Handlebars.js | Prototype pollution | >= 4.7.7 |
| Lodash | Prototype pollution via `merge`/`defaultsDeep` | >= 4.17.21 |
| Underscore.js | Prototype pollution | >= 1.13.0 |

### npm audit

```bash
# For projects with package.json (e.g., SFDX project tools, Jest setup)
npm audit --audit-level=moderate --json > npm-audit-results.json

# Fail CI pipeline on high/critical
npm audit --audit-level=high
```

---

## Pipeline Integration Reference

### GitHub Actions example

```yaml
name: SCA Gate
on: [pull_request]
jobs:
  sca:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Setup Node
        uses: actions/setup-node@v4
        with:
          node-version: '20'
      - name: Install Salesforce CLI
        run: npm install -g @salesforce/cli
      - name: Install Code Analyzer
        run: sf plugins install @salesforce/plugin-code-analyzer
      - name: Run PMD
        run: |
          sf scanner run \
            --target "force-app/**/*.cls" \
            --engine pmd \
            --severity-threshold 2 \
            --format github-actions
      - name: Run ESLint
        run: |
          npm ci
          npx eslint force-app/main/default/lwc --ext .js
      - name: RetireJS
        run: |
          npm install -g retire
          retire --path force-app/main/default/staticresources --severity high
```

### Severity Threshold Guidance

| Pipeline Stage | Recommended Threshold |
|---------------|----------------------|
| Pull Request (feature branch) | Fail on Critical; Warn on High |
| Merge to main/develop | Fail on Critical and High |
| Production deployment | Fail on Critical, High, and Medium |
