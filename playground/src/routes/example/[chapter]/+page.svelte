<script lang="ts">
	import { chapterLogics, displayName } from '$lib/logic.js';
	import ChapterExamples from '../ChapterExamples.svelte';
	import Back from '$lib/components/Back.svelte';

	const { data } = $props();
	const { chapter, chapterExamples } = $derived(data);
</script>

<main class="column">
	<Back href="/example" />
	<h1 class="mb-2 text-3xl font-bold">
		Chapter {chapter}: {displayName[chapterLogics[parseInt(chapter) - 1]]} logic
	</h1>

	<ChapterExamples chapter={parseInt(chapter)} {chapterExamples} />
</main>
