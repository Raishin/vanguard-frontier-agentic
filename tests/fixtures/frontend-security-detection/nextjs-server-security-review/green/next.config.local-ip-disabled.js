/** @type {import('next').NextConfig} */
// Local-IP image optimization stays disabled; dynamic `src` values are
// constrained to a fixed allowlist of external hostnames instead.
module.exports = {
  images: {
    dangerouslyAllowLocalIP: false,
    remotePatterns: [{ protocol: 'https', hostname: 'images.example.com' }],
  },
}
