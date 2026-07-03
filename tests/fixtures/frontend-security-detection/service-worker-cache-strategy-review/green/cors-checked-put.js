self.addEventListener('fetch', (event) => {
  if (new URL(event.request.url).pathname === '/cdn/widget.js') {
    event.respondWith(
      // SAFE: CORS mode makes status/headers inspectable, so only a
      // confirmed-ok response is ever written to the Cache API.
      fetch('/cdn/widget.js', { mode: 'cors' }).then((res) => {
        if (res.ok) {
          caches.open('cdn-v1').then((cache) => cache.put('/cdn/widget.js', res.clone()));
        }
        return res;
      })
    );
  }
});
