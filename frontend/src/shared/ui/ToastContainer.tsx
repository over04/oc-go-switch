import { useSyncExternalStore } from "react";
import { motion, AnimatePresence } from "framer-motion";
import clsx from "clsx";
import { getToasts, subscribe } from "@/shared/lib/toast";

const toneClass = {
  success: "bg-green-600 text-white",
  error: "bg-red-600 text-white",
  info: "bg-gray-800 text-white dark:bg-gray-600",
};

export function ToastContainer() {
  const toasts = useSyncExternalStore(subscribe, getToasts, getToasts);

  return (
    <div className="fixed bottom-4 right-4 z-50 flex flex-col gap-1.5 pointer-events-none">
      <AnimatePresence>
        {toasts.map((t) => (
          <motion.div
            key={t.id}
            initial={{ opacity: 0, x: 40, scale: 0.9 }}
            animate={{ opacity: 1, x: 0, scale: 1 }}
            exit={{ opacity: 0, x: 40, scale: 0.9 }}
            transition={{ type: "spring", stiffness: 500, damping: 35 }}
            className={clsx(
              "px-3 py-1.5 rounded-lg text-xs font-medium shadow-lg pointer-events-auto",
              toneClass[t.type],
            )}
          >
            {t.message}
          </motion.div>
        ))}
      </AnimatePresence>
    </div>
  );
}
