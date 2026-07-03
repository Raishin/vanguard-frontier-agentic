import { registerRoute } from 'workbox-routing';
import { NetworkOnly } from 'workbox-strategies';

// SAFE: /login sets a session cookie, so it is excluded from the Cache API
// entirely via NetworkOnly — nothing is ever written to persistent storage.
registerRoute('/login', new NetworkOnly());
