<script lang="ts">
	import { is } from 'valibot';
	import { logicOfChapter, singleExampleSchema, type Example, type Examples } from '$lib/examples';
	import Concludes from '$lib/Concludes.svelte';
	import { chapterLogics } from '$lib/logic';

	const { chapter, chapterExamples }: { chapter: number; chapterExamples: Examples[string] } =
		$props();
</script>

{#snippet exampleSnippet(
	example: Example,
	chapter: number,
	section: string,
	paragraph: string,
	index?: string
)}
	<a class="flex gap-2" href="/example/{chapter}/{section}/{paragraph}{index ? '#' + index : ''}">
		<div class="font-bold">
			{chapter}.{section}.{paragraph}
			<em class="font-normal"
				>{#if index}
					{index})
				{/if}</em
			>
		</div>

		<span>
			{example.premises}
			<span class="font-bold">
				<Concludes holds={example.holds ?? true} logic={logicOfChapter(chapter, example)} />
			</span>
			{example.conclusion}
		</span>
	</a>
{/snippet}

{#each Object.entries(chapterExamples) as [section, sectionExamples] (section)}
	{#each Object.entries(sectionExamples) as [paragraph, exampleOrExamples] (paragraph)}
		{#if section === 'exercise'}
			<h3 class="text-xl font-bold">
				<a href="/example/{chapter}/{section}">Exercise {paragraph}</a>
			</h3>

			<ol>
				{#each Object.entries(exampleOrExamples as Record<string, Example>) as [key, example] (key)}
					<li>
						<a href="/example/{chapter}/{section}/{paragraph}#{key}">
							<span class="mx-2 font-bold italic">
								({key})
							</span>
							<span>
								{example.premises}
								<span class="font-bold">
									<Concludes
										holds={example.holds ?? true}
										logic={logicOfChapter(chapter, example)}
									/>
								</span>
								{example.conclusion}
							</span>
						</a>
					</li>
				{/each}
			</ol>
		{:else if is(singleExampleSchema, exampleOrExamples)}
			{@render exampleSnippet(exampleOrExamples, chapter, section, paragraph)}
		{:else}
			{#each Object.entries(exampleOrExamples) as [index, example] (index)}
				{@render exampleSnippet(example, chapter, section, paragraph, index)}
			{/each}
		{/if}
	{/each}
{/each}
