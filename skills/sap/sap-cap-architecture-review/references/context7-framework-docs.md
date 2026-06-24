# Context7 framework docs — SAP CAP Architecture Review

**Role**: supplementary. Official SAP CAP documentation at cap.cloud.sap is the primary source for all architecture guidance. Context7-sourced CAP documentation supplements with code-level examples and implementation detail for authorization, multitenancy, and draft patterns.

**Library used**: SAP Cloud Application Programming Model (CAP)
Context7 library ID: `/websites/cap_cloud_sap`
Lookup targets: CDS service authorization (@requires/@restrict), multitenancy extensibility, CAP draft handling, CDS modeling patterns
Skill: `sap-cap-architecture-review`
Classification: supplementary — strongly applies for this skill

---

## CAP @requires — service-level authorization (supplementary)

Source: cap.cloud.sap (Context7 `/websites/cap_cloud_sap`)
Reference: https://cap.cloud.sap/docs/guides/security/remote-authentication

The `@requires` annotation on a CDS service restricts access to callers holding a specified role. This works for both user-facing and App-2-App (technical user) flows.

```cds
using { sap.capire.flights.data as data } from './data-service';

annotate data with @(requires: 'data-consumer');
```

A service with no `@requires` annotation is accessible without role restriction. In App-2-App IAS flows, the role is derived from the IAS API identifier registered with the service.

**Relevance**: Use this pattern to assess whether externally exposed CAP services have role-based guards and to classify services missing `@requires` as critical or high findings depending on exposure scope.

---

## CAP @restrict — entity-level fine-grained access control (supplementary)

Source: cap.cloud.sap (Context7 `/websites/cap_cloud_sap`)
Reference: https://cap.cloud.sap/docs/guides/security/authorization

`@restrict` provides operation-level, role-specific, and instance-level access control on CDS service entities and actions. It is preferred over `@requires` for entities containing sensitive business data.

```cds
service CustomerService @(requires: 'authenticated-user') {
  entity Products @(restrict: [
    { grant: 'READ' },
    { grant: 'WRITE', to: 'Vendor' },
    { grant: 'addRating', to: 'Customer'}
  ]) {/*...*/}
  actions {
     action addRating (stars: Integer);
  }
  entity Orders @(restrict: [
    { grant: '*', to: 'Customer', where: (CreatedBy = $user) }
  ]) {/*...*/}
  action monthlyBalance @(requires: 'Vendor') ();
}
```

Key patterns:
- `grant: 'READ'` without `to:` allows any authenticated user to read (matches service-level `@requires`).
- `grant: 'WRITE', to: 'Vendor'` restricts write operations to the `Vendor` role.
- `where: (CreatedBy = $user)` restricts instance access — each customer sees only their own orders.
- CAP runtime enforces `@restrict` server-side before any resource access.

**Relevance**: The central annotation model for entity-level security review. Use to assess whether `where` clauses are present for tenant-scoped or user-scoped data, whether write operations are role-restricted, and whether actions have explicit authorization guards.

---

## CAP default authorization stance (supplementary)

Source: cap.cloud.sap (Context7 `/websites/cap_cloud_sap`)
Reference: https://cap.cloud.sap/docs/guides/security/data-protection

> By default, CAP services and entities are not authorized, requiring developers to design and test access rules.

The CAP runtime guarantees server-side enforcement only after authorization annotations are explicitly declared. The absence of `@requires` or `@restrict` on a service or entity means no authorization enforcement — not a default-deny posture.

**Relevance**: This is the foundational reason why missing annotations are a critical or high finding. The default is permissive, not restrictive. Every service and entity exposed to external callers must have explicit annotation coverage.

---

## Scope boundaries for Context7 usage

Context7 CAP documentation applies to all CAP review domains in this skill:

- **Authorization** (`@requires`, `@restrict`, `where` clauses): directly applicable
- **CDS Modeling** (entity/service definitions, projection patterns): directly applicable
- **Multitenancy** (MTX wiring, extensibility activation): applicable for `cds add extensibility` and `@sap/cds-mtxs` patterns
- **Draft Handling**: applicable for `draftActivate` and `BeforeSave` handler patterns
- **Testing**: applicable for `cds.test` setup and mock authentication patterns

Always label Context7-sourced guidance as `context7-supplementary` in responses. For runtime version-specific behavior (e.g., `@sap/cds` 7.x vs 8.x), direct the user to verify against the official cap.cloud.sap changelog.
