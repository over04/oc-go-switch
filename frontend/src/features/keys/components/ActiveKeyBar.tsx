import { motion } from "framer-motion";
import { Button } from "@/shared/ui/Button";

interface ActiveKeyBarProps {
  activeKeyId: string | null;
  onClear: () => void;
}

export function ActiveKeyBar({ activeKeyId, onClear }: ActiveKeyBarProps) {
  if (!activeKeyId) return null;

  const shortId = activeKeyId.split("/").pop() ?? activeKeyId;

  return (
    <motion.div
      initial={{ opacity: 0, height: 0 }}
      animate={{ opacity: 1, height: "auto" }}
      className="mb-3 overflow-hidden"
    >
      <div className="flex items-center gap-2 bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded px-3 py-1.5">
        <motion.span
          className="w-1.5 h-1.5 rounded-full bg-blue-500 inline-block"
          animate={{ scale: [1, 1.4, 1] }}
          transition={{ repeat: Infinity, duration: 1.5 }}
        />
        <span className="text-2xs text-blue-700 dark:text-blue-300">
          当前活跃密钥:
        </span>
        <code className="text-2xs font-mono text-blue-800 dark:text-blue-200 font-medium">
          {shortId}
        </code>
        <Button size="xs" tone="default" onClick={onClear}>
          清除
        </Button>
      </div>
    </motion.div>
  );
}
