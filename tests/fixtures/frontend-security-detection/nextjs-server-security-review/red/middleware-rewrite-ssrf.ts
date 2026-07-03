// middleware.ts
// Builds the NextResponse.rewrite() destination directly from a
// user-controlled query parameter with no allowlist check — an attacker can
// point `target` at an internal service or arbitrary external host
// (SSRF / open redirect via the rewrite backend).
import { NextResponse } from 'next/server'

export function middleware(request: Request) {
  const nextUrl = (request as any).nextUrl
  return NextResponse.rewrite(new URL(nextUrl.searchParams.get('target'), request.url))
}
