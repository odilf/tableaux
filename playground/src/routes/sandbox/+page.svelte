<script lang="ts">
	import { classObject, displayName, logics, type LogicKind } from '$lib/logic';
	import TableauEdit from '$lib/TableauEdit.svelte';
	import { Logic, symbolAsciiStr, symbolChar, symbolName } from '$rust';
	import { expoOut } from 'svelte/easing';
	import { fly, slide } from 'svelte/transition';

	let selected: LogicKind = $state('classical');

	let reflexive = $state(false);
	let symmetric = $state(false);
	let transitive = $state(false);
	let extendable = $state(false);

	let logic = $derived.by(() => {
		if (selected === 'classical') {
			return Logic.classical();
		} else if (selected === 'modal') {
			return Logic.modal();
		} else if (selected === 'normalModal') {
			return Logic.normalModal(reflexive, symmetric, transitive, extendable);
		} else {
			throw new Error('Unreachable');
		}
	});
</script>

<main class="flex h-screen">
	<div class="text-md flex w-50 flex-col bg-neutral-secondary p-3 text-secondary">
		<a href="/" class="mb-4 opacity-50">⟞ Home </a>
		<h2 class="mb-2 text-3xl font-bold">Logic type</h2>
		<ul class="flex flex-col gap-2">
			{#each logics as logic (logic)}
				<li class="w-full">
					<button onclick={() => (selected = logic)} class="w-full" disabled={selected === logic}>
						{displayName[logic]}
					</button>
				</li>
			{/each}
		</ul>

		{#if selected === 'normalModal'}
			<div class="mt-2 flex flex-col gap-2" transition:slide={{ easing: expoOut }}>
				<label>
					<input type="checkbox" bind:checked={reflexive} />
					Reflexive
				</label>

				<label>
					<input type="checkbox" bind:checked={symmetric} />
					Symmetric
				</label>

				<label>
					<input type="checkbox" bind:checked={transitive} />
					Transitive
				</label>

				<label>
					<input type="checkbox" bind:checked={extendable} />
					Extensible
				</label>
			</div>
		{/if}

		<h2 class="mt-6 mb-2 text-xl font-bold">Symbols</h2>
		<div class="font-math grid grid-cols-[1fr_1fr_3fr] gap-2">
			<div class="text-sm opacity-50">Sym.</div>
			<div class="text-sm opacity-50">ASCII</div>
			<div class="text-sm opacity-50">Name</div>

			{#each classObject[selected].symbols() as symbol, i (symbol)}
				<button
					transition:fly={{ duration: 10 + i * 100, x: 20 }}
					class="grid place-content-center rounded bg-black/20 p-1 text-center"
					onclick={() => navigator.clipboard.writeText(symbolChar(symbol))}
				>
					{symbolChar(symbol)}
				</button>

				<button
					transition:fly={{ duration: 10 + i * 100, x: 20 }}
					class="grid place-content-center rounded bg-black/20 p-1 text-center"
					onclick={() => navigator.clipboard.writeText(symbolChar(symbol))}
				>
					{symbolAsciiStr(symbol)}
				</button>

				<div transition:fly={{ duration: 10 + i * 100, x: 20 }} class="flex items-center text-sm">
					{symbolName(symbol)}
				</div>
			{/each}
		</div>
	</div>

	<div class="flex-1 p-4">
		<TableauEdit {logic} premises="p > q, q > r" conclusion="p > r" width={1000} />
	</div>
</main>

<style lang="postcss">
	@reference "../../app.css";

	button {
		@apply rounded bg-primary/20 p-1 shadow transition-colors hover:bg-primary/40 active:bg-primary/60 disabled:bg-primary/70;
	}
</style>
