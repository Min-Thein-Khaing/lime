import { readFile, readdir, stat } from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { load } from 'cheerio';
import { pageKeys } from '../src/navigation.mjs';

const dist = path.resolve(fileURLToPath(new URL('../dist/', import.meta.url)));
const prefix = '/lime/';

export function resolveLocalReference(reference, currentUrl) {
  if (!reference || /^(?:mailto:|tel:|data:|javascript:)/i.test(reference)) return null;
  const url = new URL(reference, currentUrl);
  if (url.origin !== 'https://eimg.github.io') return null;
  if (!url.pathname.startsWith(prefix)) throw new Error(`Link leaves project base: ${url.pathname}`);
  const relative = decodeURIComponent(url.pathname.slice(prefix.length));
  const file = path.resolve(dist, relative, url.pathname.endsWith('/') ? 'index.html' : '');
  if (!file.startsWith(`${dist}${path.sep}`)) throw new Error(`Link leaves output directory: ${reference}`);
  return { file, hash: decodeURIComponent(url.hash.slice(1)), url };
}

export async function checkBuild() {
  const errors = [];
  const cache = new Map();
  async function html(file) {
    if (!cache.has(file)) cache.set(file, load(await readFile(file, 'utf8')));
    return cache.get(file);
  }
  let linkCount = 0;
  for (const key of pageKeys) {
    for (const locale of ['', 'mm/']) {
      const route = `${locale}${key === 'index' ? '' : `${key}/`}`;
      const filename = path.join(dist, route, 'index.html');
      const currentUrl = `https://eimg.github.io${prefix}${route}`;
      const $ = await html(filename);
      const lang = locale ? 'my' : 'en';
      if ($('html').attr('lang') !== lang) errors.push(`${route}: wrong HTML language`);
      if (!$('h1').text().trim() || !$('title').text().trim()) errors.push(`${route}: missing title`);
      if (!$('meta[name="description"]').attr('content')) errors.push(`${route}: missing description`);
      const themeSelectors = $('starlight-theme-select select');
      if (!themeSelectors.length) errors.push(`${route}: missing theme selector`);
      themeSelectors.each((_, select) => {
        const labels = $(select).find('option').map((_, option) => $(option).text().trim()).get();
        if (JSON.stringify(labels) !== JSON.stringify(['Dark', 'Light', 'Auto'])) errors.push(`${route}: theme selector labels must stay in English`);
      });
      if ($('link[rel="canonical"]').attr('href') !== currentUrl) errors.push(`${route}: incorrect canonical URL`);
      const counterpart = `${prefix}${locale ? '' : 'mm/'}${key === 'index' ? '' : `${key}/`}`;
      if (!$('[data-guide-language]').toArray().some((el) => $(el).attr('href') === counterpart)) errors.push(`${route}: missing language counterpart`);
      for (const [language, target] of [['en', `${prefix}${key === 'index' ? '' : `${key}/`}`], ['my', `${prefix}mm/${key === 'index' ? '' : `${key}/`}`]]) {
        if ($(`link[rel="alternate"][hreflang="${language}"]`).attr('href') !== `https://eimg.github.io${target}`) errors.push(`${route}: incorrect ${language} alternate URL`);
      }
      if ($('body').text().match(/\{#[a-z][a-z0-9-]*\}/)) errors.push(`${route}: unprocessed explicit heading ID`);
      const ids = new Set();
      $('[id]').each((_, el) => {
        const id = $(el).attr('id');
        if (ids.has(id)) errors.push(`${route}: duplicate ID ${id}`);
        ids.add(id);
      });
      for (const el of $('a[href], img[src], script[src], link[href]').toArray()) {
        const ref = $(el).attr(el.name === 'a' || el.name === 'link' ? 'href' : 'src');
        try {
          const local = resolveLocalReference(ref, currentUrl);
          if (!local) continue;
          linkCount++;
          const info = await stat(local.file);
          if (!info.isFile()) throw new Error(`not a file: ${local.url.pathname}`);
          if (local.hash && local.file.endsWith('.html')) {
            const target = await html(local.file);
            const found = target('[id]').toArray().some((node) => target(node).attr('id') === local.hash);
            if (!found) throw new Error(`missing fragment: ${local.url.pathname}#${local.hash}`);
          }
        } catch (error) { errors.push(`${route}: ${ref}: ${error.message}`); }
      }
    }
  }
  const searchFiles = await readdir(path.join(dist, 'pagefind'));
  for (const lang of ['en', 'my']) {
    if (!searchFiles.some((file) => file.startsWith(`pagefind.${lang}_`))) errors.push(`Missing ${lang} search index`);
  }
  if (errors.length) throw new Error(errors.join('\n'));
  console.log(`Build OK: ${pageKeys.length * 2} pages, ${linkCount} local links/assets, English/Myanmar search indexes.`);
}

if (process.argv[1] && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  try { await checkBuild(); }
  catch (error) { console.error(error.message); process.exitCode = 1; }
}
