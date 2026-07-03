import { cookies } from 'next/headers'
import { Suspense } from 'react'

// SAFE: no route-level `revalidate` export at all. The personalized lookup
// is isolated behind a 'use cache: private' function instead, which caches
// only in the requesting user's own browser and never shares a server-side
// cache entry across users.
export default async function DashboardPage() {
  return (
    <main>
      <Suspense fallback={<p>Loading...</p>}>
        <UserSummary />
      </Suspense>
    </main>
  )
}

async function UserSummary() {
  const user = await getUser()
  return (
    <>
      <h1>Welcome, {user.name}</h1>
      <p>Account balance: {user.balance}</p>
    </>
  )
}

async function getUser() {
  'use cache: private'
  const session = (await cookies()).get('session')?.value
  return db.users.findBySession(session)
}
