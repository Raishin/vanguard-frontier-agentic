// Vulnerable: authorization code flow for a public client (SPA) with no
// PKCE parameters. Without code_challenge/code_challenge_method, an
// intercepted authorization code can be redeemed by an attacker at the
// token endpoint -- PKCE is what binds the code to this specific client.
function buildAuthorizeUrl(clientId, redirectUri) {
  return `https://auth.provider.com/authorize?response_type=code&client_id=${clientId}&redirect_uri=${redirectUri}`;
}
