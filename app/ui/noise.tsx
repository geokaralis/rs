import React from "react";

export function Noise() {
  return (
    <svg className="pointer-events-none fixed inset-0 bottom-0 left-0 right-0 top-0 -z-50 min-h-full min-w-full overflow-x-hidden overflow-y-hidden bg-white/10 opacity-5">
      <filter id="noise">
        <feTurbulence
          type="fractalNoise"
          baseFrequency=".85"
          numOctaves="4"
          stitchTiles="stitch"
        ></feTurbulence>
        <feColorMatrix type="saturate" values="0"></feColorMatrix>
      </filter>
      <rect width="100%" height="100%" filter="url(#noise)"></rect>
    </svg>
  );
}
