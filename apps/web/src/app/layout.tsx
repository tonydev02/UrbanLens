import type { Metadata } from "next";
import type { ReactNode } from "react";

import { AppShell } from "./app-shell";
import "./globals.css";
import { Providers } from "./providers";

export const metadata: Metadata = {
  title: "UrbanLens",
  description: "Tokyo commercial real-estate intelligence from official public data.",
};

export default function RootLayout({ children }: Readonly<{ children: ReactNode }>) {
  return (
    <html lang="en">
      <body>
        <Providers>
          <AppShell>{children}</AppShell>
        </Providers>
      </body>
    </html>
  );
}
