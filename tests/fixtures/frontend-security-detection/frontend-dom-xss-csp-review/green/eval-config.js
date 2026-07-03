// Evaluates a filter expression the user typed into the search box.
import { parseFilterExpression } from './safe-filter-parser.js';

function applyUserFilter(expressionFromSearchBox) {
  // A dedicated grammar-limited parser interprets only a fixed set of
  // comparison operators; it never hands the string to the JS engine.
  return parseFilterExpression(expressionFromSearchBox);
}
