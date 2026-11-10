import type { Metadata } from "next";
import "./globals.css";

export const metadata: Metadata = {
  title: "ARSY Console",
  description: "ARSY hybrid cloud control plane console.",
};

export default function RootLayout({ children }: Readonly<{ children: React.ReactNode }>) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  );
}
