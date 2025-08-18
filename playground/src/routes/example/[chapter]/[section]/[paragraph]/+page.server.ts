export const prerender = true;

import { error } from '@sveltejs/kit';

export const entries = async () => {
	const { examples } = await import('$lib/examples/index.server');

	return Object.entries(examples).flatMap(([chapter, chapterExamples]) =>
		Object.entries(chapterExamples).flatMap(([section, sectionExamples]) =>
			Object.keys(sectionExamples).map((paragraph) => ({ chapter, section, paragraph }))
		)
	);
};

export const load = async ({ params }) => {
	const { validateChapter } = await import('$lib/examples');
	const { examples } = await import('$lib/examples/index.server');

	const { section, paragraph } = params;
	const chapter = validateChapter(params.chapter) ?? error(400, 'Invalid chapter');

	const exampleOrExamples = examples[chapter][section][paragraph];

	if (!exampleOrExamples) {
		throw error(404, 'Example not found');
	}

	return {
		exampleOrExamples,
		chapter,
		section,
		paragraph
	};
};
