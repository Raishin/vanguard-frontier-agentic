/** @type {import('next').NextConfig} */
// Enables local-IP optimization for the <Image> component with no
// accompanying allowlist validation on dynamic `src` values — an
// attacker-controlled image URL could target internal/loopback services
// (SSRF via the image optimizer).
module.exports = {
  images: {
    dangerouslyAllowLocalIP: true,
    remotePatterns: [{ protocol: 'https', hostname: '**' }],
  },
}
