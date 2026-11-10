import type { Metadata } from "next";
import "./globals.css";
export const metadata: Metadata = { title: "ARSY — Small Core. Global Infrastructure.", description: "An independent hybrid cloud infrastructure platform." };
export default function RootLayout({ children }: Readonly<{ children: React.ReactNode }>) {
  return <html lang="en"><body>{children}</body></html>;
}
