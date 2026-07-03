import { registerRoute } from 'workbox-routing';
import { StaleWhileRevalidate } from 'workbox-strategies';

// SAFE: navigation requests are served from cache immediately but
// revalidated in the background, so a new deploy is picked up on the very
// next navigation instead of stranding users on a stale app shell forever.
registerRoute(
  ({ request }) => request.mode === 'navigate',
  new StaleWhileRevalidate({ cacheName: 'app-shell-v1' })
);
