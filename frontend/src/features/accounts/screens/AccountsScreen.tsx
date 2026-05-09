import { useState, useCallback } from "react";
import { useQuery, useQueryClient } from "@tanstack/react-query";
import { motion, AnimatePresence } from "framer-motion";
import { Button } from "@/shared/ui/Button";
import { Input } from "@/shared/ui/Input";
import { Card } from "@/shared/ui/Card";
import { Badge } from "@/shared/ui/Badge";
import { WorkspaceSection } from "@/features/pool/components/WorkspaceSection";
import { AddAccountDialog } from "@/features/settings/components/AddAccountDialog";
import { getAccounts, addAccount, deleteAccount, editAccount } from "@/features/settings/service/settingsService";
import { toastSuccess, toastError } from "@/shared/lib/toast";
import { Skeleton } from "@/shared/ui/Skeleton";
import { POOL_STATUS_KEY } from "@/features/pool/logic/usePoolStatus";
import { usePoolStatus } from "@/features/pool/logic/usePoolStatus";

const ACCOUNTS_KEY = ["api", "accounts"] as const;

export function AccountsScreen() {
  const queryClient = useQueryClient();
  const { data: accountList, isPending, isError } = useQuery({ queryKey: ACCOUNTS_KEY, queryFn: getAccounts, staleTime: 30_000 });
  const { data: poolData } = usePoolStatus();
  const [showAdd, setShowAdd] = useState(false);
  const [deleting, setDeleting] = useState<string | null>(null);
  const [editing, setEditing] = useState<string | null>(null);
  const [editLabel, setEditLabel] = useState("");

  const handleDelete = useCallback(async (name: string) => {
    if (!confirm(`确认删除账户 "${name}"？`)) return;
    setDeleting(name);
    try {
      await deleteAccount(name);
      await Promise.all([queryClient.invalidateQueries({ queryKey: ACCOUNTS_KEY }), queryClient.invalidateQueries({ queryKey: POOL_STATUS_KEY })]);
      toastSuccess(`已删除 "${name}"`);
    } catch (e) { toastError(e instanceof Error ? e.message : "删除失败"); }
    finally { setDeleting(null); }
  }, [queryClient]);

  const handleAdd = useCallback(async (name: string, auth: string, label: string) => {
    await addAccount(name, auth, label);
    await Promise.all([queryClient.invalidateQueries({ queryKey: ACCOUNTS_KEY }), queryClient.invalidateQueries({ queryKey: POOL_STATUS_KEY })]);
    toastSuccess(`已添加 "${label}"`);
  }, [queryClient]);

  const startEdit = (name: string, label: string) => { setEditing(name); setEditLabel(label); };
  const saveEdit = useCallback(async (name: string) => {
    if (!editLabel.trim()) return;
    await editAccount(name, { label: editLabel.trim() });
    await queryClient.invalidateQueries({ queryKey: ACCOUNTS_KEY });
    setEditing(null);
    toastSuccess("已更新");
  }, [editLabel, queryClient]);

  if (isPending) return <Skeleton className="h-48" />;
  if (isError) return <p className="text-xs text-red-500">加载账户失败</p>;

  return (
    <div>
      <div className="flex items-center justify-between mb-3">
        <h2 className="text-sm font-semibold">账户 · {accountList?.accounts.length ?? 0}</h2>
        <Button size="xs" tone="primary" onClick={() => setShowAdd(true)}>添加账户</Button>
      </div>

      <div className="space-y-3">
        {accountList?.accounts.map((acct) => {
          const poolAcct = poolData?.accounts.find((a) => a.name === acct.name);
          const workspaces = poolAcct?.workspaces ?? [];
          const keyCount = workspaces.reduce((s, w) => s + w.keys.length, 0);

          return (
            <motion.div key={acct.name} initial={{ opacity: 0, y: 8 }} animate={{ opacity: 1, y: 0 }}>
              <Card>
                <div className="flex items-center justify-between pb-2 mb-2 border-b border-gray-100 dark:border-gray-700">
                  <div className="flex items-center gap-2">
                    {editing === acct.name ? (
                      <div className="flex items-center gap-1">
                        <Input size="xs" value={editLabel} onChange={(e) => setEditLabel(e.target.value)} onKeyDown={(e) => { if (e.key === "Enter") saveEdit(acct.name); if (e.key === "Escape") setEditing(null); }} autoFocus className="!w-24" />
                        <Button size="xs" tone="primary" onClick={() => saveEdit(acct.name)}>保存</Button>
                        <Button size="xs" onClick={() => setEditing(null)}>取消</Button>
                      </div>
                    ) : (
                      <>
                        <span className="text-sm font-medium text-gray-800 dark:text-gray-100 cursor-pointer hover:text-blue-500" onClick={() => startEdit(acct.name, acct.label)} title="点击编辑标签">{acct.label}</span>
                        <span className="text-xs text-gray-400">{acct.name}</span>
                        <code className="text-xs text-gray-400 font-mono">{acct.auth_masked}</code>
                        <Button size="xs" onClick={() => startEdit(acct.name, acct.label)}>编辑</Button>
                      </>
                    )}
                  </div>
                  <div className="flex items-center gap-2">
                    <span className="text-xs text-gray-400">{workspaces.length} 工作区 · {keyCount} 密钥</span>
                    <Button size="xs" tone="danger" onClick={() => handleDelete(acct.name)} disabled={deleting === acct.name}>
                      {deleting === acct.name ? "..." : "删除"}
                    </Button>
                  </div>
                </div>
                {workspaces.length === 0 ? (
                  <p className="text-xs text-gray-400 py-2">暂无工作区数据，等待密钥池刷新</p>
                ) : (
                  workspaces.map((ws) => <WorkspaceSection key={ws.id} workspace={ws} />)
                )}
              </Card>
            </motion.div>
          );
        })}
      </div>
      {accountList?.accounts.length === 0 && <p className="text-xs text-gray-400 text-center py-8">暂无账户</p>}
      <AddAccountDialog open={showAdd} onClose={() => setShowAdd(false)} onAdd={handleAdd} />
    </div>
  );
}
