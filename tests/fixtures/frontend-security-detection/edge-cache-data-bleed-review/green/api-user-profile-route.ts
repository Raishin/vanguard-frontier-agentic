import { cookies } from 'next/headers'

// SAFE: the response carries `Cache-Control: private`, which tells shared
// CDN/proxy caches they must not store the response at all -- only the
// requesting user's own browser may cache it locally.
export async function GET() {
  const session = (await cookies()).get('session')?.value
  const userData = await db.users.findBySession(session)
  return new Response(JSON.stringify(userData), {
    headers: {
      'Cache-Control': 'private, max-age=300',
      'Content-Type': 'application/json',
    },
  })
}
