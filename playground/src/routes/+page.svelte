<script>
	import Tableau from '$lib/Tableau.svelte';
	import { Logic } from '$rust';

	const links = [
		{ name: 'Sandbox', href: '/sandbox', description: "Edit to your heart's content." },
		{
			name: 'Examples',
			href: '/example',
			description: `From Graham Priest's <em>Introduction to Non-Classical Logics</em>.`
		},
		{
			name: 'About',
			href: '/about',
			description: 'What are tableaux and how was this made.'
		}
	];

	const tableau = Logic.classical().tableau(['p > q', 'q > r'], 'p > r').inferred();
</script>

<main class="mx-auto flex h-screen max-w-xl flex-col p-4 font-serif">
	<h1 class="mt-8 mb-2 text-5xl font-bold">Tableaux playground</h1>
	<p class="opacity-80">For non-classical logics</p>

	<ul class="mt-4 flex flex-col gap-3 leading-none">
		{#each links as { name, href, description } (href)}
			<li>
				<a {href}>
					<h2 class="text-xl font-bold">⊢ {name}</h2>
					<!-- eslint-disable-next-line svelte/no-at-html-tags -->
					<p class="ml-5 text-balance opacity-80">{@html description}</p>
				</a>
			</li>
		{/each}
	</ul>

	<div class="flex-1"></div>

	<a href="/todo">
		<Tableau {tableau} />
		<p class="text-center text-balance opacity-50">
			Example: Transitivity across worlds in modal logic
		</p>
	</a>

	<div class="flex-1"></div>
</main>
