// Global auth guard. Every route -- including the Login route itself --
// gets funneled through this check, so an expired session on the Login page
// bounces right back to itself: an infinite navigation loop.
router.beforeEach((to, from) => {
  if (to.meta.requiresAuth && !auth.isLoggedIn()) {
    return { name: 'Login' }
  }
})
