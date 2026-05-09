import clsx from "clsx";
import { motion } from "framer-motion";

interface UsageBarProps {
  label: string;
  percent: number;
  resetSec: number;
}

function formatReset(sec: number): string {
  if (sec < 60) return `${sec}s`;
  if (sec < 3600) return `${Math.floor(sec / 60)}m`;
  if (sec < 86400)
    return `${Math.floor(sec / 3600)}h ${Math.floor((sec % 3600) / 60)}m`;
  return `${Math.floor(sec / 86400)}d ${Math.floor((sec % 86400) / 3600)}h`;
}

export function UsageBar({ label, percent, resetSec }: UsageBarProps) {
  const barColor =
    percent >= 90
      ? "bg-terra-400"
      : percent >= 70
        ? "bg-harvest-300"
        : "bg-harvest-500";

  return (
    <div className="flex items-center gap-3">
      <span className="text-xs text-espresso-500 w-10 shrink-0 font-medium">
        {label}
      </span>
      <div className="flex-1 h-2 bg-cream-200 rounded-full overflow-hidden">
        <motion.div
          className={clsx("h-full rounded-full", barColor)}
          initial={{ width: 0 }}
          animate={{ width: `${Math.min(percent, 100)}%` }}
          transition={{ duration: 0.6, ease: "easeOut" }}
        />
      </div>
      <span className="text-xs font-mono tabular-nums text-espresso-600 w-10 text-right font-medium">
        {percent}%
      </span>
      <span className="text-xs text-espresso-400 w-16 text-right tabular-nums">
        {formatReset(resetSec)}
      </span>
    </div>
  );
}
