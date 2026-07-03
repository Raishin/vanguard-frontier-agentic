<script>
	/** @type {{ comment: { body: string } }} */
	let { comment } = $props();
</script>

<!-- DANGER: comment.body is user-submitted markup rendered with no
     sanitization step. {@html} sets innerHTML directly, so a comment
     containing <script> or an onerror-bearing tag executes for every
     viewer of this comment. -->
<div class="comment-body">
	{@html comment.body}
</div>
