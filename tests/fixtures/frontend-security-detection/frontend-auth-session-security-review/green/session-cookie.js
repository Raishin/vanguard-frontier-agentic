// Safe idiom: session cookie carries all three independent flags --
// HttpOnly blocks JS read access, Secure restricts transport to HTTPS, and
// SameSite=Strict blocks cross-site attachment.
function setSessionCookie(res, sessionId) {
  res.setHeader('Set-Cookie', `sessionid=${sessionId}; HttpOnly; Secure; SameSite=Strict`);
}
