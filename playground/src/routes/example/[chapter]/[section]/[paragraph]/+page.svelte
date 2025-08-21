<script lang="ts">
	import TableauEdit from '$lib/TableauEdit.svelte';
	import { is } from 'valibot';
	import { logicOfChapter, singleExampleSchema, type Example } from '$lib/examples';
	import Back from '$lib/components/Back.svelte';
	import { chapterLogics, displayName } from '$lib/logic.js';

	const { data } = $props();
	const { chapter, section, paragraph, exampleOrExamples } = $derived(data);
	const width = 574;
</script>

{#snippet openInSandbox(example: Example)}
	<a
		class="opacity-50"
		href="/sandbox?logic={chapterLogics[chapter]}&statement={example.premises}⊢{example.conclusion}"
	>
		⊢ Open in sandbox
	</a>
{/snippet}

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
		<div>
			<TableauEdit
				premises={exampleOrExamples.premises?.split(',') ?? []}
				conclusion={exampleOrExamples.conclusion}
				{width}
				{logic}
			/>

			<div class="text-center">
				{@render openInSandbox(exampleOrExamples)}
			</div>
		</div>
	{:else}
		{#each Object.entries(exampleOrExamples) as [key, example] (key)}
			{@const logic = logicOfChapter(chapter, example)}
			<div id={key} class="scroll-mt-6">
				<div class="flex items-baseline gap-2">
					<h2 class="text-2xl font-bold italic">({key})</h2>
					{@render openInSandbox(example)}
				</div>

				<TableauEdit
					premises={example.premises?.split(',') ?? []}
					conclusion={example.conclusion}
					{width}
					{logic}
				/>
			</div>
		{/each}
	{/if}
</main>
