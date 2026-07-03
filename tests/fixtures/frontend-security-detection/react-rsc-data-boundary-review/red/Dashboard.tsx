// Server Component — fetches config and hands secrets straight to a Client Component.
import ClientDashboard from './ClientDashboard'

export async function Dashboard() {
  const SECRET_API_KEY = process.env.SERVICE_API_KEY
  // DO NOT DO THIS: a raw secret crosses the serialization boundary as a prop.
  return <ClientDashboard password={SECRET_API_KEY} />
}
