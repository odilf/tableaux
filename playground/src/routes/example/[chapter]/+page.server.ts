import { error } from '@sveltejs/kit';
import { validateChapter } from '..';
import { examples } from '../index.server';

export const entries = () => Object.keys(examples).map((chapter) => ({ chapter }));

export const load = async ({ params }) => {
	const chapter = validateChapter(params.chapter) ?? error(400, 'Invalid chapter');
	const chapterExamples = examples[chapter];

	return {
		chapter,
		chapterExamples
	};
};
