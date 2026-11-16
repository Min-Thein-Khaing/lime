import { readFile, readdir, writeFile } from 'node:fs/promises';
import { createHash } from 'node:crypto';
import { fileURLToPath } from 'node:url';
import path from 'node:path';
import { parse } from 'yaml';
import { pageKeys } from '../src/navigation.mjs';

export const root = fileURLToPath(new URL('..', import.meta.url));
const docs = path.join(root, 'src/content/docs');
const digest = (text) => createHash('sha256').update(text).digest('hex');

export function parsePage(text, filename) {
  const match = text.match(/^---\r?\n([\s\S]*?)\r?\n---(?:\r?\n|$)/);
  if (!match) throw new Error(`${filename}: missing frontmatter`);
  const data = parse(match[1]);
  const body = text.slice(match[0].length);
  const headings = [...body.matchAll(/^##\s+(.+)$/gm)].map(([line, title]) => {
    const id = title.match(/\{#([a-z][a-z0-9-]*)\}\s*$/)?.[1];
    if (!id) throw new Error(`${filename}: H2 needs a stable explicit ID: ${line}`);
    return id;
  });
  if (new Set(headings).size !== headings.length) throw new Error(`${filename}: duplicate section ID`);
  return { data, body, headings, hash: digest(text) };
}

export function validatePair(key, en, my) {
  for (const [locale, page] of [['en', en], ['my', my]]) {
    if (!page) throw new Error(`${key}: missing ${locale} translation`);
    for (const field of ['title', 'description', 'sourceRevision', 'translatedRevision']) {
      if (typeof page.data[field] !== 'string' || !page.data[field].trim()) throw new Error(`${key} (${locale}): missing ${field}`);
    }
    if (page.data.translationKey !== key) throw new Error(`${key} (${locale}): incorrect translationKey`);
    if (page.data.translatedRevision !== en.data.sourceRevision || page.data.sourceRevision !== en.data.sourceRevision) throw new Error(`${key} (${locale}): stale translation revision`);
    if (key !== 'index' && page.headings.length < 2) throw new Error(`${key} (${locale}): article needs substantive sections`);
  }
  if (!/[\u1000-\u109f]/u.test(key === 'index' ? my.data.title : my.body)) throw new Error(`${key}: Myanmar body is not translated`);
  if (JSON.stringify(en.headings) !== JSON.stringify(my.headings)) throw new Error(`${key}: English/Myanmar section IDs do not match`);
}

async function filesIn(directory) {
  const entries = await readdir(directory, { withFileTypes: true });
  return (await Promise.all(entries.map((item) => item.isDirectory() ? filesIn(path.join(directory, item.name)) : path.join(directory, item.name)))).flat();
}

export async function checkContent({ record = false } = {}) {
  const files = (await filesIn(docs)).filter((file) => /\.mdx?$/.test(file));
  const pages = new Map();
  for (const file of files) {
    const route = path.relative(docs, file).replace(/\\/g, '/').replace(/\.mdx?$/, '');
    if (pages.has(route)) throw new Error(`Duplicate content route: ${route}`);
    pages.set(route, parsePage(await readFile(file, 'utf8'), route));
  }
  const expected = pageKeys.flatMap((key) => [key, `mm/${key}`]);
  const extra = [...pages.keys()].filter((route) => !expected.includes(route));
  if (extra.length) throw new Error(`Pages missing from the navigation manifest: ${extra.join(', ')}`);
  const signatures = {};
  for (const key of pageKeys) {
    const en = pages.get(key);
    const my = pages.get(`mm/${key}`);
    validatePair(key, en, my);
    signatures[key] = { en: en.hash, my: my.hash };
  }
  const ledger = path.join(root, 'translations.json');
  if (record) {
    await writeFile(ledger, `${JSON.stringify(signatures, null, 2)}\n`);
    console.log(`Recorded ${pageKeys.length} English/Myanmar article pairs.`);
  } else {
    const recorded = JSON.parse(await readFile(ledger, 'utf8'));
    for (const key of pageKeys) {
      if (JSON.stringify(recorded[key]) !== JSON.stringify(signatures[key])) throw new Error(`${key}: content changed since translation recording; update both languages, review, then run npm run translations:record`);
    }
    if (Object.keys(recorded).length !== pageKeys.length) throw new Error('Translation ledger contains removed pages.');
    console.log(`Content OK: ${pages.size} pages, paired sections and current translation records.`);
  }
  return signatures;
}

if (process.argv[1] && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  try { await checkContent({ record: process.argv.includes('--record') }); }
  catch (error) { console.error(error.message); process.exitCode = 1; }
}
