# Workflow and output contract

Use this reference only when performing a full Backstage Scaffolder template review, producing implementation guidance, triaging a scaffolder security incident, or completing a production-readiness pass.

## Review domains

Check these areas before giving a verdict:

- Template `metadata.name`, `spec.owner`, and namespace scoping
- Each `steps:` entry: action type, input parameters, and provisioning blast radius
- Input `parameters:` schema: type enforcement, `maxLength`, `pattern`, `enum`, and data-flow into step inputs
- RBAC permission gate: presence and scope of `@backstage/plugin-permission-backend` policies for this template
- Integration secret scope: GitHub PAT, Azure DevOps token, or other credential used by `publish:*` actions
- `catalog:register` usage: whether registered YAML is user-supplied or template-controlled
- `output:` stanza: whether plaintext secrets or credentials are surfaced in the Backstage UI

## Safe workflow

1. **Frame scope**
   - Template name and `spec.owner`:
   - Target environment (dev / staging / production):
   - Backstage version and active plugins:
   - Whether `@backstage/plugin-permission-backend` is installed:
   - Required outcome of this review:
   - Explicit non-goals:

2. **Collect evidence**
   - Prefer user-provided sanitized Template YAML as primary evidence.
   - Confirm Backstage version and installed plugins from `app-config.yaml` or Backstage `package.json`.
   - Label each finding as `user-provided evidence`, `documentation-based`, or `inference`.

3. **Map action blast radius**
   For each `steps[].action`, ask:
   ```
   - What external system does this action write to?
   - What credential does it use and what is that credential's scope?
   - Is there an RBAC permission policy gating this template for that action?
   - Can a user-controlled input reach this action unsanitized?
   ```
   Example: `publish:github` with `repoUrl: ${{ parameters.repoName }}` where `repoName` has no `pattern`
   validation — a value like `../../../sensitive-repo` could traverse the expected org boundary.

4. **Validate input parameter schema**
   Check each parameter field:
   ```yaml
   parameters:
     - title: Repository Name
       properties:
         repoName:
           type: string
           # REQUIRED: maxLength to prevent oversized inputs
           maxLength: 63
           # REQUIRED: pattern to block path traversal and injection
           pattern: '^[a-z0-9][a-z0-9-]{0,61}[a-z0-9]$'
   ```
   Missing `maxLength` or `pattern` on fields that flow into `publish:github.repoUrl`,
   `roadiehq:utils:fs:write`, or shell-exec actions is a HIGH finding.

5. **Check RBAC permission gate**
   A permission policy protecting a Terraform-provisioning template looks like:
   ```typescript
   // packages/backend/src/plugins/permission.ts
   if (
     isPermission(request.permission, scaffolderTemplateRules.instantiateTemplate)
   ) {
     if (request.credentials.principal.type === 'user') {
       const groups = await catalogClient.getEntities({
         filter: { kind: 'Group', 'spec.members': request.credentials.principal.userEntityRef }
       });
       const isPlatformEngineer = groups.items.some(g => g.metadata.name === 'platform-engineers');
       return { result: isPlatformEngineer ? AuthorizeResult.ALLOW : AuthorizeResult.DENY };
     }
   }
   ```
   If no policy like this exists for infrastructure-provisioning templates, flag as CRITICAL.

6. **Assess integration secret scope**
   Examine the Backstage `integrations:` config that the template's `publish:*` action uses:
   ```yaml
   # app-config.yaml
   integrations:
     github:
       - host: github.com
         token: ${GITHUB_TOKEN}  # scope: repo (read/write all repos in org)
   ```
   A token with `repo` scope on all org repos means any template using `publish:github`
   can write to any repo in the org. Prefer a scoped GitHub App with per-repo installation.

7. **Review catalog:register usage**
   ```yaml
   steps:
     - id: register
       action: catalog:register
       input:
         repoContentsUrl: ${{ steps['publish'].output.repoContentsUrl }}
         catalogInfoPath: '/catalog-info.yaml'
   ```
   If `catalogInfoPath` or the registered YAML content is user-controlled (not template-generated),
   it can inject arbitrary `spec.owner`, `spec.lifecycle`, or `metadata.annotations` values
   into the catalog — overwriting existing entities' ownership metadata. Flag as MEDIUM.

8. **Inspect output stanza**
   ```yaml
   output:
     links:
       - title: Repository
         url: ${{ steps['publish'].output.remoteUrl }}
     # HIGH: do not surface generated credentials here
     # - title: Database password
     #   url: ${{ steps['create-db'].output.password }}
   ```
   Any `output:` value that contains a generated password, API key, connection string,
   or bearer token is a HIGH finding — it persists in the Backstage task log in plaintext.

9. **Recommend the smallest safe action**
   - Prefer narrowing input validation before adding RBAC, as validation is deploy-free.
   - For RBAC gaps, provide the minimum permission policy snippet.
   - If the safest action is to quarantine the template (mark it `spec.lifecycle: deprecated`
     and alert the platform team), say that plainly.

## Validation commands

```bash
# List all templates in the catalog
kubectl get templates -n backstage --all-namespaces

# Inspect a specific template
kubectl get template <name> -n backstage -o yaml

# Check whether permission backend plugin is present
grep -r 'plugin-permission-backend' packages/backend/package.json

# List Backstage integrations config (sanitize before sharing)
grep -A5 'integrations:' app-config.yaml

# Enumerate templates with no permission policy annotation
kubectl get templates -A -o json | jq '.items[] | select(.metadata.annotations["backstage.io/permission-policy"] == null) | .metadata.name'
```

## Output contract

Return this structure:

```markdown
# Backstage Scaffolder Template Review: <template-name>

## Executive verdict
- Status: SAFE / SAFE WITH RISKS / NOT SAFE / NEEDS EVIDENCE
- Biggest risk:
- Evidence level:

## Scope and assumptions
- Template name and owner:
- Backstage version:
- Permission backend installed:
- Confirmed:
- Unknown:
- Out of scope:

## Findings

| Severity | Field / Step | Finding | Evidence | Why it matters | Minimum safe action |
|---|---|---|---|---|---|

## Action blast radius summary

| Step ID | Action | Blast radius | RBAC gated? |
|---|---|---|---|

## Recommended actions
1. <action> — owner: <owner>, validation: <check>, rollback: <rollback>

## Validation
- Commands or checks:
- Expected result:

## Residual risk
- <risk or explicit none>
```
