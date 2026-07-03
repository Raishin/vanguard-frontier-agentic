// Admin panel route guard -- this is the ONLY check gating access to /admin.
// There is no server-side authorization check on the /api/admin/* endpoints;
// they trust any authenticated session and return data to whoever asks.
router.beforeEach((to, from) => {
  if (to.meta.requiresAdmin && !authStore.isAdmin) {
    return { name: 'Login' }
  }
})
