// DANGEROUS: generateStaticParams enumerates per-user account IDs, and the
// route also carries a `revalidate` export. Once the ISR window elapses, the
// next request for /account/[userId] re-triggers rendering and re-caches the
// authenticated page -- any visitor who polls this URL within the next
// 60-second window is served the previously cached user's authenticated HTML.
export const revalidate = 60

export async function generateStaticParams() {
  const users = await db.users.findAll()
  return users.map((u) => ({ userId: u.id }))
}

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
