import clsx from "clsx";

interface UsageBarProps {
  label: string;
  percent: number;
  resetSec: number;
}

function formatReset(sec: number): string {
  if (sec < 60) return `${sec}s`;
  if (sec < 3600) return `${Math.floor(sec / 60)}m`;
  if (sec < 86400) return `${Math.floor(sec / 3600)}h ${Math.floor((sec % 3600) / 60)}m`;
  return `${Math.floor(sec / 86400)}d ${Math.floor((sec % 86400) / 3600)}h`;
}

export function UsageBar({ label, percent, resetSec }: UsageBarProps) {
  const barColor =
    percent >= 90
      ? "bg-red-500"
      : percent >= 70
        ? "bg-amber-500"
        : "bg-green-500";

  return (
    <div className="flex items-center gap-2">
      <span className="text-2xs text-gray-500 dark:text-gray-400 w-12 shrink-0">
        {label}
      </span>
      <div className="flex-1 h-1.5 bg-gray-200 dark:bg-gray-700 rounded-full overflow-hidden">
        <div
          className={clsx("h-full rounded-full transition-all", barColor)}
          style={{ width: `${Math.min(percent, 100)}%` }}
        />
      </div>
      <span className="text-2xs font-mono tabular-nums text-gray-600 dark:text-gray-300 w-8 text-right">
        {percent}%
      </span>
      <span className="text-2xs text-gray-400 w-14 text-right tabular-nums">
        {formatReset(resetSec)}
      </span>
    </div>
  );
}
