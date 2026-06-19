import { useCallback, useState } from "react";
import { useQueryClient } from "@tanstack/react-query";
import { motion, AnimatePresence } from "framer-motion";
import {
  useWorkspaceSchedule,
  WORKSPACE_SCHEDULE_KEY,
} from "@/features/workspaces/logic/useWorkspaceSchedule";
import { AffinityWorkspaceBar } from "../components/AffinityWorkspaceBar";
import { WorkspaceScheduleGrid } from "../components/WorkspaceScheduleGrid";
import { WorkspaceFilterBar } from "../components/WorkspaceFilterBar";
import { Skeleton } from "@/shared/ui/Skeleton";
import { Button } from "@/shared/ui/Button";
import {
  setAffinityWorkspace,
  clearAffinityWorkspace,
  refreshWorkspaces,
} from "@/features/settings/service/settingsService";
import { toastSuccess, toastError } from "@/shared/lib/toast";
import type { WorkspaceStatusKind } from "@/shared/types/api";

export function WorkspacesScreen() {
  const queryClient = useQueryClient();
  const { data, isPending, isError } = useWorkspaceSchedule();
  const [refreshing, setRefreshing] = useState(false);
  const [search, setSearch] = useState("");
  const [workspaceFilter, setWorkspaceFilter] = useState<WorkspaceStatusKind | "all">("all");

  const handleSetAffinity = useCallback(
    async (workspaceId: string) => {
      try {
        await setAffinityWorkspace(workspaceId);
        await queryClient.invalidateQueries({ queryKey: WORKSPACE_SCHEDULE_KEY });
        toastSuccess("已设置亲和工作区");
      } catch (e) {
        toastError(e instanceof Error ? e.message : "切换失败");
      }
    },
    [queryClient],
  );

  const handleClearAffinity = useCallback(async () => {
    try {
      await clearAffinityWorkspace();
      await queryClient.invalidateQueries({ queryKey: WORKSPACE_SCHEDULE_KEY });
      toastSuccess("已清除亲和工作区");
    } catch (e) {
      toastError(e instanceof Error ? e.message : "操作失败");
    }
  }, [queryClient]);

  const handleRefresh = useCallback(async () => {
    setRefreshing(true);
    try {
      await refreshWorkspaces();
      await queryClient.invalidateQueries({ queryKey: WORKSPACE_SCHEDULE_KEY });
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

  const goAccounts = data.accounts.filter((a) => a.workspaces.length > 0);
  const affinityWorkspaceName =
    goAccounts
      .flatMap((account) => account.workspaces)
      .find((workspace) => workspace.id === data.affinity_workspace_id)?.name ??
    null;

  return (
    <div className="max-w-5xl space-y-5">
      <div className="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-3">
        <div className="flex items-center gap-3">
          <div className="w-1 h-6 bg-terra-500 rounded-full" />
          <div>
            <h2 className="text-lg font-semibold text-espresso-700 tracking-tight">
              工作区调度
            </h2>
            <p className="text-xs text-espresso-400 mt-0.5">
              {goAccounts.length} 账户
            </p>
          </div>
        </div>
        <Button
          size="sm"
          tone="primary"
          onClick={handleRefresh}
          disabled={refreshing}
        >
          {refreshing ? "刷新中..." : "刷新工作区状态"}
        </Button>
      </div>

      <AffinityWorkspaceBar
        affinityWorkspaceName={affinityWorkspaceName}
        onClear={handleClearAffinity}
      />

      <WorkspaceFilterBar
        search={search}
        onSearchChange={setSearch}
        workspaceFilter={workspaceFilter}
        onWorkspaceFilterChange={setWorkspaceFilter}
      />

      {goAccounts.length === 0 ? (
        <motion.div
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          className="text-center py-16"
        >
          <div className="w-16 h-16 mx-auto rounded-full bg-cream-100 flex items-center justify-center mb-3">
            <span className="text-2xl text-espresso-300">◆</span>
          </div>
          <p className="text-sm text-espresso-500">暂无工作区数据</p>
        </motion.div>
      ) : (
        <WorkspaceScheduleGrid
          accounts={goAccounts}
          affinityWorkspaceId={data.affinity_workspace_id}
          onSetAffinity={handleSetAffinity}
          search={search}
          workspaceFilter={workspaceFilter}
        />
      )}
    </div>
  );
}
