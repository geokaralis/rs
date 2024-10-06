import React from "react";
import "globals.css";
import { Main } from "./main";
import { Noise } from "./ui/noise";

export default function Index() {
  return (
    <html lang="en">
      <head>
        <meta charSet="utf-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <link
          rel="preload"
          href="inter.woff2"
          as="font"
          crossOrigin=""
          type="font/woff2"
        />
        <title>React static components</title>
        <link href="./output.css" rel="stylesheet" />
      </head>
      <body className="bg-[#0f0f0f] antialiased">
        <Main />
        <Noise />
      </body>
    </html>
  );
}
