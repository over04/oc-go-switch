import { useState, useCallback, type ComponentProps } from "react";
import { motion } from "framer-motion";
import clsx from "clsx";

interface CopyButtonProps extends ComponentProps<typeof motion.button> {
  value: string;
  size?: "xs" | "sm";
}

export function CopyButton({
  value,
  size = "xs",
  className,
  ...props
}: CopyButtonProps) {
  const [copied, setCopied] = useState(false);
  const handleCopy = useCallback(async () => {
    try {
      await navigator.clipboard.writeText(value);
      setCopied(true);
      setTimeout(() => setCopied(false), 1200);
    } catch {
      /* noop */
    }
  }, [value]);

  return (
    <motion.button
      whileTap={{ scale: 0.9 }}
      onClick={handleCopy}
      className={clsx(
        "inline-flex items-center justify-center rounded-full font-medium transition-all duration-200",
        "bg-cream-100 text-espresso-500 hover:bg-cream-200",
        "focus:outline-none focus-visible:ring-2 focus-visible:ring-terra-500/40",
        size === "xs" ? "h-6 px-2.5 text-[0.6875rem]" : "h-7 px-3 text-xs",
        copied && "!bg-harvest-500 !text-white",
        className,
      )}
      {...props}
    >
      {copied ? "✓" : "复制"}
    </motion.button>
  );
}
