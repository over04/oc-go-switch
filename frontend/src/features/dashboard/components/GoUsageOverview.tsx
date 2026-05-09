import { motion } from "framer-motion";
import clsx from "clsx";
import { UsageBar } from "@/features/pool/components/UsageBar";
import type { WorkspaceStatus } from "@/shared/types/api";

interface GoUsageOverviewProps {
  workspace: WorkspaceStatus;
  delay?: number;
}

export function GoUsageOverview({
  workspace,
  delay = 0,
}: GoUsageOverviewProps) {
  if (!workspace.go_usage) return null;
  const u = workspace.go_usage;

  // 判断整体状态
  const maxPct = Math.max(u.hourly_percent, u.weekly_percent, u.monthly_percent);
  const statusColor =
    maxPct >= 90
      ? "border-terra-400/30"
      : maxPct >= 70
        ? "border-harvest-300/30"
        : "border-harvest-500/20";

  return (
    <motion.div
      initial={{ opacity: 0, y: 8 }}
      animate={{ opacity: 1, y: 0 }}
      transition={{ delay, duration: 0.35, ease: "easeOut" }}
      whileHover={{ y: -2 }}
      className={clsx(
        "bg-white rounded-mcm-xl border shadow-mcm px-5 py-4 transition-shadow duration-200 hover:shadow-mcm-md",
        statusColor,
      )}
    >
      <div className="flex items-center justify-between mb-4">
        <div className="flex items-center gap-2.5">
          <span className="w-2 h-2 rounded-full bg-harvest-500" />
          <span className="text-sm font-semibold text-espresso-700">
            {workspace.name}
          </span>
        </div>
        <span className="text-xs font-semibold text-harvest-500 bg-harvest-500/5 px-3 py-0.5 rounded-full border border-harvest-500/15">
          GO
        </span>
      </div>

      <div className="space-y-2.5">
        <UsageBar
          label="小时"
          percent={u.hourly_percent}
          resetSec={u.hourly_reset_sec}
        />
        <UsageBar
          label="本周"
          percent={u.weekly_percent}
          resetSec={u.weekly_reset_sec}
        />
        <UsageBar
          label="本月"
          percent={u.monthly_percent}
          resetSec={u.monthly_reset_sec}
        />
      </div>
    </motion.div>
  );
}
