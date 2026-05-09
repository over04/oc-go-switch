import { motion } from "framer-motion";
import clsx from "clsx";

interface StatCardProps {
  label: string;
  value: number | string;
  tone?: "default" | "success" | "danger" | "info";
  delay?: number;
}

const toneConfig: Record<
  NonNullable<StatCardProps["tone"]>,
  { text: string; bg: string; border: string; dot: string }
> = {
  default: {
    text: "text-espresso-700",
    bg: "bg-white",
    border: "border-cream-200",
    dot: "bg-espresso-400",
  },
  success: {
    text: "text-harvest-500",
    bg: "bg-white",
    border: "border-harvest-500/15",
    dot: "bg-harvest-500",
  },
  danger: {
    text: "text-terra-400",
    bg: "bg-white",
    border: "border-terra-400/15",
    dot: "bg-terra-400",
  },
  info: {
    text: "text-sky-400",
    bg: "bg-white",
    border: "border-sky-400/15",
    dot: "bg-sky-400",
  },
};

export function StatCard({
  label,
  value,
  tone = "default",
  delay = 0,
}: StatCardProps) {
  const c = toneConfig[tone];

  return (
    <motion.div
      initial={{ opacity: 0, y: 8 }}
      animate={{ opacity: 1, y: 0 }}
      transition={{ delay, duration: 0.35, ease: "easeOut" }}
      whileHover={{ y: -2 }}
      className={clsx(
        "rounded-mcm-xl border shadow-mcm p-4 transition-shadow duration-200 hover:shadow-mcm-md",
        c.bg,
        c.border,
      )}
    >
      <div className="flex items-center gap-2 mb-2">
        <span className={clsx("w-1.5 h-1.5 rounded-full", c.dot)} />
        <span className="text-xs text-espresso-400 uppercase tracking-wider font-medium">
          {label}
        </span>
      </div>
      <motion.p
        className={clsx(
          "text-[2rem] font-bold tabular-nums tracking-tight leading-none",
          c.text,
        )}
        initial={{ opacity: 0, scale: 0.8 }}
        animate={{ opacity: 1, scale: 1 }}
        transition={{ delay: delay + 0.1, duration: 0.3 }}
      >
        {value}
      </motion.p>
    </motion.div>
  );
}
