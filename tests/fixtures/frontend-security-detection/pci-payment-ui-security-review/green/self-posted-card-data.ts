// The card element is tokenized through the Stripe API first; only the
// resulting single-use token is ever POSTed to the first-party endpoint,
// so the merchant server (and its logs) never sees raw card data.
async function submitPayment(cardElement: stripe.elements.Element) {
  const { token } = await stripe.createToken(cardElement);
  const response = await fetch('/api/pay', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ token: token.id }),
  });
  return response.json();
}

export { submitPayment };
