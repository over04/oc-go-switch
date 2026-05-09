import { motion } from "framer-motion";
import clsx from "clsx";

interface StatCardProps {
  label: string;
  value: number | string;
  tone?: "default" | "success" | "danger" | "info";
  delay?: number;
}

const toneClass: Record<string, string> = {
  default: "text-gray-900 dark:text-gray-100",
  success: "text-green-600 dark:text-green-400",
  danger: "text-red-500",
  info: "text-blue-600 dark:text-blue-400",
};

const bgClass: Record<string, string> = {
  default: "",
  success: "bg-green-50 dark:bg-green-900/20 border-green-200 dark:border-green-800",
  danger: "bg-red-50 dark:bg-red-900/20 border-red-200 dark:border-red-800",
  info: "bg-blue-50 dark:bg-blue-900/20 border-blue-200 dark:border-blue-800",
};

export function StatCard({ label, value, tone = "default", delay = 0 }: StatCardProps) {
  return (
    <motion.div
      initial={{ opacity: 0, y: 8, scale: 0.95 }}
      animate={{ opacity: 1, y: 0, scale: 1 }}
      transition={{ delay, duration: 0.3, ease: "easeOut" }}
      whileHover={{ y: -2, transition: { duration: 0.15 } }}
      className={clsx(
        "bg-white dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700 px-3 py-2.5 shadow-sm",
        bgClass[tone],
      )}
    >
      <span className="text-2xs text-gray-400 dark:text-gray-500">{label}</span>
      <motion.p
        className={clsx("text-sm font-semibold tabular-nums mt-0.5", toneClass[tone])}
        initial={{ opacity: 0 }}
        animate={{ opacity: 1 }}
        transition={{ delay: delay + 0.15, duration: 0.3 }}
      >
        {value}
      </motion.p>
    </motion.div>
  );
}
