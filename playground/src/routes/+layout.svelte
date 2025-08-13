<script lang="ts">
	import '../app.css';
	import favicon from '$lib/assets/favicon.svg';

	import '@fontsource/libertinus-serif/400.css';
	import '@fontsource/libertinus-serif/600.css';
	import '@fontsource/libertinus-serif/700.css';
	import '@fontsource/libertinus-math';

	let { children } = $props();
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

<main class="mx-auto flex h-screen max-w-xl flex-col p-4 font-serif">
	{@render children?.()}
</main>
