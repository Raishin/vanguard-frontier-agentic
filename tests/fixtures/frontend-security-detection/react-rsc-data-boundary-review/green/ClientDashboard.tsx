import ClientDashboardView from './ClientDashboardView'
import { getSystemConfig } from './config'

export async function ConfigPanel() {
  const config = await getSystemConfig()
  // Safe: only the non-sensitive, narrowed property crosses the boundary.
  return <ClientDashboardView version={config.SERVICE_API_VERSION} />
}
