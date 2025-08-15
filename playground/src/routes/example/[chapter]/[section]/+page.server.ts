import { redirect } from '@sveltejs/kit';
import { examples } from '../../index.server';

export const load = async ({ params }) => {
	const { chapter, section } = params;

	const sectionExamples = examples[chapter][section];
	if (Object.keys(sectionExamples).length === 1) {
		const paragraph = Object.keys(sectionExamples)[0];
		throw redirect(302, `/example/${chapter}/${section}/${paragraph}`);
	} else {
		throw redirect(302, `/example/${chapter}/`);
	}
};
