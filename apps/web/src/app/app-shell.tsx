"use client";

import Link from "next/link";
import { usePathname } from "next/navigation";
import type { ReactNode } from "react";

export function AppShell({ children }: Readonly<{ children: ReactNode }>) {
  const pathname = usePathname();
  const marketMapActive = pathname === "/" || pathname.startsWith("/market-map");

  return (
    <div className="app-shell">
      <header className="top-bar">
        <Link className="brand" href="/market-map" aria-label="UrbanLens market map">
          <span className="brand-mark" aria-hidden="true">
            UL
          </span>
          <span>
            <span className="brand-name">UrbanLens</span>
            <span className="brand-context">Tokyo public-data CRE intelligence</span>
          </span>
        </Link>
        <nav aria-label="Primary navigation">
          <Link className="nav-link" aria-current={marketMapActive ? "page" : undefined} href="/market-map">
            Market Map
          </Link>
        </nav>
      </header>
      <main id="main-content" className="workspace">
        {children}
      </main>
    </div>
  );
}
