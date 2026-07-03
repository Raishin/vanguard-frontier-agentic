import ClientDashboardView from './ClientDashboardView'
import { getSystemConfig } from './config'

export async function ConfigPanel() {
  const config = await getSystemConfig()
  // Whole config object (which contains SERVICE_API_KEY) forwarded verbatim — no
  // narrowing to a safe subset, and no taint marker on the sensitive property.
  return <ClientDashboardView config={config} />
}
