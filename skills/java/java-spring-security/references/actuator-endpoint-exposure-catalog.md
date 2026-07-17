> Static review only. Scope: Spring Boot 3.x Actuator (`management.endpoints.web.exposure.*` properties, `org.springframework.boot.actuate.autoconfigure.security.servlet.EndpointRequest`). Anchor findings to the official Spring Boot Actuator reference (`docs.spring.io/spring-boot/reference/actuator/`), not to a remembered default from a specific patch release. **Known uncertainty:** the exact set of endpoints exposed over HTTP by default, and which endpoints are enabled-by-default vs. opt-in, has changed across Spring Boot minor versions historically. Do not assert a specific default exposure set from memory — verify against the project's declared Spring Boot version and the corresponding reference page, or mark the default-exposure claim `assumption (source absent)` and ask the user to confirm the Boot version and effective properties.

## 1. `management.endpoints.web.exposure.include`

**Dangerous:**
```yaml
management:
  endpoints:
    web:
      exposure:
        include: "*"
```
This opts every actuator endpoint into HTTP exposure, including operationally sensitive ones (`env`, `heapdump`, `threaddump`, `beans`, `configprops`, `loggers`, `shutdown` if separately enabled, `mappings`). Whether this is a critical finding depends entirely on whether the exposed path is then fenced by Spring Security — treat the wildcard include as the trigger to go check for that fence, not as an automatic critical on its own (though in practice an unfenced wildcard include is almost always the finding).

**Safer:** an explicit, minimal include list (e.g. `health,info,metrics`) scoped to what operators actually need over HTTP, with anything more sensitive reserved for a JMX-only or internal-only exposure path.

## 2. Securing `/actuator` with Spring Security

**Safe pattern** — require authentication (and typically a specific authority) for the actuator base path using `EndpointRequest`:
```java
http.authorizeHttpRequests(auth -> auth
    .requestMatchers(EndpointRequest.to("health", "info")).permitAll()
    .requestMatchers(EndpointRequest.toAnyEndpoint()).hasRole("ACTUATOR_ADMIN")
    .anyRequest().authenticated()
);
```
`EndpointRequest.toAnyEndpoint()` matches the actuator base path regardless of its configured `management.endpoints.web.base-path`, which is more robust than hand-writing an Ant pattern like `/actuator/**` (a hand-written pattern silently stops matching if the base path is customized — flag a hardcoded `/actuator/**` matcher as fragile even when currently correct).

**Dangerous:** no actuator-specific rule at all, relying on a general `anyRequest().authenticated()` — this still requires authentication but grants it to *any* authenticated principal, not specifically an operator/admin role; treat lack of role-scoping on sensitive endpoints (`env`, `heapdump`, `shutdown`) as a defect distinct from lack of authentication entirely.

## 3. Sensitive endpoints requiring the strictest posture

- `env` / `configprops` — can reveal configuration values, and depending on Spring Boot's sanitization configuration, potentially secrets if sanitization has been weakened (`management.endpoint.env.show-values` / a custom `SanitizingFunction`). Flag any override that widens what `env` reveals.
- `heapdump` / `threaddump` — can leak in-memory secrets, session tokens, or PII.
- `shutdown` — remotely stops the application; verify `management.endpoint.shutdown.enabled` is not turned on without the endpoint being strictly authenticated and role-scoped (this endpoint is opt-in, not on by default — but a source that explicitly enables it demands the tightest possible authorization check).
- `beans` / `mappings` — reveal internal application structure useful for further attack reconnaissance.
- `loggers` — allows remotely changing log levels, which can be used to suppress security-relevant logging.

## 4. Separate management port

When `management.server.port` is set to a different port than the main application, actuator endpoints are served by a **separate** embedded server context and are **not** covered by the main application's `SecurityFilterChain` / `EndpointRequest`-based rules — the reference documents that a management-port setup requires its own security configuration. Treat a project that sets `management.server.port` without a corresponding management-specific security configuration as an unfenced-exposure finding, and mark the actual network reachability of that port (is it bound to loopback, an internal interface, or `0.0.0.0`?) as an open question requiring the user's confirmation — that is deployment/network fact this static review cannot observe.

## 5. What this review cannot confirm

Static review cannot verify the actually-running exposure set, the real network reachability of the actuator port, or whether an infrastructure-level control (a reverse proxy or network policy blocking `/actuator/**` externally) compensates for an otherwise-unfenced configuration. State any such compensating-control claim as `assumption (source absent)` unless the user supplies the infrastructure configuration as part of the review.
