<script lang="ts">
	import { Node, Tableau } from '$rust';
	import * as d3 from 'd3';

	let {
		tableau = $bindable(),
		width,
		height,
		editable = false,
		margin = { top: 20, right: 20, bottom: 20, left: 20 },
		crossOffset = 20,
		minLineHeight = 20,
		minExprWidth = 4,
		splitHeight = 35,
		linePadding = { top: 12, bottom: 14 }
	}: {
		tableau: Tableau;
		width: number;
		height?: number;
		editable?: boolean;
		margin?: { top: number; right: number; bottom: number; left: number };
		/** How much space between the leaf nodes and the crosses if they're dead */
		crossOffset?: number;
		/** Minimum line height for line, in case `height` is undefined or too small */
		minLineHeight?: number;
		/** Minimum width for an expression, in case `width` is undefined or too small */
		minExprWidth?: number;
		splitHeight?: number;
		/**
		 * How much extra space to leave between the top and bottom of lines that connect node.
		 * If this were 0 then the lines would come out of the center of nodes, which is undesirable.
		 */
		linePadding?: { top: number; bottom: number };
	} = $props();

	const d3Height = $derived(
		Math.max(
			height ?? 0,
			tableau.depth() * (minLineHeight + splitHeight) + margin.top + margin.bottom
		)
	);

	const actualHeight = $derived.by(() => {
		const heightOf = (node: number): number => {
			const children = tableau.children(node);
			if (children.length === 1) {
				return heightOf(children[0]) + minLineHeight;
			} else {
				return (d3.max(Array.from(children).map(heightOf)) ?? 0) + minLineHeight + splitHeight;
			}
		};

		return Math.max(heightOf(tableau.root()) + margin.top + margin.bottom, height ?? 0);
	});

	const d3Width = $derived.by(() => {
		const minFractionOf = (node: number): number => {
			const children = Array.from(tableau.children(node));
			return d3.min(children.map((child) => minFractionOf(child) / children.length)) ?? 1;
		};

		return Math.max(width, (1 / minFractionOf(tableau.root())) * minExprWidth);
	});

	let d3root = $derived(
		d3.hierarchy({ value: tableau.get(tableau.root()), id: tableau.root() }, (node) =>
			Array.from(tableau.children(node.id)).map((child) => ({
				value: tableau.get(child),
				id: child
			}))
		)
	);

	let d3Tree = $derived(
		d3
			.tree<{ value: Node; id: number }>()
			.size([d3Width - margin.left - margin.right, d3Height - margin.top - margin.bottom])(d3root)
	);
</script>

{#snippet tree(
	{ data, x, y, children }: d3.HierarchyPointNode<{ value: Node; id: number }>,
	yOffset = 0
)}
	{@const isDead = tableau.isDead(data.id)}
	{@const inferCurrent = () => {
		if (!editable) {
			return;
		}
		tableau.inferNode(data.id);
		// Trigger update...
		// See https://github.com/sveltejs/svelte/issues/14520
		const tmp = tableau;
		tableau = null as unknown as Tableau;
		tableau = tmp;
	}}
	<text
		{x}
		y={y + yOffset}
		font-weight={isDead ? 'normal' : 'bold'}
		onclick={inferCurrent}
		onkeydown={(event) => {
			if (event.key === 'Enter') {
				inferCurrent();
			}
		}}
		role="button"
		tabindex={0}
	>
		{data.value}
	</text>

	{#if children !== undefined}
		<!-- eslint-disable-next-line svelte/require-each-key -->
		{#each children as child}
			{#if children.length > 1}
				<line
					stroke="currentColor"
					stroke-width="1pt"
					stroke-linecap="round"
					x1={x}
					y1={y + yOffset + linePadding.top}
					x2={child.x}
					y2={child.y + yOffset - linePadding.bottom}
				/>
			{/if}

			{@render tree(child, children.length === 1 ? yOffset - splitHeight : yOffset)}
		{/each}
	{:else if isDead}
		<text {x} y={yOffset + y + crossOffset}> ╳ </text>
	{/if}
{/snippet}

<svg viewBox="0 0 {d3Width} {actualHeight}" class="font-math w-full">
	<g transform="translate({margin.left}, {margin.top})" fill="currentColor">
		<g text-anchor="middle" dominant-baseline="middle">
			{@render tree(d3Tree)}
		</g>
	</g>
</svg>
