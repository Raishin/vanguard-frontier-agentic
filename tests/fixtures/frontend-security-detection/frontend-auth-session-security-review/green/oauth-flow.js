// Safe idiom: authorization code flow with PKCE for a public client. The
// authorization server returns a short-lived code, exchanged server-side for
// tokens; nothing token-shaped ever appears in the URL/fragment/history.
async function startLogin(clientId, redirectUri) {
  const codeVerifier = generateCodeVerifier();
  const codeChallenge = await sha256Base64Url(codeVerifier);
  sessionStorage.setItem('pkce_code_verifier', codeVerifier);

  const authUrl =
    `https://auth.provider.com/authorize?response_type=code` +
    `&client_id=${clientId}&redirect_uri=${redirectUri}` +
    `&code_challenge=${codeChallenge}&code_challenge_method=S256`;
  window.location.assign(authUrl);
}
