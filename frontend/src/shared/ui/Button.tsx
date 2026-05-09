import { type ComponentProps, forwardRef } from "react";
import { motion } from "framer-motion";
import clsx from "clsx";

type BtnSize = "xs" | "sm";
const sizeClass: Record<BtnSize, string> = { xs: "h-7 px-3 text-xs", sm: "h-8 px-4 text-sm" };

type Tone = "default" | "primary" | "danger";
const toneClass: Record<Tone, string> = {
  default: "bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-200 hover:bg-gray-200 dark:hover:bg-gray-600",
  primary: "bg-blue-500 text-white hover:bg-blue-600",
  danger: "bg-red-500 text-white hover:bg-red-600",
};

interface ButtonProps extends ComponentProps<typeof motion.button> {
  size?: BtnSize;
  tone?: Tone;
}

export const Button = forwardRef<HTMLButtonElement, ButtonProps>(
  ({ size = "xs", tone = "default", className, ...props }, ref) => (
    <motion.button
      ref={ref}
      whileTap={{ scale: 0.95 }}
      className={clsx(
        "inline-flex items-center justify-center rounded font-medium transition-colors",
        "focus:outline-none focus-visible:ring-2 focus-visible:ring-blue-400",
        "disabled:opacity-40 disabled:pointer-events-none",
        sizeClass[size], toneClass[tone], className,
      )}
      {...props}
    />
  ),
);
Button.displayName = "Button";
