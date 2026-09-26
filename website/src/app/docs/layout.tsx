import { getSidebarTree, getAllDocs } from '@/lib/docs';
import DocsLayoutClient from '@/components/docs-layout-client';

export default function DocsLayout({ children }: { children: React.ReactNode }) {
  const tree = getSidebarTree();
  const allDocs = getAllDocs();

  const sidebarItems: { title: string; href: string; depth: number }[] = [];

  function traverse(nodes: typeof tree, depth: number) {
    for (const node of nodes) {
      if (node.slug) {
        sidebarItems.push({
          title: node.title,
          href: `/docs/${node.slug}`,
          depth,
        });
      }
      if (node.children.length > 0) {
        traverse(node.children, depth + 1);
      }
    }
  }

  traverse(tree, 0);

  return (
    <DocsLayoutClient sidebarItems={sidebarItems} allDocs={allDocs}>
      {children}
    </DocsLayoutClient>
  );
}
