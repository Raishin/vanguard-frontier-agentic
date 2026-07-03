import { cookies } from 'next/headers'

// DANGEROUS: the CDN is told this response varies only by Accept-Encoding.
// Because the session Cookie is absent from Vary, the CDN treats requests
// from two different users (two different Cookie values) as cache-equivalent
// and may serve one user's cached, personalized response to the other.
export async function GET() {
  const session = (await cookies()).get('session')?.value
  const userData = await db.users.findBySession(session)
  return new Response(JSON.stringify(userData), {
    headers: {
      'Cache-Control': 'public, max-age=300',
      Vary: 'Accept-Encoding',
    },
  })
}
