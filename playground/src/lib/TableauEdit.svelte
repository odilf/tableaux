<script lang="ts">
	import { Logic } from '$rust';
	import Tableau from './Tableau.svelte';
	type Props = {
		logic: Logic;
		premises: string;
		conclusion: string;
	};

	let { logic = $bindable(), premises = $bindable(), conclusion = $bindable() }: Props = $props();

	let tableauResult = $derived.by(() => {
		try {
			const t = logic.tableau(premises.split(','), conclusion);
			t.infer();
			return { ok: t };
		} catch (error) {
			return { error };
		}
	});
</script>

<div>
	<div class="grid grid-cols-2 place-items-center font-bold">
		<label for="premises">Premises</label>
		<label for="conclusion">Conclusion</label>
	</div>

	<div
		class="grid grid-cols-[1fr_auto_1fr] place-content-center place-items-center gap-2 font-math"
	>
		<textarea
			id="premises"
			name="premises"
			rows="1"
			class="w-full text-center"
			bind:value={premises}
		>
		</textarea>
		<div class="font-bold">⊢</div>
		<textarea
			id="conclusion"
			name="conclusion"
			rows="1"
			class="w-full text-center"
			bind:value={conclusion}
		>
		</textarea>
	</div>
</div>

{#if tableauResult.ok !== undefined}
	<Tableau tableau={tableauResult.ok} />
{:else if 'error' in tableauResult}
	<div>
		Error: <pre>{tableauResult.error}</pre>
	</div>
{/if}

<style>
	textarea {
		resize: none;
	}
</style>
