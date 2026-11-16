// One page manifest drives navigation and bilingual publication checks.
export const navigation = [
  { label: 'Start here', my: 'စတင်ရန်', pages: ['start/install', 'start/workspace', 'start/first-edit', 'start/new-image'] },
  { label: 'Editing basics', my: 'အခြေခံ ပုံပြင်နည်းများ', pages: ['basics/layers', 'basics/colors', 'basics/selections', 'basics/masks', 'basics/previews'] },
  { label: 'Image and canvas', my: 'ပုံနှင့် Canvas', pages: ['image/crop', 'image/resize', 'image/import-layers', 'image/pdf'] },
  { label: 'Common tasks', my: 'အသုံးများသော လုပ်ဆောင်ချက်များ', pages: ['guides/color-corrections', 'guides/remove-background', 'guides/combine-images', 'guides/paint-retouch', 'guides/fills-gradients', 'guides/text-effects', 'guides/paths'] },
  { label: 'Tools and effects', my: 'Tools နှင့် Effects', pages: ['reference/selection', 'reference/painting', 'reference/transforms', 'reference/effects'] },
  { label: 'Files and shortcuts', my: 'Files နှင့် Shortcuts', pages: ['reference/files', 'reference/shortcuts'] },
  { label: 'Fix a problem', my: 'ပြဿနာအဖြေရှာရန်', pages: ['troubleshooting/painting', 'troubleshooting/moving', 'troubleshooting/files-text-recovery'] },
];
export const pageKeys = ['index', ...navigation.flatMap(({ pages }) => pages)];
