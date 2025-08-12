<script lang="ts">
	import { Logic } from '$rust';
	import Tableau from './Tableau.svelte';
	type Props = {
		logic: Logic;
		premises: string[];
		conclusion: string;
	};

	let { logic = $bindable(), premises = $bindable(), conclusion = $bindable() }: Props = $props();

	let tableau = $derived.by(() => {
		const t = logic.tableau(premises, conclusion);
		t.infer();
		return t;
	});
</script>

<label for="premises">Premises</label>
<textarea id="premises" name="premises" rows="1" cols="33"> {premises} </textarea>

<div>⊢</div>
<label for="conclusion">Conclusion</label>
<textarea id="conclusion" name="conclusion" rows="1" cols="33" bind:value={conclusion}> </textarea>

{conclusion}
<Tableau {tableau} />
