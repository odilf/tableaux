<script>
	import Tableau from '$lib/Tableau.svelte';
	import { Logic } from '$rust';

	const links = [
		{ name: 'Sandbox', href: '/sandbox', description: "Edit to your heart's content." },
		{
			name: 'Examples',
			href: '/example',
			description: "From Graham Priest's <em>Introduction to Non-Classical Logics</em>."
		},
		{
			name: 'About',
			href: '/about',
			description: 'What are tableaux and how was this made.'
		}
	];

	const premises = ['□(A ⊃ B)', '□(B ⊃ C)'];
	const conclusion = '□(A ⊃ C)';
	// const premises = [];
	// const conclusion = '(□(A ⊃ B) && □(B ⊃ C)) > □(A ⊃ C)';
	const tableau = Logic.modal().tableau(premises, conclusion).inferred();
</script>

<main class="column flex h-screen flex-col">
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

	<div class="font-math mb-4 text-center text-xl">
		{premises}
		⊢
		{conclusion}
	</div>
	<Tableau
		{tableau}
		width={570}
		minLineHeight={30}
		editable={true}
		margin={{ top: 12, bottom: 12, left: 0, right: 0 }}
	/>
	<p class="text-center text-balance opacity-50">
		Example: Transitivity across worlds in modal logic. <br />
		<a href="/example/2/4/6"> Open in playground </a>
	</p>

	<div class="flex-1"></div>
</main>
