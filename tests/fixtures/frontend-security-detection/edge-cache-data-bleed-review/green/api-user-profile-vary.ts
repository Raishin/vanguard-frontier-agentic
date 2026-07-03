import { cookies } from 'next/headers'

// SAFE: Cookie is included in Vary, so the CDN keys its cache on the
// session cookie's value and never serves one user's cached response body to
// a request carrying a different Cookie header.
export async function GET() {
  const session = (await cookies()).get('session')?.value
  const userData = await db.users.findBySession(session)
  return new Response(JSON.stringify(userData), {
    headers: {
      'Cache-Control': 'public, max-age=300',
      Vary: 'Cookie, Accept-Encoding',
    },
  })
}
