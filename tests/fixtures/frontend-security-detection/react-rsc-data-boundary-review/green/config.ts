// Server-only data module — guarded so any accidental client import fails the build.
import 'server-only'

export async function getSystemConfig() {
  const res = await fetch('https://internal-config.example.com/config', {
    headers: { authorization: process.env.SERVICE_API_KEY },
  })
  return res.json()
}
