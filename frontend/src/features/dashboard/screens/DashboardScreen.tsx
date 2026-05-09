import { motion } from "framer-motion";
import { usePoolStatus } from "@/features/pool/logic/usePoolStatus";
import { REFRESH_INTERVAL_MS } from "@/shared/config";
import { StatCard } from "../components/StatCard";
import { GoUsageOverview } from "../components/GoUsageOverview";
import { Skeleton } from "@/shared/ui/Skeleton";

const container = {
  hidden: { opacity: 0 },
  show: { opacity: 1, transition: { staggerChildren: 0.06 } },
};

export function DashboardScreen() {
  const { data, isPending, isError, dataUpdatedAt } = usePoolStatus();

  if (isPending) {
    return (
      <div className="space-y-4">
        <div className="grid grid-cols-4 gap-3">
          {[1, 2, 3, 4].map((i) => (
            <Skeleton key={i} className="h-16" />
          ))}
        </div>
        <Skeleton className="h-32" />
      </div>
    );
  }

  if (isError || !data) {
    return (
      <motion.p
        initial={{ opacity: 0 }}
        animate={{ opacity: 1 }}
        className="text-xs text-red-500"
      >
        无法加载数据，请检查后端是否正常运行
      </motion.p>
    );
  }

  const goWorkspaces = data.accounts.flatMap((a) =>
    a.workspaces.filter((w) => w.subscribed),
  );

  return (
    <motion.div variants={container} initial="hidden" animate="show" className="space-y-4">
      {/* Stat cards */}
      <div className="grid grid-cols-4 gap-3">
        <StatCard label="密钥总数" value={data.total_keys} delay={0} />
        <StatCard label="可用" value={data.available_keys} tone="success" delay={0.05} />
        <StatCard label="耗尽" value={data.depleted_keys} tone="danger" delay={0.1} />
        <StatCard label="Go 工作区" value={goWorkspaces.length} tone="info" delay={0.15} />
      </div>

      {/* Go usage cards */}
      {goWorkspaces.length > 0 && (
        <motion.div variants={container} initial="hidden" animate="show">
          <h2 className="text-xs font-medium text-gray-500 dark:text-gray-400 mb-2">
            Go 使用量
          </h2>
          <div className="grid grid-cols-1 lg:grid-cols-2 gap-3">
            {goWorkspaces.map((ws, i) => (
              <GoUsageOverview key={ws.id} workspace={ws} delay={i * 0.05} />
            ))}
          </div>
        </motion.div>
      )}

      {/* Empty state */}
      {data.accounts.length === 0 && (
        <motion.p
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          className="text-2xs text-gray-400 text-center py-8"
        >
          暂无账户，请到设置页面添加
        </motion.p>
      )}

      {/* Last refresh */}
      <p className="text-2xs text-gray-400">
        自动刷新 {REFRESH_INTERVAL_MS / 1000}s · 上次更新:{" "}
        {dataUpdatedAt ? new Date(dataUpdatedAt).toLocaleTimeString() : "--:--:--"}
      </p>
    </motion.div>
  );
}
