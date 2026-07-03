// Vulnerable: implicit-grant OAuth flow. The access token is returned
// directly in the URL fragment, exposed to browser history, Referer
// leakage, and any script that can read the URL. No PKCE, no refresh token.
function startLogin(clientId, redirectUri) {
  const authUrl =
    `https://auth.provider.com/authorize?response_type=token` +
    `&client_id=${clientId}&redirect_uri=${redirectUri}`;
  window.location.assign(authUrl);
}
