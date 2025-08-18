import { chapterLogics } from '$lib/logic';
import { Logic } from '$rust';
import TOML from 'smol-toml';
import * as v from 'valibot';
import { symbolAsciiStr, symbolChar, symbolIter } from '$rust';

export const singleExampleSchema = v.object({
	premises: v.optional(v.string()),
	conclusion: v.string(),
	holds: v.optional(v.boolean()),
	kind: v.optional(v.unknown())
});

export const examplesFileSchema = v.record(
	v.string(),
	v.record(
		v.string(),
		v.record(v.string(), v.union([singleExampleSchema, v.record(v.string(), singleExampleSchema)]))
	)
);

export function parseExampleFile(fileContent: string) {
	return v.parse(examplesFileSchema, TOML.parse(fileContent)) as unknown as Examples;
}

export type Example = v.InferInput<typeof singleExampleSchema>;

// NOTE: Should be the same as the inferred type, but we get excessive
// recursion errors. Also this way it's more clear what the keys are.
// export type Example = v.InferInput<typeof examplesFileSchema>;
export type Examples = {
	[chapter: string]: {
		[section: string]: {
			[paragraph: string]: Example | Record<string, Example>;
		};
	};
};

export const normalModalKind = v.object({
	reflexive: v.optional(v.boolean()),
	symmetric: v.optional(v.boolean()),
	transitive: v.optional(v.boolean()),
	extendable: v.optional(v.boolean())
});

export function logicOfChapter(chapter: number, example: Example) {
	if (chapter === 1) {
		return Logic.classical();
	} else if (chapter === 2) {
		return Logic.modal();
	} else if (chapter === 3) {
		const { reflexive, symmetric, transitive, extendable } = v.parse(normalModalKind, example.kind);
		return Logic.normalModal(
			reflexive ?? false,
			symmetric ?? false,
			transitive ?? false,
			extendable ?? false
		);
	} else {
		throw new Error(`Invalid chapter: ${chapter}`);
	}
}

export function validateChapter(chapter: string): number | null {
	const output = parseInt(chapter);
	if (isNaN(output)) {
		return null;
	}

	if (output < 1 || output > Object.keys(chapterLogics).length) {
		return null;
	}

	return output;
}

export function replaceAsciiWithUnicode(text: string) {
	for (const symbol of symbolIter()) {
		console.log('replacing ', symbolAsciiStr(symbol), ' with ', symbolChar(symbol));
		text = text.replaceAll(symbolAsciiStr(symbol), symbolChar(symbol));
	}

	return text;
}
