# OpenTelemetry Web Tracing Wiring

Use this reference only when wiring or reviewing `WebTracerProvider`, `DocumentLoadInstrumentation`/`DocumentLoad`, `FetchInstrumentation`, or exporter configuration for browser-origin traces.

> Version note: `@opentelemetry/sdk-trace-web` and instrumentation package APIs (constructor option shapes, plugin package names) are version-sensitive. Verify against Context7-fetched docs (`/open-telemetry/opentelemetry-js`) and the installed package versions before prescribing exact constructor syntax.

## What people get wrong

The naive story is:

> "I'll create a `WebTracerProvider`, register it, and traces just work."

Wrong. Official OpenTelemetry JS docs imply at least four separate concerns that must each be configured, not assumed:

1. **Context propagation across async boundaries** — the browser's default context manager does not automatically follow promises/timers/event callbacks; `ZoneContextManager` (from `@opentelemetry/context-zone`) is the documented pattern for supporting asynchronous operations in `provider.register()`.
2. **Instrumentation registration is separate from provider creation** — creating a `WebTracerProvider` does not, by itself, produce document-load or fetch/XHR spans. Instrumentations (`DocumentLoad`/`DocumentLoadInstrumentation`, `FetchInstrumentation`, `XMLHttpRequestInstrumentation`) must be explicitly registered via `registerInstrumentations({instrumentations: [...]})`.
3. **Exporter and span-processor choice affects both delivery reliability and payload timing** — `SimpleSpanProcessor` exports each span as it ends (useful for local `ConsoleSpanExporter` debugging); `BatchSpanProcessor` batches and is the documented production pattern for network exporters like OTLP.
4. **Trace-propagation format must match the backend** — the B3 propagator vs. W3C Trace Context are not interchangeable; using the example combination that doesn't match your collector/backend silently breaks distributed trace stitching.

## Officially grounded wiring shape

Minimal document-load + console-export setup (from `@opentelemetry/sdk-trace-web` docs):

```javascript
import {
  ConsoleSpanExporter,
  SimpleSpanProcessor,
  WebTracerProvider,
} from '@opentelemetry/sdk-trace-web';
import { DocumentLoad } from '@opentelemetry/plugin-document-load';
import { ZoneContextManager } from '@opentelemetry/context-zone';
import { registerInstrumentations } from '@opentelemetry/instrumentation';

const provider = new WebTracerProvider({
  spanProcessors: [new SimpleSpanProcessor(new ConsoleSpanExporter())],
});

provider.register({
  contextManager: new ZoneContextManager(),
});

registerInstrumentations({
  instrumentations: [new DocumentLoad()],
});
```

Production-shaped fetch instrumentation + OTLP export (batched):

```javascript
import {
  BatchSpanProcessor,
  WebTracerProvider,
} from '@opentelemetry/sdk-trace-web';
import { OTLPTraceExporter } from '@opentelemetry/exporter-trace-otlp-http';
import { FetchInstrumentation } from '@opentelemetry/instrumentation-fetch';
import { ZoneContextManager } from '@opentelemetry/context-zone';
import { registerInstrumentations } from '@opentelemetry/instrumentation';

const exporter = new OTLPTraceExporter({
  url: '<opentelemetry-collector-url>/v1/traces',
  headers: {},
  concurrencyLimit: 10,
});

const provider = new WebTracerProvider({
  spanProcessors: [
    new BatchSpanProcessor(exporter, {
      maxQueueSize: 100,
      maxExportBatchSize: 10,
      scheduledDelayMillis: 500,
      exportTimeoutMillis: 30000,
    }),
  ],
});

provider.register({
  contextManager: new ZoneContextManager(),
});

registerInstrumentations({
  instrumentations: [new FetchInstrumentation()],
});
```

Note the OTLP endpoint must end in `/v1/traces` per the exporter's documented contract.

## Non-negotiable design rules

### 1. Never ship `ConsoleSpanExporter`/`SimpleSpanProcessor` as the production pipeline

These are debugging tools. Production browser tracing needs a network exporter (e.g., `OTLPTraceExporter`) paired with `BatchSpanProcessor` so span export is batched and rate-limited instead of firing one network request per span.

### 2. Register a context manager that supports async work

Without `ZoneContextManager` (or an equivalent async-aware context manager), span parent/child relationships across `await`, `setTimeout`, and event-handler boundaries can silently break, producing disconnected traces that look correct individually but don't stitch into one distributed trace.

### 3. Treat sampling as a first-class config, not an afterthought

A browser SDK with no sampler configured effectively samples at 100% by default in many setups — that is a cost and cardinality risk at scale. See `sampling-cardinality-pii-controls.md` for sizing `TraceIdRatioBasedSampler` against traffic volume.

### 4. Do not put secrets or bearer tokens directly in exporter headers as the default pattern

If the collector endpoint requires auth, treat exporter `headers` the same way you would treat any credential-bearing config: source it from a managed secret/config mechanism appropriate to the deployment target, not hardcoded inline in client-shipped JS (client-shipped JS is fully visible to end users — never place a durable secret there regardless of source).

### 5. Match the propagation format to what the backend actually consumes

Verify (via Context7 or the collector/backend's own docs) whether the destination expects W3C Trace Context or B3 headers before wiring a propagator; a mismatch silently drops trace correlation without an error.

## Minimal safe implementation flow

1. Confirm the target collector/backend endpoint and its expected trace-propagation header format.
2. Create `WebTracerProvider` with a `BatchSpanProcessor` wrapping the production exporter (OTLP or equivalent) — not `SimpleSpanProcessor`/`ConsoleSpanExporter`.
3. Register with `ZoneContextManager` (or confirmed async-aware equivalent) so cross-async-boundary spans stay connected.
4. Register only the instrumentations actually needed (`DocumentLoad`, `FetchInstrumentation`, `XMLHttpRequestInstrumentation`) — registering unused instrumentations increases span volume without benefit.
5. Configure sampling (see `sampling-cardinality-pii-controls.md`) before enabling in production, sized to stated traffic.
6. Name any custom span/attribute per OpenTelemetry Semantic Conventions (see `sampling-cardinality-pii-controls.md`) rather than inventing ad-hoc keys.
7. Run every attribute the instrumentation or custom spans emit through the PII check before enabling in production.

## High-risk assumptions to kill

- "Creating the provider means tracing already works" — instrumentations must be separately registered.
- "The default context manager is fine for async code" — it is not; use `ZoneContextManager` or a verified equivalent.
- "No sampler configured means it's fine for now" — unconfigured sampling at high traffic is a cost/cardinality incident waiting to happen.
- "Fetch instrumentation only captures URLs, so it's automatically safe" — captured URLs can include query strings carrying tokens/PII; review `FetchInstrumentation` URL-capture behavior against the PII check.

## When to push back

Push back if the user asks for:

- shipping `SimpleSpanProcessor` + a network exporter direct to production ("it's simpler"),
- no sampler configuration for a stated high-traffic production app,
- hardcoding a durable collector auth token into client-shipped exporter `headers`,
- registering every available instrumentation "to be thorough" without a stated need.

That is not "faster." It is a cost, correctness, and security liability.
