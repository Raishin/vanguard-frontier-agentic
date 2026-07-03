// entry-client.js
import { createApp } from './main'
import { isValidPiniaStateShape } from './hydrate-guard'

const { app, pinia } = createApp()

const raw = JSON.parse(window.__pinia)
if (isValidPiniaStateShape(raw)) {
  pinia.state.value = raw
} else {
  console.error('Discarding malformed hydration payload')
}

app.mount('#app')
