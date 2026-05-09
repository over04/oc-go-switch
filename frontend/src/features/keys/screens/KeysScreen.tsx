import { useCallback, useState } from "react";
import { useQueryClient } from "@tanstack/react-query";
import { motion, AnimatePresence } from "framer-motion";
import { usePoolStatus, POOL_STATUS_KEY } from "@/features/pool/logic/usePoolStatus";
import { ActiveKeyBar } from "../components/ActiveKeyBar";
import { KeyCardGrid } from "../components/KeyCardGrid";
import { KeyFilterBar } from "../components/KeyFilterBar";
import { Skeleton } from "@/shared/ui/Skeleton";
import { Button } from "@/shared/ui/Button";
import {
  setActiveKey,
  clearActiveKey,
  refreshPool,
} from "@/features/settings/service/settingsService";
import { toastSuccess, toastError } from "@/shared/lib/toast";
import type { KeyStatus } from "@/shared/types/api";

export function KeysScreen() {
  const queryClient = useQueryClient();
  const { data, isPending, isError } = usePoolStatus();
  const [refreshing, setRefreshing] = useState(false);
  const [search, setSearch] = useState("");
  const [statusFilter, setStatusFilter] = useState<KeyStatus | "all">("all");

  const handleSetActive = useCallback(
    async (keyId: string) => {
      try {
        await setActiveKey(keyId);
        await queryClient.invalidateQueries({ queryKey: POOL_STATUS_KEY });
        toastSuccess("已切换活跃密钥");
      } catch (e) {
        toastError(e instanceof Error ? e.message : "切换失败");
      }
    },
    [queryClient],
  );

  const handleClearActive = useCallback(async () => {
    try {
      await clearActiveKey();
      await queryClient.invalidateQueries({ queryKey: POOL_STATUS_KEY });
      toastSuccess("已清除活跃密钥");
    } catch (e) {
      toastError(e instanceof Error ? e.message : "操作失败");
    }
  }, [queryClient]);

  const handleRefresh = useCallback(async () => {
    setRefreshing(true);
    try {
      await refreshPool();
      await queryClient.invalidateQueries({ queryKey: POOL_STATUS_KEY });
      toastSuccess("刷新完成");
    } catch (e) {
      toastError(e instanceof Error ? e.message : "刷新失败");
    } finally {
      setRefreshing(false);
    }
  }, [queryClient]);

  if (isPending)
    return (
      <div className="space-y-4">
        <Skeleton className="h-8 w-48" />
        <Skeleton className="h-64" />
      </div>
    );
  if (isError || !data)
    return (
      <div className="flex items-center justify-center h-64">
        <p className="text-sm text-terra-400">加载失败</p>
      </div>
    );

  const goAccounts = data.accounts
    .map((a) => ({
      ...a,
      workspaces: a.workspaces.filter((w) => w.subscribed),
    }))
    .filter((a) => a.workspaces.length > 0);

  const totalKeys = goAccounts.reduce(
    (s, a) => s + a.workspaces.reduce((s2, w) => s2 + w.keys.length, 0),
    0,
  );

  return (
    <div className="max-w-5xl space-y-5">
      {/* Page header */}
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-3">
          <div className="w-1 h-6 bg-terra-500 rounded-full" />
          <div>
            <h2 className="text-lg font-semibold text-espresso-700 tracking-tight">
              Go 订阅密钥池
            </h2>
            <p className="text-xs text-espresso-400 mt-0.5">
              {goAccounts.length} 账户 · {totalKeys} 密钥
            </p>
          </div>
        </div>
        <Button
          size="sm"
          tone="primary"
          onClick={handleRefresh}
          disabled={refreshing}
        >
          {refreshing ? "刷新中..." : "刷新密钥池"}
        </Button>
      </div>

      {/* Active key bar */}
      <ActiveKeyBar
        activeKeyId={data.current_key_id}
        onClear={handleClearActive}
      />

      {/* Filter bar */}
      <KeyFilterBar
        search={search}
        onSearchChange={setSearch}
        statusFilter={statusFilter}
        onStatusFilterChange={setStatusFilter}
      />

      {/* Key cards by workspace */}
      {goAccounts.length === 0 ? (
        <motion.div
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          className="text-center py-16"
        >
          <div className="w-16 h-16 mx-auto rounded-full bg-cream-100 flex items-center justify-center mb-3">
            <span className="text-2xl text-espresso-300">◆</span>
          </div>
          <p className="text-sm text-espresso-500">暂无 Go 订阅工作区</p>
        </motion.div>
      ) : (
        <KeyCardGrid
          accounts={goAccounts}
          currentKeyId={data.current_key_id}
          onSetActive={handleSetActive}
          search={search}
          statusFilter={statusFilter}
        />
      )}
    </div>
  );
}
