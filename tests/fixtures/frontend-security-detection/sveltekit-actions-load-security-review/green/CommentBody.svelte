<script>
	import DOMPurify from 'dompurify';

	/** @type {{ comment: { body: string } }} */
	let { comment } = $props();
</script>

<!-- Safe: the user-submitted markup is piped through DOMPurify.sanitize()
     inline in the template expression before {@html} ever sets innerHTML. -->
<div class="comment-body">
	{@html DOMPurify.sanitize(comment.body)}
</div>
