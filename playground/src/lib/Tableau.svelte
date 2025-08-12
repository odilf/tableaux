<script lang="ts">
	import { Node, Tableau } from '$rust';
	import * as d3 from 'd3';

	const { tableau }: { tableau: Tableau } = $props();

	let root = $derived(
		d3.hierarchy({ value: tableau.get(tableau.root()), id: tableau.root() }, (node) =>
			Array.from(tableau.children(node.id)).map((child) => ({
				value: tableau.get(child),
				id: child
			}))
		)
	);

	const width = 800;
	const height = 600;
	const margin = { top: 40, right: 40, bottom: 40, left: 40 };
	const linePadding = 15;

	let d3Tree = $derived(
		d3
			.tree<{ value: Node; id: number }>()
			.size([width - margin.left - margin.right, height - margin.top - margin.bottom])(root)
	);
</script>

{#snippet tree({ data, x, y, children }: d3.HierarchyPointNode<{ value: Node; id: number }>)}
	<text {x} {y}> {data.value} </text>

	{#if children !== undefined}
		<!-- eslint-disable-next-line svelte/require-each-key -->
		{#each children as child}
			{@render tree(child)}
		{/each}
	{/if}
{/snippet}

<svg {width} {height} viewBox="0 0 {width} {height}">
	<g transform="translate({margin.left}, {margin.top})">
		<g text-anchor="middle" dominant-baseline="middle">
			{@render tree(d3Tree)}
		</g>

		<g stroke="black" stroke-width="2" stroke-linecap="round">
			{#each root.links() as link (link)}
				<line
					x1={link.source.x}
					y1={(link.source.y ?? 0) + linePadding}
					x2={link.target.x}
					y2={(link.target.y ?? 0) - linePadding}
				/>
			{/each}
		</g>
	</g>
</svg>
