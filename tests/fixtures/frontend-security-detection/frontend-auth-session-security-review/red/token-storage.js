// Vulnerable: access token persisted to localStorage, readable by any
// injected script (stored/reflected XSS, compromised CDN dependency, etc.)
async function completeLogin(credentials) {
  const res = await fetch('/api/login', {
    method: 'POST',
    body: JSON.stringify(credentials),
  });
  const { jwtToken } = await res.json();
  localStorage.setItem('accessToken', jwtToken);
  return jwtToken;
}
