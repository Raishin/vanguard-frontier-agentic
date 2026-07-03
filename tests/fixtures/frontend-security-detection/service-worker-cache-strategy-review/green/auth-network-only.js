import { registerRoute } from 'workbox-routing';
import { NetworkOnly } from 'workbox-strategies';

// SAFE: /api/user echoes the caller's auth token in its response body, so
// authenticated endpoints are routed NetworkOnly and never touch the Cache
// API — the header is inspected only for logging, never persisted.
registerRoute(/^\/api\/(user|account|orders)/, new NetworkOnly());

self.addEventListener('fetch', (event) => {
  const req = event.request;
  if (new URL(req.url).pathname === '/api/user') {
    event.respondWith(
      fetch(req).then((response) => {
        const authHeader = response.headers.get('Authorization');
        console.debug('auth header present:', Boolean(authHeader));
        return response;
      })
    );
  }
});
