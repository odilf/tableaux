import fs from 'node:fs';
import { EXAMPLES_GRAHAM_PRIEST_PATH } from '$env/static/public';
import { parseExampleFile, replaceAsciiWithUnicode } from '.';

export const examples = parseExampleFile(
	replaceAsciiWithUnicode(fs.readFileSync(EXAMPLES_GRAHAM_PRIEST_PATH, { encoding: 'utf8' }))
);

console.log('Loaded examples: ', examples);
