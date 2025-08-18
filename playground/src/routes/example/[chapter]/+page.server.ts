export const prerender = true;

import { error } from '@sveltejs/kit';

export const entries = async () => {
	const { examples } = await import('$lib/examples/index.server');
	return Object.keys(examples).map((chapter) => ({ chapter }));
};

export const load = async ({ params }) => {
	const { examples } = await import('$lib/examples/index.server');
	const { validateChapter } = await import('$lib/examples');

	const chapter = validateChapter(params.chapter) ?? error(400, 'Invalid chapter');
	const chapterExamples = examples[chapter];

	return {
		chapter,
		chapterExamples
	};
};
