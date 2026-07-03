// server/api/session.ts
export default defineEventHandler((event) => {
  const userId = getQuery(event).userId // per-request local variable, never shared
  return { id: userId }
})
