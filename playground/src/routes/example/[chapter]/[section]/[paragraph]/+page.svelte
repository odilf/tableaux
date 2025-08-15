<script lang="ts">
	import TableauEdit from '$lib/TableauEdit.svelte';
	import { is } from 'valibot';
	import { logicOfChapter, singleExampleSchema } from '../../../index.js';
	import Back from '$lib/components/Back.svelte';

	const { data } = $props();
	const { chapter, section, paragraph, exampleOrExamples } = $derived(data);
</script>

<main class="column">
	<Back href="/example" />
	<h1 class="text-3xl font-bold">
		{#if section === 'exercise'}
			Chapter {chapter}, exercise {paragraph}
		{:else}
			Example {chapter}.{section}.{paragraph}
		{/if}
	</h1>
	<p class="opacity-50">
		i.e., chapter {chapter}, section {section}, paragraph {paragraph}.
	</p>

	{#if is(singleExampleSchema, exampleOrExamples)}
		{@const logic = logicOfChapter(parseInt(chapter), exampleOrExamples)}
		<div class="h-full">
			<TableauEdit
				premises={exampleOrExamples.premises ?? ''}
				conclusion={exampleOrExamples.conclusion}
				{logic}
			/>

			<div>Should hold: {exampleOrExamples.holds ?? 'true'}</div>
		</div>
	{:else}
		{#each Object.entries(exampleOrExamples) as [key, example] (key)}
			{@const logic = logicOfChapter(parseInt(chapter), example)}
			<div id={key} class="scroll-mt-6">
				<h2 class="text-2xl font-bold italic">({key})</h2>
				<TableauEdit premises={example.premises ?? ''} conclusion={example.conclusion} {logic} />

				<div>Should hold: {example.holds ?? 'true'}</div>
			</div>
		{/each}
	{/if}
</main>
