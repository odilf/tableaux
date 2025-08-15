import { symbolAsciiStr, symbolChar, symbolIter } from '$rust';
import fs from 'node:fs/promises';
import { parseExampleFile, singleExampleSchema, type Example, type Examples } from '.';
import { is } from 'valibot';

export function replaceAsciiWithUnicode(text: string) {
	for (const symbol of symbolIter()) {
		console.log('replacing ', symbolAsciiStr(symbol), ' with ', symbolChar(symbol));
		text = text.replaceAll(symbolAsciiStr(symbol), symbolChar(symbol));
	}

	return text;
}

export function replaceAllAsciiWithUnicode(examples: Examples) {
	const replace = (example: Example) => {
		if (example.premises) {
			example.premises = replaceAsciiWithUnicode(example.premises);
		}
		example.conclusion = replaceAsciiWithUnicode(example.conclusion);
	};

	for (const chapter in examples) {
		for (const section in examples[chapter]) {
			for (const paragraph in examples[chapter][section]) {
				if (is(singleExampleSchema, examples[chapter][section][paragraph])) {
					replace(examples[chapter][section][paragraph]);
				} else {
					for (const key in examples[chapter][section][paragraph]) {
						replace(examples[chapter][section][paragraph][key]);
					}
				}
			}
		}
	}
}

// const tomlData = await fs.readFile('../examples-graham-priest.toml', { encoding: 'utf8' });
const tomlData = replaceAsciiWithUnicode(
	await fs.readFile('../examples-graham-priest.toml', { encoding: 'utf8' })
);
export const examples = parseExampleFile(tomlData);

// // @ts-expect-error Excessive or infinite recursion detected, but it's clearly ok.
// replaceAllAsciiWithUnicode(examples);
