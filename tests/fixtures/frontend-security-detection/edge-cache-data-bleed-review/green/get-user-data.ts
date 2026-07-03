import { cookies } from 'next/headers'

// SAFE: 'use cache: private' is the first statement in the function body.
// Next.js caches this function's result only in the requesting user's own
// browser -- it is never written to a shared server-side cache entry that a
// different user's request could read.
async function getUserData() {
  'use cache: private'
  const session = (await cookies()).get('session')?.value
  return db.query(session)
}

export default getUserData
