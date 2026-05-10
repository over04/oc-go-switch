import { useState, useRef, useEffect } from "react";
import { motion, AnimatePresence } from "framer-motion";
import clsx from "clsx";

type SelectSize = "xs" | "sm";

const sizeClass: Record<SelectSize, string> = {
  xs: "h-7 px-2.5 text-xs",
  sm: "h-8 px-3 text-sm",
};

interface SelectOption {
  value: string;
  label: string;
}

interface SelectProps {
  size?: SelectSize;
  value: string;
  onChange: (value: string) => void;
  options: SelectOption[];
  placeholder?: string;
  className?: string;
  disabled?: boolean;
}

export function Select({
  size = "xs",
  value,
  onChange,
  options,
  placeholder = "选择...",
  className,
  disabled,
}: SelectProps) {
  const [open, setOpen] = useState(false);
  const [focusIdx, setFocusIdx] = useState(-1);
  const ref = useRef<HTMLDivElement>(null);
  const listRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (!open) {
      setFocusIdx(-1);
      return;
    }
    const handler = (e: MouseEvent) => {
      if (ref.current && !ref.current.contains(e.target as Node)) setOpen(false);
    };
    document.addEventListener("mousedown", handler);
    return () => document.removeEventListener("mousedown", handler);
  }, [open]);

  useEffect(() => {
    if (!open || focusIdx < 0 || !listRef.current) return;
    const el = listRef.current.children[focusIdx] as HTMLElement | undefined;
    el?.scrollIntoView({ block: "nearest" });
  }, [open, focusIdx]);

  function handleKey(e: React.KeyboardEvent) {
    switch (e.key) {
      case "ArrowDown":
        e.preventDefault();
        if (!open) setOpen(true);
        setFocusIdx((i) => Math.min(i + 1, options.length - 1));
        break;
      case "ArrowUp":
        e.preventDefault();
        setFocusIdx((i) => Math.max(i - 1, 0));
        break;
      case "Enter":
        e.preventDefault();
        if (open && focusIdx >= 0 && focusIdx < options.length) {
          onChange(options[focusIdx]!.value);
          setOpen(false);
        } else {
          setOpen(!open);
        }
        break;
      case "Escape":
        setOpen(false);
        break;
    }
  }

  const selected = options.find((o) => o.value === value);

  return (
    <div ref={ref} className={clsx("relative", className)} onKeyDown={handleKey}>
      <button
        type="button"
        disabled={disabled}
        onClick={() => !disabled && setOpen(!open)}
        className={clsx(
          "w-full rounded-[10px] border bg-white text-espresso-700 text-left",
          "transition-all duration-200 flex items-center justify-between gap-2",
          "focus:outline-none focus:ring-[3px] focus:ring-terra-500/15 focus:border-terra-500",
          disabled ? "opacity-50 cursor-not-allowed" : "cursor-pointer",
          "border-cream-300",
          sizeClass[size],
        )}
      >
        <span className={selected ? "text-espresso-700" : "text-espresso-300"}>
          {selected?.label ?? placeholder}
        </span>
        <motion.svg
          animate={{ rotate: open ? 180 : 0 }}
          transition={{ duration: 0.2 }}
          className="w-3 h-3 text-espresso-400 shrink-0"
          viewBox="0 0 10 6"
        >
          <path
            d="M1 1l4 4 4-4"
            stroke="currentColor"
            strokeWidth="1.5"
            fill="none"
          />
        </motion.svg>
      </button>

      <AnimatePresence>
        {open && (
          <motion.div
            ref={listRef}
            initial={{ opacity: 0, y: -6, scale: 0.95 }}
            animate={{ opacity: 1, y: 0, scale: 1 }}
            exit={{ opacity: 0, y: -6, scale: 0.95 }}
            transition={{ duration: 0.18, ease: "easeOut" }}
            className="absolute z-50 mt-1.5 w-full bg-white rounded-[10px] border border-cream-200 shadow-mcm-md overflow-hidden max-h-56 overflow-y-auto py-1"
          >
            {options.map((opt, i) => (
              <button
                key={opt.value}
                type="button"
                onClick={() => {
                  onChange(opt.value);
                  setOpen(false);
                }}
                onMouseEnter={() => setFocusIdx(i)}
                className={clsx(
                  "w-full text-left px-3 py-1.5 text-xs transition-colors",
                  opt.value === value
                    ? "bg-terra-500/10 text-terra-600 font-medium"
                    : focusIdx === i
                      ? "bg-cream-100 text-espresso-700"
                      : "text-espresso-600 hover:bg-cream-50",
                )}
              >
                {opt.label}
              </button>
            ))}
          </motion.div>
        )}
      </AnimatePresence>
    </div>
  );
}
