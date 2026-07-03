// Admin panel route guard -- gates the UI, but authorization is actually
// decided by a server round-trip so a forged client flag can't bypass it.
// The /api/admin/* endpoints independently re-check the session server-side.
router.beforeEach(async (to, from) => {
  if (to.meta.requiresAdmin) {
    const allowed = await verifyAdminOnServer()
    if (!allowed) {
      return { name: 'Login' }
    }
  }
})
