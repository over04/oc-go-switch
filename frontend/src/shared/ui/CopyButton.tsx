import { useState, useCallback, type ComponentProps } from "react";
import { motion } from "framer-motion";
import clsx from "clsx";

interface CopyButtonProps extends ComponentProps<typeof motion.button> {
  value: string;
  size?: "xs" | "sm";
}

export function CopyButton({ value, size = "xs", className, ...props }: CopyButtonProps) {
  const [copied, setCopied] = useState(false);
  const handleCopy = useCallback(async () => {
    try { await navigator.clipboard.writeText(value); setCopied(true); setTimeout(() => setCopied(false), 1200); } catch { /* noop */ }
  }, [value]);

  return (
    <motion.button
      whileTap={{ scale: 0.9 }}
      onClick={handleCopy}
      className={clsx(
        "inline-flex items-center justify-center rounded font-medium transition-colors",
        "bg-gray-100 dark:bg-gray-700 text-gray-600 dark:text-gray-300 hover:bg-gray-200 dark:hover:bg-gray-600",
        "focus:outline-none focus-visible:ring-2 focus-visible:ring-blue-400",
        size === "xs" ? "h-6 px-2 text-xs" : "h-7 px-2.5 text-sm",
        copied && "!bg-green-500 !text-white",
        className,
      )}
      {...props}
    >
      {copied ? "✓" : "复制"}
    </motion.button>
  );
}
