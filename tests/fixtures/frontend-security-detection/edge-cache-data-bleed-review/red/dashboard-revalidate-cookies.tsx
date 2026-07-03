import { cookies } from 'next/headers'

// DANGEROUS: revalidate is set on a route that also reads cookies() to render
// per-user data. The ISR cache entry is shared across every visitor who hits
// this path within the 60-second window -- the next visitor within that
// window gets served the first visitor's cached, personalized HTML.
export const revalidate = 60

export default async function DashboardPage() {
  const session = (await cookies()).get('session')?.value
  const user = await db.users.findBySession(session)
  return (
    <main>
      <h1>Welcome, {user.name}</h1>
      <p>Account balance: {user.balance}</p>
    </main>
  )
}
