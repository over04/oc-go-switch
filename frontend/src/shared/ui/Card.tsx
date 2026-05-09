import type { ComponentProps } from "react";
import clsx from "clsx";

interface CardProps extends ComponentProps<"div"> {
  size?: "xs" | "sm";
}

export function Card({ size = "xs", className, ...props }: CardProps) {
  return (
    <div
      className={clsx(
        "bg-white rounded-mcm-lg border border-cream-200/80 shadow-mcm",
        size === "xs" ? "px-4 py-3" : "px-5 py-4",
        className,
      )}
      {...props}
    />
  );
}
