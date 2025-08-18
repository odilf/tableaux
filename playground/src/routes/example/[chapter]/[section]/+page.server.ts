export const prerender = true;

import { error, redirect } from '@sveltejs/kit';

const { validateChapter } = await import('$lib/examples');
const { examples } = await import('$lib/examples/index.server');

export const entries = () =>
	Object.entries(examples).flatMap(([chapter, chapterExamples]) =>
		Object.keys(chapterExamples).map((section) => ({ chapter, section }))
	);

export const load = async ({ params }) => {
	const { chapter, section } = params;

	const sectionExamples =
		examples[validateChapter(chapter) ?? error(400, 'Invalid chapter')][section];
	if (Object.keys(sectionExamples).length === 1) {
		const paragraph = Object.keys(sectionExamples)[0];
		throw redirect(302, `/example/${chapter}/${section}/${paragraph}`);
	} else {
		throw redirect(302, `/example/${chapter}/`);
	}
};
