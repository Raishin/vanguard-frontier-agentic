// Raw card fields captured from the form are POSTed directly to a
// first-party endpoint instead of being tokenized by the Stripe API
// first. The PAN and CVV traverse the merchant's own network stack and
// logs, pulling the endpoint (and everything downstream of it) into
// full PCI-DSS scope.
async function submitPayment(pan: string, cvv: string, exp: string) {
  const response = await fetch('/api/pay', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ pan, cvv, exp }),
  });
  return response.json();
}

export { submitPayment };
