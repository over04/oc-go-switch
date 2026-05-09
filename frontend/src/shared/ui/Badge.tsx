import { type ComponentProps } from "react";
import clsx from "clsx";

type BadgeSize = "xs" | "sm";
const sizeClass: Record<BadgeSize, string> = {
  xs: "text-[0.6875rem] px-2.5 py-0.5 rounded-full",
  sm: "text-xs px-3 py-1 rounded-full",
};

type Tone = "go" | "default" | "success" | "danger" | "warning";
const toneClass: Record<Tone, string> = {
  go: "bg-harvest-300/20 text-harvest-500",
  default: "bg-cream-100 text-espresso-500",
  success: "bg-harvest-500/10 text-harvest-500",
  danger: "bg-terra-400/10 text-terra-400",
  warning: "bg-harvest-300/20 text-amber-700",
};

interface BadgeProps extends ComponentProps<"span"> {
  size?: BadgeSize;
  tone?: Tone;
}

export function Badge({
  size = "xs",
  tone = "default",
  className,
  ...props
}: BadgeProps) {
  return (
    <span
      className={clsx(
        "inline-flex items-center font-medium tracking-wide",
        sizeClass[size],
        toneClass[tone],
        className,
      )}
      {...props}
    />
  );
}
