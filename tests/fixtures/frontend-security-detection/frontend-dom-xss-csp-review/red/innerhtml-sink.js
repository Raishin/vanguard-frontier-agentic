// Renders a user-supplied comment body directly into the DOM.
function renderComment(comment) {
  const el = document.getElementById('comment-body');
  // comment.body comes straight from the API response (attacker-controlled
  // via the comment submission form) with no sanitization step.
  el.innerHTML = comment.body;
}
