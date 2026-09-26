import { NextResponse } from 'next/server';

export const revalidate = 3600;

export async function GET() {
  const res = await fetch(
    'https://github.com/hars-21/reqsh/releases/latest/download/reqsh-installer.sh',
    {
      next: { revalidate: 3600 },
    }
  );

  if (!res.ok) {
    return new NextResponse('Failed to fetch install script', {
      status: 502,
    });
  }

  const content = await res.text();

  return new NextResponse(content, {
    headers: {
      'Content-Type': 'text/plain; charset=utf-8',
      'Cache-Control': 'public, max-age=3600, s-maxage=3600',
    },
  });
}
