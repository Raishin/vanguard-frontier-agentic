self.addEventListener('fetch', (event) => {
  const req = event.request;
  if (new URL(req.url).pathname === '/api/user') {
    event.respondWith(
      fetch(req).then((response) => {
        // BUG: /api/user echoes the caller's bearer token back in a header
        // for debugging, and the handler blindly persists that response into
        // the Cache API, so the token is readable from disk on any shared
        // device that later opens DevTools > Application > Cache Storage.
        const authHeader = response.headers.get('Authorization');
        caches.open('user-cache-v1').then((cache) => {
          cache.put(req, response.clone());
        });
        return response;
      })
    );
  }
});
