import React from "react";
import { Card } from "./card";

export function Main() {
  return (
    <main className="flex items-center justify-center h-screen">
      {/* @ts-expect-error Async Server Component */}
      <Card />
    </main>
  );
}
