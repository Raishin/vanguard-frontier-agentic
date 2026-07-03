// middleware.ts
// Broad matcher that excludes the entire /api tree from middleware execution.
// The team believes "middleware protects the app" but every Server Function
// and Route Handler under /api never runs through this middleware at all —
// a Proxy matcher that excludes a path also skips Server Function calls on
// that path.
import { NextResponse } from 'next/server'

export function middleware(request: Request) {
  // No session check happens here for excluded paths — they never reach this code.
  return NextResponse.next()
}

export const config = {
  matcher: ['/((?!api|_next).*)'],
}
