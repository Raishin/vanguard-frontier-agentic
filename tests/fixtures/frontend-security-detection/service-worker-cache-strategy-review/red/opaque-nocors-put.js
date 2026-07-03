self.addEventListener('fetch', (event) => {
  if (new URL(event.request.url).pathname === '/cdn/widget.js') {
    event.respondWith(
      // BUG: opaque no-cors response cannot be inspected for status/headers,
      // so a 404 or 500 from the third-party CDN is cached and served as if
      // it were a valid script, indefinitely, with no visibility.
      fetch('/cdn/widget.js', { mode: 'no-cors' }).then((res) => {
        caches.open('cdn-v1').then((cache) => cache.put('/cdn/widget.js', res));
        return res;
      })
    );
  }
});
