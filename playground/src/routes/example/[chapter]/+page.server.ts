import { examples } from '../index.server';

export const load = async ({ params }) => {
	const { chapter } = params;
	const chapterExamples = examples[chapter];

	return {
		chapter,
		chapterExamples
	};
};
