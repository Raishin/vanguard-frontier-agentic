// Global auth guard. The Login route is explicitly excluded from the check,
// so an expired session on the Login page never gets redirected back to
// itself -- no loop is possible.
router.beforeEach((to, from) => {
  if (to.meta.requiresAuth && !auth.isLoggedIn() && to.name !== 'Login') {
    return { name: 'Login' }
  }
})
