// middleware.ts
// Matcher explicitly lists only the protected page routes it must run on.
// It does not exclude /api via a negative lookahead, and every Server Action /
// Route Handler under /api independently verifies its own session inside the
// handler body (see app/actions.ts), so nothing relies on middleware alone.
import { NextResponse } from 'next/server'

export function middleware(request: Request) {
  return NextResponse.next()
}

export const config = {
  matcher: ['/dashboard/:path*', '/admin/:path*'],
}
