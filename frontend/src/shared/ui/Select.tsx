import { type ComponentProps, forwardRef } from "react";
import clsx from "clsx";

type SelectSize = "xs" | "sm";

const sizeClass: Record<SelectSize, string> = {
  xs: "h-6 px-2 text-2xs",
  sm: "h-7 px-2.5 text-xs",
};

interface SelectProps extends Omit<ComponentProps<"select">, "size"> {
  size?: SelectSize;
}

export const Select = forwardRef<HTMLSelectElement, SelectProps>(
  ({ size = "xs", className, ...props }, ref) => (
    <select
      ref={ref}
      className={clsx(
        "rounded border border-gray-200 dark:border-gray-600",
        "bg-gray-50 dark:bg-gray-700 text-gray-900 dark:text-gray-100",
        "focus:outline-none focus:ring-1 focus:ring-blue-400",
        sizeClass[size],
        className,
      )}
      {...props}
    />
  ),
);

Select.displayName = "Select";
