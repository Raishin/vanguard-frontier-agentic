import { registerRoute } from 'workbox-routing';
import { CacheFirst } from 'workbox-strategies';

// BUG: /login responds with a Set-Cookie session header, but the route is
// matched with CacheFirst, so the Cache API persists the session-bearing
// response for every future visit on this device (shared-device leak).
registerRoute('/login', new CacheFirst({ cacheName: 'auth-v1' }));
