import { forwardRef } from "react";
import type { ComponentProps } from "react";
import clsx from "clsx";

type InputSize = "xs" | "sm";
const sizeClass: Record<InputSize, string> = {
  xs: "h-7 px-3 text-xs",
  sm: "h-8 px-3.5 text-sm",
};

interface InputProps extends Omit<ComponentProps<"input">, "size"> {
  size?: InputSize;
  error?: string;
}

export const Input = forwardRef<HTMLInputElement, InputProps>(
  ({ size = "xs", error, className, ...props }, ref) => (
    <input
      ref={ref}
      className={clsx(
        "w-full rounded-[10px] border bg-white text-espresso-700",
        "transition-all duration-200",
        "focus:outline-none focus:ring-[3px] focus:ring-terra-500/15 focus:border-terra-500",
        "placeholder:text-espresso-300",
        error
          ? "border-terra-400 ring-terra-400/15"
          : "border-cream-300",
        sizeClass[size],
        className,
      )}
      {...props}
    />
  ),
);
Input.displayName = "Input";
