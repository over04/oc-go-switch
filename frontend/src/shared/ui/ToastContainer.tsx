import { useSyncExternalStore } from "react";
import { motion, AnimatePresence } from "framer-motion";
import clsx from "clsx";
import { getToasts, subscribe } from "@/shared/lib/toast";

const toneClass = {
  success: "bg-harvest-500 text-white",
  error: "bg-terra-400 text-white",
  info: "bg-espresso-700 text-cream-100",
};

export function ToastContainer() {
  const toasts = useSyncExternalStore(subscribe, getToasts, getToasts);

  return (
    <div className="fixed bottom-4 right-4 z-50 flex flex-col gap-2 pointer-events-none">
      <AnimatePresence>
        {toasts.map((t) => (
          <motion.div
            key={t.id}
            initial={{ opacity: 0, x: 40, scale: 0.9 }}
            animate={{ opacity: 1, x: 0, scale: 1 }}
            exit={{ opacity: 0, x: 40, scale: 0.9 }}
            transition={{ type: "spring", stiffness: 400, damping: 30 }}
            className={clsx(
              "px-4 py-2 rounded-xl text-xs font-medium shadow-mcm-md pointer-events-auto",
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
