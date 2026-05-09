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
        "backdrop:bg-espresso-900/30 rounded-mcm-lg border border-cream-200",
        "bg-white text-espresso-700",
        "p-0 shadow-mcm-lg max-w-sm w-full",
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
            transition={{ type: "spring", stiffness: 400, damping: 30 }}
          >
            <div className="flex items-center justify-between px-5 py-3 border-b border-cream-100">
              <h2 className="text-sm font-semibold tracking-tight">{title}</h2>
              <Button size="xs" tone="default" onClick={onClose}>
                &times;
              </Button>
            </div>
            <div className="px-5 py-4">{children}</div>
          </motion.div>
        )}
      </AnimatePresence>
    </dialog>
  );
}
