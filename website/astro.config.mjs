import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';
import { unified } from '@astrojs/markdown-remark';
import { navigation } from './src/navigation.mjs';
import { explicitHeadingIds } from './scripts/headings.mjs';

export default defineConfig({
  site: 'https://eimg.github.io',
  base: '/lime',
  trailingSlash: 'always',
  markdown: { processor: unified({ remarkPlugins: [explicitHeadingIds] }) },
  integrations: [starlight({
    title: 'Lime',
    description: 'Practical image editing guides in English and Myanmar.',
    logo: { src: './public/icon.svg', replacesTitle: false },
    favicon: '/icon.svg',
    defaultLocale: 'root',
    locales: {
      root: { label: 'English', lang: 'en' },
      mm: { label: 'မြန်မာ', lang: 'my' },
    },
    social: [{ icon: 'github', label: 'GitHub', href: 'https://github.com/eimg/lime' }],
    sidebar: navigation.map(({ label, my, pages }) => ({
      label,
      translations: { my },
      collapsed: false,
      items: pages.map((slug) => ({ slug })),
    })),
    customCss: ['./src/styles/custom.css'],
    components: {
      Footer: './src/components/Footer.astro',
      LanguageSelect: './src/components/LanguageSelect.astro',
    },
    lastUpdated: false,
    pagination: true,
    editLink: { baseUrl: 'https://github.com/eimg/lime/edit/main/website/' },
  })],
});
