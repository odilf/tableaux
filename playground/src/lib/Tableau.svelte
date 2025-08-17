<script lang="ts">
	import { Node, Tableau } from '$rust';
	import * as d3 from 'd3';

	let {
		tableau = $bindable(),
		// FIXME: we should get the initial values from the parent or something.
		// Right now, the height does't really matter and the width is just `--container-xl`
		// from tailwind, which is a bit bodgy.
		width,
		height,
		editable = false,
		minLineHeight = 40,
		margin = { top: 40, right: 40, bottom: 40, left: 40 }
	}: {
		tableau: Tableau;
		width: number;
		height?: number;
		editable?: boolean;
		minLineHeight?: number;
		margin?: { top: number; right: number; bottom: number; left: number };
	} = $props();

	const actualHeight = $derived(
		Math.max(height ?? 0, tableau.depth() * minLineHeight + margin.top + margin.bottom)
	);

	let root = $derived(
		d3.hierarchy({ value: tableau.get(tableau.root()), id: tableau.root() }, (node) =>
			Array.from(tableau.children(node.id)).map((child) => ({
				value: tableau.get(child),
				id: child
			}))
		)
	);

	const linePadding = { top: 10, bottom: 12 };

	let d3Tree = $derived(
		d3
			.tree<{ value: Node; id: number }>()
			.size([width - margin.left - margin.right, actualHeight - margin.top - margin.bottom])(root)
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
		{y}
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
			{@render tree(child, yOffset)}
		{/each}
	{:else if isDead}
		<text {x} y={y + 20}> ╳ </text>
	{/if}
{/snippet}

<svg viewBox="0 0 {width} {actualHeight}" class="font-math w-full">
	<g transform="translate({margin.left}, {margin.top})" fill="currentColor">
		<g text-anchor="middle" dominant-baseline="middle">
			{@render tree(d3Tree)}
		</g>

		<g stroke="currentColor" stroke-width="1pt" stroke-linecap="round">
			{#each root.links() as link (link)}
				{#if Math.abs((link.source.x ?? 0) - (link.target.x ?? 0)) > 0.01}
					<line
						x1={link.source.x}
						y1={(link.source.y ?? 0) + linePadding.top}
						x2={link.target.x}
						y2={(link.target.y ?? 0) - linePadding.bottom}
					/>
				{/if}
			{/each}
		</g>
	</g>
</svg>
