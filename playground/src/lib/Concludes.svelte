<script lang="ts">
	import { Logic } from '$rust/tableaux';

	const { holds = false, logic = Logic.classical() } = $props();

	const inferSymbol = holds ? '⊢' : '⊬';
</script>

{inferSymbol} <sub class="inline-block -translate-x-1 text-[0.7rem]"> {logic.symbol()} </sub>
