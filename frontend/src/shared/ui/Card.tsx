import type { ComponentProps } from "react";
import clsx from "clsx";

interface CardProps extends ComponentProps<"div"> {
  size?: "xs" | "sm";
}

export function Card({ size = "xs", className, ...props }: CardProps) {
  return <div className={clsx(
    "bg-white dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700 shadow-sm",
    size === "xs" ? "px-4 py-3" : "px-5 py-4",
    className,
  )} {...props} />;
}
