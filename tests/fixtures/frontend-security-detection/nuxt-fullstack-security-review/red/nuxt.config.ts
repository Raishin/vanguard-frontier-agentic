export default defineNuxtConfig({
  runtimeConfig: {
    public: {
      apiSecret: process.env.NUXT_PUBLIC_API_SECRET // exposed to every client bundle
    }
  }
})
