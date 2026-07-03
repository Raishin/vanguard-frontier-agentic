// server/api/external-call.ts
export default defineEventHandler(async (event) => {
  // explicit allowlist: only forward the one header the downstream call needs
  const headers = useRequestHeaders(['user-agent'])
  return $fetch('https://external-api.com', { headers })
})
