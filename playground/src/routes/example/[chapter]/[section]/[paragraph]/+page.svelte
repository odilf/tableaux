<script lang="ts">
	import TableauEdit from '$lib/TableauEdit.svelte';
	import { is } from 'valibot';
	import { logicOfChapter, singleExampleSchema } from '$lib/examples';
	import Back from '$lib/components/Back.svelte';
	import { chapterLogics, displayName } from '$lib/logic.js';

	const { data } = $props();
	const { chapter, section, paragraph, exampleOrExamples } = $derived(data);
	const width = 574;
</script>

<main class="column">
	<Back href="/example" />
	<h1 class="text-3xl font-bold">
		{#if section === 'exercise'}
			Chapter {chapter}, exercise {paragraph}
		{:else}
			Example {chapter}.{section}.{paragraph}
		{/if} <span class="font-normal">({displayName[chapterLogics[chapter]]} logic)</span>
	</h1>
	<p class="opacity-50">
		i.e., chapter {chapter}, section {section}, paragraph {paragraph}.
	</p>

	{#if is(singleExampleSchema, exampleOrExamples)}
		{@const logic = logicOfChapter(chapter, exampleOrExamples)}
		<div class="h-full">
			<TableauEdit
				premises={exampleOrExamples.premises ?? ''}
				conclusion={exampleOrExamples.conclusion}
				{width}
				{logic}
			/>
		</div>
	{:else}
		{#each Object.entries(exampleOrExamples) as [key, example] (key)}
			{@const logic = logicOfChapter(chapter, example)}
			<div id={key} class="scroll-mt-6">
				<h2 class="text-2xl font-bold italic">({key})</h2>
				<TableauEdit
					premises={example.premises ?? ''}
					conclusion={example.conclusion}
					{width}
					{logic}
				/>
			</div>
		{/each}
	{/if}
</main>
