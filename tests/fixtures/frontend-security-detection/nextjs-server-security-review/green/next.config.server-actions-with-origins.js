/** @type {import('next').NextConfig} */
// Same reverse-proxy deployment, but the safe origins are explicitly
// enumerated so Next.js can compare the Server Action request's Origin
// header against a known-good allowlist instead of only the Host header.
module.exports = {
  experimental: {
    serverActions: {
      allowedOrigins: ['my-proxy.example.com', '*.my-proxy.example.com'],
      bodySizeLimit: '5mb',
    },
  },
}
