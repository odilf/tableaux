<script>
	import TableauEdit from '$lib/TableauEdit.svelte';
	import { Classical, Logic } from '$rust';
</script>

<h1>Classic transitivity</h1>

<TableauEdit
	logic={Logic.classical(new Classical())}
	premises={['p > q', 'q > r']}
	conclusion="p > !r"
/>
