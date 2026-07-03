export default defineNuxtConfig({
  runtimeConfig: {
    apiSecret: process.env.NUXT_API_SECRET, // server-only, never sent to the client
    public: {
      apiBase: '/api' // genuinely public-facing value
    }
  }
})
