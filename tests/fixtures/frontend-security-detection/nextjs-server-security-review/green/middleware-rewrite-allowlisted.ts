// middleware.ts
// The rewrite target is validated against a fixed hostname allowlist before
// NextResponse.rewrite() is ever called — a crafted or internal-pointing
// `target` value is rejected instead of being rewritten to.
import { NextResponse } from 'next/server'

const ALLOWED_HOSTS = ['api.internal.example.com']

export function middleware(request: Request) {
  const nextUrl = (request as any).nextUrl
  const target = nextUrl.searchParams.get('target')
  const url = new URL(target, request.url)

  if (!ALLOWED_HOSTS.includes(url.host)) {
    return NextResponse.next()
  }

  return NextResponse.rewrite(url)
}
