// server/api/external-call.ts
export default defineEventHandler(async (event) => {
  const headers = useRequestHeaders() // blindly forwards ALL incoming headers
  return $fetch('https://external-api.com', { headers })
})
