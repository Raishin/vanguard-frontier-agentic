// Builds a custom formatter from a template string stored in user settings.
function buildFormatter(userSuppliedTemplate) {
  // userSuppliedTemplate is loaded from the user's saved preferences (editable
  // via the settings API) and compiled straight into an executable function body.
  const formatter = new Function('value', 'return `' + userSuppliedTemplate + '`;');
  return formatter;
}
