import { useEffect, useRef, type ComponentProps } from "react";
import { motion, AnimatePresence } from "framer-motion";
import clsx from "clsx";
import { Button } from "./Button";

interface DialogProps extends ComponentProps<"dialog"> {
  open: boolean;
  title: string;
  onClose: () => void;
}

export function Dialog({
  open,
  title,
  onClose,
  children,
  className,
  ...props
}: DialogProps) {
  const ref = useRef<HTMLDialogElement>(null);

  useEffect(() => {
    const el = ref.current;
    if (!el) return;
    if (open && !el.open) el.showModal();
    else if (!open && el.open) el.close();
  }, [open]);

  return (
    <dialog
      ref={ref}
      onClose={onClose}
      className={clsx(
        "backdrop:bg-black/50 rounded-lg border border-gray-200 dark:border-gray-700",
        "bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100",
        "p-0 shadow-xl max-w-sm w-full",
        className,
      )}
      {...props}
    >
      <AnimatePresence>
        {open && (
          <motion.div
            initial={{ opacity: 0, scale: 0.95, y: 8 }}
            animate={{ opacity: 1, scale: 1, y: 0 }}
            exit={{ opacity: 0, scale: 0.95, y: 8 }}
            transition={{ type: "spring", stiffness: 500, damping: 35 }}
          >
            <div className="flex items-center justify-between px-4 py-3 border-b border-gray-100 dark:border-gray-700">
              <h2 className="text-xs font-semibold">{title}</h2>
              <Button size="xs" tone="default" onClick={onClose} className="!text-2xs">
                ✕
              </Button>
            </div>
            <div className="px-4 py-3">{children}</div>
          </motion.div>
        )}
      </AnimatePresence>
    </dialog>
  );
}
