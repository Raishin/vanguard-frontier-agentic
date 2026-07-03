import { createSSRApp } from 'vue'
import { reactive } from 'vue'
import App from './App.vue'

// DANGER: declared at module scope — this single object is shared across
// every concurrent request handled by this long-lived Node.js process.
const state = reactive({ userId: null, userData: {} })

export async function render(url, manifest) {
  const app = createSSRApp(App)
  state.userId = extractUserIdFromUrl(url)
  state.userData = await loadUserData(state.userId)
  return app
}

function extractUserIdFromUrl(url) {
  return new URLSearchParams(url.split('?')[1]).get('userId')
}

async function loadUserData(userId) {
  return { id: userId }
}
