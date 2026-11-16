// Explicit IDs keep deep links identical when a heading is translated or renamed.
export function explicitHeadingIds() {
  return (tree) => {
    function visit(node) {
      if (node.type === 'heading') {
        const tail = node.children?.at(-1);
        const match = tail?.type === 'text' && tail.value.match(/\s*\{#([a-z][a-z0-9-]*)\}\s*$/);
        if (match) {
          tail.value = tail.value.slice(0, match.index);
          node.data ??= {};
          node.data.hProperties = { ...node.data.hProperties, id: match[1] };
        }
      }
      node.children?.forEach(visit);
    }
    visit(tree);
  };
}
