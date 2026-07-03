// server/api/proxy.ts
const ALLOWED_HOSTS = new Set(['api.trusted-partner.com'])

export default defineEventHandler(async (event) => {
  const { path } = getQuery(event)
  const target = new URL(String(path), 'https://api.trusted-partner.com')
  if (!ALLOWED_HOSTS.has(target.host)) {
    throw createError({ statusCode: 400 })
  }
  return $fetch(target.toString())
})
