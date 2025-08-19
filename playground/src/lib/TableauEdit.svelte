<script lang="ts">
	import { Logic } from '$rust';
	import Concludes from './Concludes.svelte';
	import Tableau from './Tableau.svelte';
	import type { ComponentProps } from 'svelte';

	type Props = {
		logic: Logic;
		premises: string;
		conclusion: string;
	} & Omit<ComponentProps<typeof Tableau>, 'tableau' | 'editable'>;

	let {
		logic,
		premises = $bindable(''),
		conclusion = $bindable(''),
		...tableauProps
	}: Props = $props();

	let tableauResult = $derived.by(() => {
		try {
			const t = logic.tableau(
				premises.split(',').filter((p) => p.trim().length > 0),
				conclusion
			);
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
		class="font-math grid grid-cols-[1fr_auto_1fr] place-content-center place-items-center gap-2"
	>
		<textarea
			id="premises"
			name="premises"
			rows="1"
			class="w-full rounded border text-center"
			bind:value={premises}
		>
		</textarea>
		<div class="font-bold">
			<Concludes holds={tableauResult.ok?.holds()} {logic} />
		</div>
		<textarea
			id="conclusion"
			name="conclusion"
			rows="1"
			class="w-full rounded border text-center"
			bind:value={conclusion}
		>
		</textarea>
	</div>
</div>

{#if tableauResult.ok !== undefined}
	<Tableau tableau={tableauResult.ok} editable={true} {...tableauProps} />
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
