export const prerender = true;

export const load = async () => {
	const { examples } = await import('$lib/examples/index.server');
	return {
		examples
	};
};
