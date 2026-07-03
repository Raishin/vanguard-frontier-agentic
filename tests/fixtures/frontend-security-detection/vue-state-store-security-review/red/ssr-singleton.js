// entry-server.js
import { createSSRApp } from 'vue'
import { createPinia } from 'pinia'
import App from './App.vue'

// WRONG: module scope — one Pinia instance shared by every concurrent request
// on this server process. One user's state can leak into another's response.
const pinia = createPinia()
const app = createSSRApp(App)
app.use(pinia)

export async function render(url) {
  pinia.state.value.route = { url }
  return app
}
