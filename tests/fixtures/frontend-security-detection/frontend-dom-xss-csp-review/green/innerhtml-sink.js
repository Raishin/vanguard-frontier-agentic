// Renders a user-supplied comment body directly into the DOM.
function renderComment(comment) {
  const el = document.getElementById('comment-body');
  // text-only assignment: the browser auto-escapes the string, no markup
  // is ever parsed, so untrusted comment.body cannot inject elements.
  el.textContent = comment.body;
}
