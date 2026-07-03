// Post-login redirect handler.
const SAFE_REDIRECT = /^\/(?!\/)/

async function onLoginSuccess(route, router) {
  await auth.login()
  // Only accept a same-origin relative path; anything else (protocol-relative
  // //evil.example.com, absolute https://evil.example.com) falls back to home.
  const target = route.query.redirect
  router.push(SAFE_REDIRECT.test(target) ? target : '/')
}
