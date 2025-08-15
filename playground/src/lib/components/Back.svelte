<script lang="ts">
	const { href = '..' }: { href?: string } = $props();
</script>

<a {href} class="sticky top-0 z-10 inline-block w-full bg-neutral pt-0 italic">
	<span class="opacity-50"> ⟞ Back </span>
</a>
