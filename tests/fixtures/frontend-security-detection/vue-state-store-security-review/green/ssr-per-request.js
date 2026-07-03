// entry-server.js
import { createSSRApp } from 'vue'
import { createPinia } from 'pinia'
import App from './App.vue'

export async function render(url) {
  // Fresh Pinia instance per request — no cross-request shared state.
  const pinia = createPinia()
  const app = createSSRApp(App)
  app.use(pinia)

  pinia.state.value.route = { url }
  return app
}
