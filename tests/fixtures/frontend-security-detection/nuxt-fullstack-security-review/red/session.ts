// server/api/session.ts
const lastUser: Record<string, unknown> = {}

export default defineEventHandler((event) => {
  const userId = getQuery(event).userId
  lastUser.id = userId // mutates a module-scope object shared by every concurrent request
  return lastUser
})
