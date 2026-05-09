import { motion, AnimatePresence } from "framer-motion";
import { Button } from "@/shared/ui/Button";

interface ActiveKeyBarProps {
  activeKeyId: string | null;
  onClear: () => void;
}

export function ActiveKeyBar({ activeKeyId, onClear }: ActiveKeyBarProps) {
  return (
    <AnimatePresence>
      {activeKeyId && (
        <motion.div
          initial={{ opacity: 0, height: 0 }}
          animate={{ opacity: 1, height: "auto" }}
          exit={{ opacity: 0, height: 0 }}
          className="overflow-hidden"
        >
          <div className="flex flex-wrap items-center gap-2 sm:gap-3 bg-terra-500/5 border border-terra-500/15 rounded-mcm-lg px-3 md:px-4 py-2.5">
            {/* Pulse dot */}
            <motion.span
              className="relative flex h-2.5 w-2.5"
              animate={{ opacity: [0.5, 1, 0.5] }}
              transition={{ repeat: Infinity, duration: 1.5 }}
            >
              <motion.span
                className="animate-ping absolute inline-flex h-full w-full rounded-full bg-terra-500 opacity-30"
                animate={{ scale: [1, 1.8, 1] }}
                transition={{ repeat: Infinity, duration: 1.5 }}
              />
              <span className="relative inline-flex rounded-full h-2.5 w-2.5 bg-terra-500" />
            </motion.span>

            <span className="text-xs text-espresso-500 font-medium">
              当前活跃密钥
            </span>
            <code className="text-sm font-mono font-semibold text-terra-500 bg-terra-500/5 px-2 py-0.5 rounded-full">
              {activeKeyId.split("/").pop() ?? activeKeyId}
            </code>

            <Button size="xs" tone="default" onClick={onClear}>
              清除
            </Button>
          </div>
        </motion.div>
      )}
    </AnimatePresence>
  );
}
