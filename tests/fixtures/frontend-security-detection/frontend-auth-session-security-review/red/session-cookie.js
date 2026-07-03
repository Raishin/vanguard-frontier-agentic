// Vulnerable: session cookie is missing HttpOnly. Any script that runs in
// the page's origin (stored/reflected XSS, a compromised third-party
// script) can read document.cookie and exfiltrate the session identifier.
function setSessionCookie(res, sessionId) {
  res.setHeader('Set-Cookie', `sessionid=${sessionId}; Secure; SameSite=Strict`);
}
