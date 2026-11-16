import test from 'node:test';
import assert from 'node:assert/strict';
import { parsePage, validatePair } from './check-content.mjs';
import { resolveLocalReference } from './check-build.mjs';
import { explicitHeadingIds } from './headings.mjs';

function page(body = '## Steps {#steps}\nText\n## Result {#result}\nText', title = 'Guide') {
  return parsePage(`---\ntitle: ${title}\ndescription: A complete guide\ntranslationKey: start/test\nsourceRevision: r1\ntranslatedRevision: r1\n---\n${body}`, 'test.md');
}
const english = () => page();
const myanmar = () => page('## လုပ်ဆောင်ရန် {#steps}\nစာသား\n## ရလဒ် {#result}\nစာသား', 'လမ်းညွှန်');

test('publication requires a complete translation with matching sections and revision', () => {
  assert.doesNotThrow(() => validatePair('start/test', english(), myanmar()));
  assert.throws(() => validatePair('start/test', english(), undefined), /missing my/);
  const stale = myanmar(); stale.data.translatedRevision = 'r0';
  assert.throws(() => validatePair('start/test', english(), stale), /stale translation/);
  const mismatched = myanmar(); mismatched.headings = ['steps', 'different'];
  assert.throws(() => validatePair('start/test', english(), mismatched), /section IDs/);
  assert.throws(() => validatePair('start/test', english(), english()), /not translated/);
});

test('missing or duplicated permanent section anchors are rejected', () => {
  assert.throws(() => page('## Steps'), /stable explicit ID/);
  assert.throws(() => page('## Steps {#steps}\n## Again {#steps}'), /duplicate section/);
});

test('translated heading labels retain their explicit permanent link', () => {
  const heading = { type: 'heading', children: [{ type: 'text', value: 'လုပ်ဆောင်ရန် {#steps}' }] };
  explicitHeadingIds()({ type: 'root', children: [heading] });
  assert.equal(heading.data.hProperties.id, 'steps');
  assert.equal(heading.children[0].value, 'လုပ်ဆောင်ရန်');
});

test('relative links retain the locale and GitHub Pages project prefix', () => {
  const target = resolveLocalReference('../../basics/layers/#locks', 'https://eimg.github.io/lime/mm/start/workspace/');
  assert.equal(target.url.pathname, '/lime/mm/basics/layers/');
  assert.equal(target.hash, 'locks');
  assert.equal(resolveLocalReference('https://github.com/eimg/lime', 'https://eimg.github.io/lime/'), null);
  assert.throws(() => resolveLocalReference('/start/workspace/', 'https://eimg.github.io/lime/'), /project base/);
});
