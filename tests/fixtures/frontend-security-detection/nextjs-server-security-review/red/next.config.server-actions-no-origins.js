/** @type {import('next').NextConfig} */
// Deployed behind a corporate reverse proxy (proxy.internal.example.com) that
// forwards to this app on a different origin, so the built-in Origin/Host
// check on Server Actions cannot be satisfied by production traffic — yet no
// allowedOrigins is configured to account for it. This leaves the CSRF check
// either failing legitimate requests or being worked around insecurely
// upstream.
module.exports = {
  experimental: {
    serverActions: {
      bodySizeLimit: '5mb',
    },
  },
}
