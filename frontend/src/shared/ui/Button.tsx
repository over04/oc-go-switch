import { type ComponentProps, forwardRef } from "react";
import { motion } from "framer-motion";
import clsx from "clsx";

type BtnSize = "xs" | "sm";
const sizeClass: Record<BtnSize, string> = {
  xs: "h-7 px-4 text-xs",
  sm: "h-8 px-5 text-sm",
};

type Tone = "default" | "primary" | "danger";
const toneClass: Record<Tone, string> = {
  default:
    "bg-cream-100 text-espresso-700 hover:bg-cream-200 border border-cream-300",
  primary:
    "bg-terra-500 text-white hover:bg-terra-600 shadow-sm",
  danger:
    "bg-terra-400 text-white hover:opacity-90 shadow-sm",
};

interface ButtonProps extends ComponentProps<typeof motion.button> {
  size?: BtnSize;
  tone?: Tone;
}

export const Button = forwardRef<HTMLButtonElement, ButtonProps>(
  ({ size = "xs", tone = "default", className, ...props }, ref) => (
    <motion.button
      ref={ref}
      whileHover={{ scale: 1.02 }}
      whileTap={{ scale: 0.96 }}
      className={clsx(
        "inline-flex items-center justify-center rounded-full font-medium transition-all duration-200",
        "focus:outline-none focus-visible:ring-2 focus-visible:ring-terra-500/40",
        "disabled:opacity-40 disabled:pointer-events-none",
        "tracking-wide",
        sizeClass[size],
        toneClass[tone],
        className,
      )}
      {...props}
    />
  ),
);
Button.displayName = "Button";
