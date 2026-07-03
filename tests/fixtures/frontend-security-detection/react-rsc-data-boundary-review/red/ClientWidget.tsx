'use client'

// Non-NEXT_PUBLIC env var read inside a client module — undefined at runtime in the
// browser bundle, and unsafe to have ever assumed it would carry a real secret value.
const apiKey = process.env.DATABASE_URL

export default function ClientWidget() {
  return <div data-key={apiKey}>Widget</div>
}
