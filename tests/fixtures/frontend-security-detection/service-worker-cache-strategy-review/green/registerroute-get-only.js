import { registerRoute } from 'workbox-routing';
import { CacheFirst, NetworkOnly } from 'workbox-strategies';

// SAFE: mutation routes are excluded from the Cache API entirely via
// NetworkOnly, and read routes are matched explicitly by method: 'GET' at
// the router level, so nothing ever attempts to cache a non-GET response.
registerRoute(
  ({ url, request }) => url.pathname === '/api/orders' && request.method === 'GET',
  new CacheFirst({ cacheName: 'api-orders-read-v1' })
);

registerRoute(
  ({ url }) => url.pathname === '/api/orders',
  new NetworkOnly()
);
