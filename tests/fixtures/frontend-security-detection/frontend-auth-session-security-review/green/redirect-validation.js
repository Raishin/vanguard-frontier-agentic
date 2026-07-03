// Safe idiom: the client performs no destination validation at all. It
// asks the server to resolve a short opaque key to a target, and the
// server checks that target against its own allow-list before ever
// returning it -- an attacker hitting the endpoint directly gets the same
// server-side allow-list enforcement, so there is no bypassable client gate.
async function completePostLoginRedirect(returnKey) {
  const res = await fetch(`/api/resolve-return-url?key=${encodeURIComponent(returnKey)}`, {
    credentials: 'include',
  });
  const { destination } = await res.json();
  window.location.assign(destination);
}
