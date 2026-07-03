// Safe idiom: access token kept only in an in-memory module variable, never
// persisted. Reload re-acquires a new access token via the HttpOnly
// refresh-cookie flow (browser sends the cookie automatically; JS never
// touches it), so nothing token-shaped is ever exposed to storage APIs.
let accessToken = null;

async function completeLogin(credentials) {
  const res = await fetch('/api/login', {
    method: 'POST',
    body: JSON.stringify(credentials),
  });
  const { jwtToken } = await res.json();
  accessToken = jwtToken;
  return jwtToken;
}

async function reacquireAccessTokenAfterReload() {
  // Server reads the HttpOnly refresh cookie automatically; response body
  // carries only the fresh short-lived access token.
  const res = await fetch('/api/token/refresh', { credentials: 'include' });
  const { jwtToken } = await res.json();
  accessToken = jwtToken;
  return jwtToken;
}
