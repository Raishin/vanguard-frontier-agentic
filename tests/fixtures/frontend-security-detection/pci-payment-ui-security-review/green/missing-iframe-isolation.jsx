import { PaymentElement, useElements, useStripe } from '@stripe/react-stripe-js';

// PaymentElement mounts Stripe's own hosted fields inside sandboxed
// iframes; card number, expiry, and CVC are all collected and rendered
// entirely outside the merchant's React tree.
export function HostedCardForm({ onSuccess }) {
  const stripe = useStripe();
  const elements = useElements();

  const handleSubmit = async (e) => {
    e.preventDefault();
    const { paymentIntent } = await stripe.confirmPayment({
      elements,
      confirmParams: { return_url: 'https://example.com/complete' },
      redirect: 'if_required',
    });
    onSuccess(paymentIntent);
  };

  return (
    <form onSubmit={handleSubmit}>
      <PaymentElement />
      <button type="submit">Pay</button>
    </form>
  );
}
