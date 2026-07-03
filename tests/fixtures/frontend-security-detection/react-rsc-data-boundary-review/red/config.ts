// Data module intended for the server, but missing the required guard import.
export async function getSystemConfig() {
  const res = await fetch('https://internal-config.example.com/config', {
    headers: { authorization: process.env.SERVICE_API_KEY },
  })
  return res.json()
}
