import { createSSRApp } from 'vue'
import { reactive } from 'vue'
import App from './App.vue'

export async function render(url, manifest) {
  // Safe: created fresh inside the per-request function, so each request
  // gets its own isolated, non-shared reactive object.
  const state = reactive({ userId: null, userData: {} })
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
