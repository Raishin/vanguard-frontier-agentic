// Server Component — fetches config and forwards only a narrowed, non-secret field.
import ClientDashboard from './ClientDashboard'
import { getSystemConfig } from './config'

export async function Dashboard() {
  const config = await getSystemConfig()
  // Safe: only the non-tainted version string crosses the boundary, never the raw config object.
  return <ClientDashboard version={config.SERVICE_API_VERSION} />
}
