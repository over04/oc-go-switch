import { useCallback, useState } from "react";
import { useQueryClient } from "@tanstack/react-query";
import { usePoolStatus, POOL_STATUS_KEY } from "@/features/pool/logic/usePoolStatus";
import { KeyTable } from "../components/KeyTable";
import { ActiveKeyBar } from "../components/ActiveKeyBar";
import { Skeleton } from "@/shared/ui/Skeleton";
import { Button } from "@/shared/ui/Button";
import { setActiveKey, clearActiveKey, refreshPool } from "@/features/settings/service/settingsService";
import { toastSuccess, toastError } from "@/shared/lib/toast";

export function KeysScreen() {
  const queryClient = useQueryClient();
  const { data, isPending, isError } = usePoolStatus();
  const [refreshing, setRefreshing] = useState(false);

  const handleSetActive = useCallback(async (keyId: string) => {
    try {
      await setActiveKey(keyId);
      await queryClient.invalidateQueries({ queryKey: POOL_STATUS_KEY });
      toastSuccess("已切换活跃密钥");
    } catch (e) { toastError(e instanceof Error ? e.message : "切换失败"); }
  }, [queryClient]);

  const handleClearActive = useCallback(async () => {
    try {
      await clearActiveKey();
      await queryClient.invalidateQueries({ queryKey: POOL_STATUS_KEY });
      toastSuccess("已清除活跃密钥");
    } catch (e) { toastError(e instanceof Error ? e.message : "操作失败"); }
  }, [queryClient]);

  const handleRefresh = useCallback(async () => {
    setRefreshing(true);
    try {
      await refreshPool();
      await queryClient.invalidateQueries({ queryKey: POOL_STATUS_KEY });
      toastSuccess("刷新完成");
    } catch (e) { toastError(e instanceof Error ? e.message : "刷新失败"); }
    finally { setRefreshing(false); }
  }, [queryClient]);

  if (isPending) return <div className="space-y-3"><Skeleton className="h-7 w-48" /><Skeleton className="h-64" /></div>;
  if (isError || !data) return <p className="text-xs text-red-500">加载失败</p>;

  // Only Go-subscribed workspace keys
  const goAccounts = data.accounts
    .map((a) => ({ ...a, workspaces: a.workspaces.filter((w) => w.subscribed) }))
    .filter((a) => a.workspaces.length > 0);

  return (
    <div>
      <div className="flex items-center justify-between mb-3">
        <h2 className="text-sm font-semibold">Go订阅密钥池 · {goAccounts.reduce((s, a) => s + a.workspaces.reduce((s2, w) => s2 + w.keys.length, 0), 0)} 密钥 · {goAccounts.reduce((s, a) => s + a.workspaces.length, 0)} 工作区</h2>
        <Button size="xs" tone="primary" onClick={handleRefresh} disabled={refreshing}>
          {refreshing ? "刷新中..." : "刷新密钥池"}
        </Button>
      </div>
      <ActiveKeyBar activeKeyId={data.current_key_id} onClear={handleClearActive} />
      {goAccounts.length === 0 ? (
        <p className="text-xs text-gray-400 text-center py-8">暂无 Go 订阅工作区</p>
      ) : (
        <KeyTable accounts={goAccounts} currentKeyId={data.current_key_id} onSetActive={handleSetActive} />
      )}
    </div>
  );
}
