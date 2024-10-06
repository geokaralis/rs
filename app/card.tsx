import React from "react";
import { hash } from "./lib/hash";
import { getUser } from "./lib/github";
import { Button } from "./ui/button";

export async function Card() {
  const sh = hash();
  const user = await getUser("geokaralis");

  return (
    <div className="flex flex-col justify-end overflow-hidden rounded-[14px] border-white/80 p-2 pl-3 shadow-tooltip bg-[#161716] shadow-[0px_1px_0px_0px_hsla(0,0%,100%,0.03)_inset,0px_0px_0px_1px_hsla(0,0%,100%,0.03)_inset,0px_0px_0px_1px_rgba(0,0,0,0.1),0px_2px_2px_0px_rgba(0,0,0,0.1),0px_4px_4px_0px_rgba(0,0,0,0.1),0px_8px_8px_0px_rgba(0,0,0,0.1)]">
      <div className="flex flex-col justify-between gap-3">
        <div className="flex flex-col gap-3">
          <div className="flex items-center justify-between">
            <div className="text-[13px] font-medium text-white">
              React static components
            </div>
            <div className="text-[13px] text-white/70">{user.login}</div>
          </div>
          <div className="select-none rounded-lg bg-white/5 px-3 py-1 text-[13px] text-white/80">
            Start by editing the{" "}
            <span className="select-none text-white">index.tsx</span> file
            inside the app directory.
          </div>
        </div>
        <div className="flex items-center justify-between gap-1 group">
          <div className="flex items-center w-auto gap-2 overflow-hidden">
            <div className="flex items-center gap-2">
              <div className="h-4 w-4 text-white">
                <svg
                  width="16"
                  height="16"
                  viewBox="0 0 16 16"
                  fill="none"
                  xmlns="http://www.w3.org/2000/svg"
                >
                  <path
                    fill-rule="evenodd"
                    clip-rule="evenodd"
                    d="M8 10.5a2.5 2.5 0 100-5 2.5 2.5 0 000 5zM8 12c1.953 0 3.579-1.4 3.93-3.25H16v-1.5h-4.07a4.001 4.001 0 00-7.86 0H0v1.5h4.07A4.001 4.001 0 008 12z"
                    fill="currentColor"
                  ></path>
                </svg>
              </div>
              <div className="w-auto select-none text-[13px] font-medium transition-colors duration-200 ease-out group-hover:text-white text-white">
                {sh}
              </div>
            </div>
            <div className="flex w-auto select-none items-center gap-2 overflow-hidden text-nowrap text-[13px] font-medium text-white">
              <div className="font-medium text-white/30">·</div>
              <div>Build</div>
            </div>
          </div>
          <Button>Docs</Button>
        </div>
      </div>
    </div>
  );
}
