import { type ComponentProps, forwardRef } from "react";
import clsx from "clsx";

type SelectSize = "xs" | "sm";

const sizeClass: Record<SelectSize, string> = {
  xs: "h-6 px-2.5 text-xs",
  sm: "h-7 px-3 text-sm",
};

interface SelectProps extends Omit<ComponentProps<"select">, "size"> {
  size?: SelectSize;
}

export const Select = forwardRef<HTMLSelectElement, SelectProps>(
  ({ size = "xs", className, ...props }, ref) => (
    <select
      ref={ref}
      className={clsx(
        "rounded-[10px] border border-cream-300 bg-white text-espresso-700",
        "transition-all duration-200",
        "focus:outline-none focus:ring-[3px] focus:ring-terra-500/15 focus:border-terra-500",
        sizeClass[size],
        className,
      )}
      {...props}
    />
  ),
);

Select.displayName = "Select";
