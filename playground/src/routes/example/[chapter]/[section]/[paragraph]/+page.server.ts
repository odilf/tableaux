import { examples } from '../../../index.server';

export const load = async ({ params }) => {
	const { chapter, section, paragraph } = params;
	const exampleOrExamples = examples[chapter][section][paragraph];
	return { exampleOrExamples, chapter, section, paragraph };
};
