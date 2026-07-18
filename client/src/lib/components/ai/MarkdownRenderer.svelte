<script lang="ts">
	import Markdown from 'svelte-markdown';

	interface Props {
		content: string;
	}

	const { content }: Props = $props();
</script>

<Markdown source={content} />
