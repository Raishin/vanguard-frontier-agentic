> Static review only. Scope: Spring Security 6.x lambda DSL — `SecurityFilterChain` `@Bean` methods, `HttpSecurity.authorizeHttpRequests(...)`, `securityMatcher(s)`. Does **not** cover the removed `WebSecurityConfigurerAdapter` / `.authorizeRequests()` API from Security 5.x and earlier; if the source still extends `WebSecurityConfigurerAdapter`, flag that the codebase needs a version-specific migration review rather than applying 6.x matcher rules to it verbatim. Anchor findings to the official Spring Security reference (`docs.spring.io/spring-security/reference/`), not to a single changelog entry or blog post.

## 1. Multiple `SecurityFilterChain` beans

**Dangerous:** two or more `@Bean SecurityFilterChain` methods whose `securityMatcher`/`securityMatchers` can both apply to the same incoming request, with no explicit `@Order` (or an `Ordered`-implementing `@Configuration` class) distinguishing precedence.

```java
@Bean
SecurityFilterChain apiChain(HttpSecurity http) throws Exception {
    http.securityMatcher("/api/**")
        .authorizeHttpRequests(auth -> auth.anyRequest().authenticated());
    return http.build();
}

@Bean
SecurityFilterChain adminChain(HttpSecurity http) throws Exception {
    // overlaps /api/** with no @Order on either bean
    http.securityMatcher("/api/admin/**")
        .authorizeHttpRequests(auth -> auth.anyRequest().hasRole("ADMIN"));
    return http.build();
}
```

Which chain actually processes `/api/admin/**` in this shape is not something a static read can determine with confidence — the reference is explicit that ordering across multiple chains must be controlled deliberately. Treat the absence of `@Order` on either bean as the defect itself, and mark any claim about which chain "wins" as `inference (partial source)` unless the source shows an explicit order.

**Safe:** disjoint `securityMatcher` patterns (e.g. `/api/admin/**` and `/api/**` given `@Order(1)` and `@Order(2)` respectively, narrowest first), or a single chain with layered `authorizeHttpRequests` rules instead of multiple beans.

## 2. `authorizeHttpRequests` matcher ordering

Rules are evaluated **first-match-wins** in declaration order within one `authorizeHttpRequests` block.

**Safe:**
```java
http.authorizeHttpRequests(auth -> auth
    .requestMatchers("/api/public/**").permitAll()
    .requestMatchers("/api/admin/**").hasRole("ADMIN")
    .anyRequest().authenticated()
);
```

**Dangerous — shadowing:**
```java
http.authorizeHttpRequests(auth -> auth
    .requestMatchers("/api/**").authenticated()   // broader rule declared first
    .requestMatchers("/api/public/**").permitAll() // shadowed — never reached
    .anyRequest().authenticated()
);
```
The second `requestMatchers` call is unreachable: every `/api/public/**` request already matched the first, broader rule. This is a fail-open risk when the shadowing runs the other direction (a broad `permitAll` declared before a narrower `hasRole` rule) and a fail-closed/dead-code defect when it runs this direction.

**Structural guard worth noting (not a substitute for reviewing order):** Spring Security's `AuthorizationManagerRequestMatcherRegistry` throws `IllegalStateException` ("Can't configure requestMatchers after anyRequest") if `requestMatchers(...)` is called after `anyRequest()` in the same block — so a rule placed after `anyRequest()` typically fails fast at application startup rather than silently existing as dead code. Still flag it if seen in source, since it indicates the author does not understand the ordering contract, and pre-startup review value comes before this fails at deploy time.

## 3. Method security vs. request-level authorization

When a controller method is reachable both through `authorizeHttpRequests` (request-level) and `@PreAuthorize`/`@PostAuthorize`/`@Secured`/`@RolesAllowed` (method-level), the two controls are independent evaluations — do not assume the stricter one governs. Compare the actual expressions: `hasRole("ADMIN")` at the request level and `@PreAuthorize("hasAuthority('SCOPE_read')")` at the method level are two different, non-redundant checks (both apply, AND semantics) and can be credited as defense-in-depth. But `authenticated()` at the request level with `@PreAuthorize("isAuthenticated()")` at the method level is the same check twice — the weaker (here, identical) condition is the real gate, and removing either one changes nothing.

`@PostAuthorize` evaluates **after** the method body executes. On a query it is fine (denial just discards the return value). On anything that persists, deletes, publishes an event, or calls another service, the side effect has already occurred by the time `@PostAuthorize` could deny — this is always a defect on a mutating method, not a style preference.

## 4. `AuthorizationManager` delegation

Custom `AuthorizationManager<T>` (or `AuthorityAuthorizationManager`/`AuthorizationManagers.anyOf`/`.allOf` composition) must resolve to `AuthorizationDecision(false)` (deny) on any unhandled input, thrown exception, or missing/anonymous `Authentication` — never fall through to an implicit grant. Read the full method body, not just the happy path, before crediting it as fail-closed; a `switch`/`if` chain with no final `else` returning deny, or a caught exception that returns `new AuthorizationDecision(true)`, is a critical fail-open defect.

## 5. CSRF for state-changing endpoints

CSRF protection defends session-cookie-authenticated browser clients against cross-site request forgery on state-changing (non-idempotent, non-safe-method) requests. `csrf(AbstractHttpConfigurer::disable)` is standard and correct for a genuinely stateless API authenticated by a bearer token, mTLS, or a signed header with no session cookie in play — but only when the source actually shows `SessionCreationPolicy.STATELESS` and a non-cookie credential. Disabling CSRF on a chain that also configures form login, `httpBasic()` with browser use, or any cookie-based session is a defect regardless of how common the disable line looks in tutorials.

## Known uncertainty

Which `SecurityFilterChain` bean Spring actually selects at runtime when ordering is ambiguous, and the precise interaction of `@Order` with component-scan discovery order, are runtime facts this static review cannot observe directly — always state the ambiguity as the finding ("ordering is not guaranteed by source alone") rather than asserting which chain wins.
