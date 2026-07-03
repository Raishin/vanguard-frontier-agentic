// Evaluates a filter expression the user typed into the search box.
function applyUserFilter(expressionFromSearchBox) {
  // expressionFromSearchBox is the raw query-string value the user typed;
  // running it through eval gives it full script execution in this origin.
  return eval(expressionFromSearchBox);
}
