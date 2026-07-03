// entry-client.js
import { createApp } from './main'

const { app, pinia } = createApp()

// WRONG: no shape validation — a compromised/altered window.__pinia payload is
// assigned directly into the store tree and trusted by every component.
pinia.state.value = JSON.parse(window.__pinia)

app.mount('#app')
