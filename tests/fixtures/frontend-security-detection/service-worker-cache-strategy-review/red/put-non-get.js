// Service worker fetch handler for /api/orders (POST create-order endpoint)
self.addEventListener('fetch', (event) => {
  const req = event.request;
  const url = new URL(req.url);

  if (url.pathname === '/api/orders') {
    event.respondWith(
      fetch(req).then((response) => {
        const responseClone = response.clone();
        caches.open('api-cache-v1').then((cache) => {
          // BUG: Cache API only stores GET responses by spec, but this manually
          // constructs a POST Request object and writes it into the cache anyway.
          cache.put(new Request(url, { method: 'POST' }), responseClone);
        });
        return response;
      })
    );
  }
});
