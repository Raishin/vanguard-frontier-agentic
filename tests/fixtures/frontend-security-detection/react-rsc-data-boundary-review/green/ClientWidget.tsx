'use client'

// Safe: only a build-time-injected NEXT_PUBLIC_ variable is read in the client module.
const publicEndpoint = process.env.NEXT_PUBLIC_API_URL

export default function ClientWidget() {
  return <div data-endpoint={publicEndpoint}>Widget</div>
}
