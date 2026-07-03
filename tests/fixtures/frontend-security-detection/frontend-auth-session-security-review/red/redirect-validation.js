// Vulnerable: redirect destination is validated only in client-side
// JavaScript before navigating. An attacker can bypass this entirely by
// hitting the server-side login/callback endpoint directly with a
// malicious `redirectUrl` query parameter -- this check never runs then.
function completePostLoginRedirect(redirectUrl, appDomain) {
  if (redirectUrl.includes(appDomain)) {
    window.location.assign(redirectUrl);
  } else {
    window.location.assign('/');
  }
}
