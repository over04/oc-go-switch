import { type ComponentProps } from "react";
import clsx from "clsx";

type BadgeSize = "xs" | "sm";
const sizeClass: Record<BadgeSize, string> = { xs: "text-xs px-2 py-0.5 rounded", sm: "text-sm px-2.5 py-1 rounded" };

type Tone = "go" | "default" | "success" | "danger" | "warning";
const toneClass: Record<Tone, string> = {
  go: "bg-green-100 dark:bg-green-900 text-green-700 dark:text-green-300",
  default: "bg-gray-100 dark:bg-gray-700 text-gray-500 dark:text-gray-400",
  success: "bg-green-100 dark:bg-green-900 text-green-700 dark:text-green-300",
  danger: "bg-red-100 dark:bg-red-900 text-red-600 dark:text-red-400",
  warning: "bg-amber-100 dark:bg-amber-900 text-amber-700 dark:text-amber-300",
};

interface BadgeProps extends ComponentProps<"span"> {
  size?: BadgeSize;
  tone?: Tone;
}

export function Badge({ size = "xs", tone = "default", className, ...props }: BadgeProps) {
  return <span className={clsx("inline-flex items-center font-medium", sizeClass[size], toneClass[tone], className)} {...props} />;
}
