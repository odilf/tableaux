<script lang="ts">
	import Back from '$lib/components/Back.svelte';
	import { chapterLogics, displayName } from '$lib/logic';
	import ChapterExamples from './ChapterExamples.svelte';
	const { data } = $props();
</script>

<main class="column">
	<Back href="/" />
	<h1 class="text-3xl font-bold">Examples and exercises</h1>
	<p>
		This page will contain almost all examples and exercises from
		<a
			href="https://www.cambridge.org/it/universitypress/subjects/philosophy/philosophy-science/introduction-non-classical-logic-if-2nd-edition"
			target="_blank"
			rel="noreferrer"
		>
			Graham Priest's <em>An Introduction to Non-Classical Logic</em>
		</a>.
	</p>

	{#each Object.entries(data.examples) as [chapter, chapterExamples] (chapter)}
		<h2 class="mt-8 text-2xl font-bold">
			<a href="/example/{chapter}">
				Chapter {chapter}: {displayName[chapterLogics[chapter]]} logic
			</a>
		</h2>

		<ChapterExamples chapter={parseInt(chapter)} {chapterExamples} />
	{/each}
</main>
