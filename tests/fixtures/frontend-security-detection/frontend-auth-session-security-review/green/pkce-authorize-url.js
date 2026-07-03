// Safe idiom: authorization code flow with PKCE wired up -- code_challenge
// and code_challenge_method are present on the authorization request, and
// the paired code_verifier is sent at token exchange (not shown here).
function buildAuthorizeUrl(clientId, redirectUri, codeChallenge) {
  return `https://auth.provider.com/authorize?response_type=code&client_id=${clientId}&redirect_uri=${redirectUri}&code_challenge=${codeChallenge}&code_challenge_method=S256`;
}
