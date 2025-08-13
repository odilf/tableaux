<script>
	import Tableau from '$lib/Tableau.svelte';
	import { Classical, Logic } from '$rust';

	const links = [
		{ name: 'Sandbox', href: '/sandbox', description: "Edit to your heart's contnent" },
		{
			name: 'Examples',
			href: '/example',
			description: `From Graham Priest's <em>Introduction to Non-Classical Logics</em>`
		}
	];

	const tableau = Logic.classical().tableau(['p > q', 'q > r'], 'p > r').inferred();
</script>

<h1 class="mt-8 text-5xl font-bold">Tableaux playground</h1>
<p class="opacity-80">For non-classical logics</p>

<ul class="mt-4 flex flex-col gap-2 leading-none">
	{#each links as { name, href, description } (href)}
		<li>
			<a href="/example">
				<h2 class="text-xl font-bold">⊃ {name}</h2>
				<!-- eslint-disable-next-line svelte/no-at-html-tags -->
				<p class="ml-5 opacity-80">{@html description}</p>
			</a>
		</li>
	{/each}
</ul>

<div class="flex-1"></div>

<a href="/todo">
	<Tableau {tableau} />
	<p class="text-center opacity-50">Example: Transitivity across worlds in modal logic</p>
</a>

<div class="flex-1"></div>
