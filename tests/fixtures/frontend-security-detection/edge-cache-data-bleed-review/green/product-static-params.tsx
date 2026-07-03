// SAFE: no `revalidate` export on this authenticated, per-user route. Instead
// the route opts fully into dynamic rendering with `force-dynamic`, so every
// request is rendered fresh on the server and nothing is ever written to a
// shared ISR cache entry keyed only by the route path.
export const dynamic = 'force-dynamic'

export default async function AccountPage({
  params,
}: {
  params: { userId: string }
}) {
  const account = await db.accounts.findByUserId(params.userId)
  return (
    <main>
      <h1>{account.ownerName}'s account</h1>
      <p>{account.privateNotes}</p>
    </main>
  )
}
