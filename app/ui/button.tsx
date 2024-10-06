import React from "react";

type ButtonProps = {
  children: React.ReactNode;
};

export function Button({ children }: ButtonProps) {
  return (
    <button className="select-none overflow-hidden rounded-md px-2 py-1 text-xs font-medium transition-colors duration-200 ease-out bg-white/5 text-[#C8C8C8] hover:bg-white/10 hover:text-white">
      <span>{children}</span>
    </button>
  );
}
