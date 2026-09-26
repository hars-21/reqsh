import { NextRequest, NextResponse } from 'next/server';
import { getDocBySlug } from '@/lib/docs';

export async function GET(
  _request: NextRequest,
  { params }: { params: Promise<{ slug: string }> }
) {
  const { slug } = await params;
  const doc = getDocBySlug(slug);

  if (!doc) {
    return NextResponse.json({ error: 'Doc not found' }, { status: 404 });
  }

  return NextResponse.json({
    markdown: doc.content,
    title: doc.title,
    slug: doc.slug,
  });
}
