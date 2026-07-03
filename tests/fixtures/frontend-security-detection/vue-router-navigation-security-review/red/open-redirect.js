// Post-login redirect handler.
async function onLoginSuccess(route, router) {
  await auth.login()
  // Whatever the query string says, off we go -- including a full
  // attacker-controlled absolute URL like //evil.example.com/phish.
  router.push(route.query.redirect)
}
