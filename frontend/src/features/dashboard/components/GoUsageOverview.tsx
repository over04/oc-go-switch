import { motion } from "framer-motion";
import { UsageBar } from "@/features/pool/components/UsageBar";
import type { WorkspaceStatus } from "@/shared/types/api";

interface GoUsageOverviewProps {
  workspace: WorkspaceStatus;
  delay?: number;
}

export function GoUsageOverview({ workspace, delay = 0 }: GoUsageOverviewProps) {
  if (!workspace.go_usage) return null;
  const u = workspace.go_usage;

  return (
    <motion.div
      initial={{ opacity: 0, y: 8 }}
      animate={{ opacity: 1, y: 0 }}
      transition={{ delay, duration: 0.3 }}
      whileHover={{ y: -1, boxShadow: "0 4px 12px rgba(0,0,0,0.08)" }}
      className="bg-white dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700 px-3 py-2.5 shadow-sm transition-shadow"
    >
      <div className="flex items-center justify-between mb-2">
        <span className="text-xs font-medium">{workspace.name}</span>
        <span className="text-2xs text-green-600 dark:text-green-400 font-medium">Go</span>
      </div>
      <div className="space-y-1">
        <UsageBar label="小时" percent={u.hourly_percent} resetSec={u.hourly_reset_sec} />
        <UsageBar label="本周" percent={u.weekly_percent} resetSec={u.weekly_reset_sec} />
        <UsageBar label="本月" percent={u.monthly_percent} resetSec={u.monthly_reset_sec} />
      </div>
    </motion.div>
  );
}
