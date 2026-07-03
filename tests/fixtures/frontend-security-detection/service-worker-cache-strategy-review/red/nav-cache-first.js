import { registerRoute } from 'workbox-routing';
import { CacheFirst } from 'workbox-strategies';

// BUG: navigation/HTML requests are matched with a blind cache-first
// strategy — after every deploy, users are stranded on the stale app shell
// cached at their first visit, with no revalidation path.
registerRoute(
  ({ request }) => request.mode === 'navigate',
  new CacheFirst({ cacheName: 'app-shell-v1' })
);
