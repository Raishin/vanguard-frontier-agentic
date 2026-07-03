import { cookies } from 'next/headers'

// DANGEROUS: the response carries `Cache-Control: public`, so an edge CDN in
// front of this Route Handler is free to store the response body and replay
// it to any subsequent client that requests the same URL -- including a
// different, unauthenticated visitor -- even though the body is derived
// entirely from the requesting user's own session cookie.
export async function GET() {
  const session = (await cookies()).get('session')?.value
  const userData = await db.users.findBySession(session)
  return new Response(JSON.stringify(userData), {
    headers: {
      'Cache-Control': 'public, max-age=300',
      'Content-Type': 'application/json',
    },
  })
}
