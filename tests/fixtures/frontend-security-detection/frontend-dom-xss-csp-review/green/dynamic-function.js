// Builds a custom formatter from a template string stored in user settings.
function buildFormatter(userSuppliedTemplate) {
  // A fixed, statically defined formatter is returned; the user-supplied
  // template only selects which precomputed placeholder key to use, and is
  // never compiled into executable code.
  const allowedTemplates = {
    currency: (value) => `$${value.toFixed(2)}`,
    percent: (value) => `${(value * 100).toFixed(0)}%`,
  };
  return allowedTemplates[userSuppliedTemplate] || allowedTemplates.currency;
}
