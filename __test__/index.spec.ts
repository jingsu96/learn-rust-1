import test from 'ava';
import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

import { plus100, analysisSourceCode } from '../index';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

test('sync function from native code', (t) => {
  const fixture = 42;
  t.is(plus100(fixture), fixture + 100);
});

test('analysis source code', (t) => {
  const sourceCode = fs.readFileSync(path.resolve(__dirname, 'mock.js'), 'utf8');
  const result = analysisSourceCode(sourceCode);
  t.deepEqual(result, {
    variableDeclarations: 4,
    functionDeclarations: 10,
    classDeclarations: 1,
    exportDeclarations: 8,
  });
});
