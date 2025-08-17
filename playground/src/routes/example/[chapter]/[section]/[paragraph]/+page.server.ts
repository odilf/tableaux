import { error } from '@sveltejs/kit';
import { validateChapter } from '../../..';
import { examples } from '../../../index.server';

export const entries = () =>
	Object.entries(examples).flatMap(([chapter, chapterExamples]) =>
		Object.entries(chapterExamples).flatMap(([section, sectionExamples]) =>
			Object.keys(sectionExamples).map((paragraph) => ({ chapter, section, paragraph }))
		)
	);

export const load = async ({ params }) => {
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
