import { forwardRef } from "react";
import type { ComponentProps } from "react";
import clsx from "clsx";

type InputSize = "xs" | "sm";
const sizeClass: Record<InputSize, string> = { xs: "h-7 px-2.5 text-xs", sm: "h-8 px-3 text-sm" };

interface InputProps extends Omit<ComponentProps<"input">, "size"> {
  size?: InputSize;
  error?: string;
}

export const Input = forwardRef<HTMLInputElement, InputProps>(
  ({ size = "xs", error, className, ...props }, ref) => (
    <input ref={ref} className={clsx(
      "w-full rounded border bg-gray-50 dark:bg-gray-700 text-gray-900 dark:text-gray-100",
      "focus:outline-none focus:ring-2 focus:ring-blue-400",
      "placeholder:text-gray-400",
      error ? "border-red-400" : "border-gray-200 dark:border-gray-600",
      sizeClass[size], className,
    )} {...props} />
  ),
);
Input.displayName = "Input";
