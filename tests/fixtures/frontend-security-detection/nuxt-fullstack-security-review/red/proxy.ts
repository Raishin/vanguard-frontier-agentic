// server/api/proxy.ts
export default defineEventHandler(async (event) => {
  const { url } = getQuery(event)
  return $fetch(url as string) // SSRF: attacker fully controls the fetch destination
})
