import { motion } from "framer-motion";
import { useDashboardStatus } from "@/features/dashboard/logic/useDashboardStatus";
import { REFRESH_INTERVAL_MS } from "@/shared/config";
import { StatCard } from "../components/StatCard";
import { GoUsageOverview } from "../components/GoUsageOverview";
import { Skeleton } from "@/shared/ui/Skeleton";

export function DashboardScreen() {
  const { data, isPending, isError, dataUpdatedAt } = useDashboardStatus();

  if (isPending) {
    return (
      <div className="space-y-6">
        <div className="grid grid-cols-2 lg:grid-cols-[1.5fr_1fr_1fr_1fr_1fr] gap-3 md:gap-4">
          <Skeleton className="h-28" />
          <Skeleton className="h-28" />
          <Skeleton className="h-28" />
          <Skeleton className="h-28" />
        </div>
        <Skeleton className="h-40" />
      </div>
    );
  }

  if (isError || !data) {
    return (
      <motion.div
        initial={{ opacity: 0 }}
        animate={{ opacity: 1 }}
        className="flex items-center justify-center h-64"
      >
        <div className="text-center space-y-3">
          <div className="w-16 h-16 mx-auto rounded-full bg-terra-400/10 flex items-center justify-center">
            <span className="text-2xl text-terra-400">!</span>
          </div>
          <p className="text-sm text-espresso-500">无法连接后端服务</p>
          <p className="text-xs text-espresso-400">请检查服务是否正常运行</p>
        </div>
      </motion.div>
    );
  }

  const goWorkspaces = data.go_workspaces;
  const totalWorkspaces = data.total_workspaces;
  const availablePercent =
    totalWorkspaces > 0
      ? Math.round((data.available_workspaces / totalWorkspaces) * 100)
      : 0;

  return (
    <div className="max-w-5xl space-y-8">
      <div className="grid grid-cols-2 lg:grid-cols-[1.5fr_1fr_1fr_1fr_1fr] gap-3 md:gap-4">
        <motion.div
          initial={{ opacity: 0, y: 12 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.4 }}
          className="relative overflow-hidden bg-white rounded-mcm-xl border border-cream-200 shadow-mcm p-5"
        >
          <div className="absolute -top-6 -right-6 w-24 h-24 opacity-[0.06]">
            <svg viewBox="0 0 100 100">
              <circle cx="50" cy="50" r="48" fill="none" stroke="currentColor" strokeWidth="1" className="text-espresso-700" />
              <line x1="50" y1="5" x2="50" y2="95" stroke="currentColor" strokeWidth="0.5" className="text-espresso-700" />
              <line x1="5" y1="50" x2="95" y2="50" stroke="currentColor" strokeWidth="0.5" className="text-espresso-700" />
              <line x1="18" y1="18" x2="82" y2="82" stroke="currentColor" strokeWidth="0.5" className="text-espresso-700" />
              <line x1="82" y1="18" x2="18" y2="82" stroke="currentColor" strokeWidth="0.5" className="text-espresso-700" />
            </svg>
          </div>

          <span className="text-xs text-espresso-400 uppercase tracking-[0.15em] font-semibold">
            工作区总数
          </span>
          <motion.p
            className="text-5xl font-bold text-espresso-700 mt-2 tracking-tight tabular-nums"
            initial={{ scale: 0.8, opacity: 0 }}
            animate={{ scale: 1, opacity: 1 }}
            transition={{ delay: 0.15, duration: 0.4 }}
          >
            {totalWorkspaces}
          </motion.p>
          <div className="flex items-center gap-2 mt-3">
            <div className="flex-1 h-1.5 bg-cream-200 rounded-full overflow-hidden">
              <motion.div
                className="h-full bg-harvest-500 rounded-full"
                initial={{ width: 0 }}
                animate={{ width: `${availablePercent}%` }}
                transition={{ delay: 0.3, duration: 0.8, ease: "easeOut" }}
              />
            </div>
            <span className="text-xs text-espresso-400 tabular-nums">
              {availablePercent}% 可用
            </span>
          </div>
        </motion.div>

        <StatCard
          label="可调度"
          value={data.available_workspaces}
          tone="success"
          delay={0.05}
        />
        <StatCard
          label="可用工作区"
          value={data.available_workspaces}
          tone="info"
          delay={0.1}
        />
        <StatCard
          label="当前无额度"
          value={data.exhausted_workspaces}
          tone="danger"
          delay={0.15}
        />
        <StatCard
          label="无订阅"
          value={data.unsubscribed_workspaces}
          tone="default"
          delay={0.2}
        />
      </div>

      {goWorkspaces.length > 0 && (
        <section>
          <div className="flex items-center gap-3 mb-4">
            <div className="w-1 h-5 bg-harvest-500 rounded-full" />
            <h2 className="text-sm font-semibold text-espresso-700 uppercase tracking-wider">
              Go 使用量
            </h2>
          </div>
          <div className="grid grid-cols-1 lg:grid-cols-2 gap-4">
            {goWorkspaces.map((ws, i) => (
              <GoUsageOverview key={ws.id} workspace={ws} delay={i * 0.06} />
            ))}
          </div>
        </section>
      )}

      {goWorkspaces.length === 0 && (
        <motion.div
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          className="text-center py-16"
        >
          <div className="w-20 h-20 mx-auto rounded-full bg-cream-100 flex items-center justify-center mb-4">
            <span className="text-3xl">+</span>
          </div>
          <p className="text-sm text-espresso-500 mb-1">尚无账户</p>
          <p className="text-xs text-espresso-400">
            前往设置页面添加第一个账户
          </p>
        </motion.div>
      )}

      <div className="flex items-center gap-2 text-xs text-espresso-300">
        <span className="w-1.5 h-1.5 rounded-full bg-harvest-500/40" />
        自动刷新 {REFRESH_INTERVAL_MS / 1000}s · 上次更新{" "}
        {dataUpdatedAt
          ? new Date(dataUpdatedAt).toLocaleTimeString()
          : "--:--:--"}
      </div>
    </div>
  );
}
