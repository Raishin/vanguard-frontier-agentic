import { cookies } from 'next/headers'

// DANGEROUS: this server function reads cookies() to look up per-user data
// but declares no cache boundary at all. If it is ever called from a context
// that Next.js decides to cache (e.g. wrapped by a parent 'use cache' scope,
// or a future refactor adds one), the session-derived result gets cached and
// reused for a different user's request with no per-user cache key.
async function getUserData() {
  const session = (await cookies()).get('session')?.value
  return db.query(session)
}

export default getUserData
