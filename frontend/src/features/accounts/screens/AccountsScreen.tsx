import { useState, useCallback } from "react";
import { useQuery, useQueryClient } from "@tanstack/react-query";
import { motion } from "framer-motion";
import { Button } from "@/shared/ui/Button";
import { Input } from "@/shared/ui/Input";
import { Card } from "@/shared/ui/Card";
import { WorkspaceSection } from "@/features/pool/components/WorkspaceSection";
import { AddAccountDialog } from "@/features/settings/components/AddAccountDialog";
import {
  getAccounts,
  addAccount,
  deleteAccount,
  editAccount,
} from "@/features/settings/service/settingsService";
import { toastSuccess, toastError } from "@/shared/lib/toast";
import { Skeleton } from "@/shared/ui/Skeleton";
import { POOL_STATUS_KEY } from "@/features/pool/logic/usePoolStatus";
import { usePoolStatus } from "@/features/pool/logic/usePoolStatus";

const ACCOUNTS_KEY = ["api", "accounts"] as const;

export function AccountsScreen() {
  const queryClient = useQueryClient();
  const {
    data: accountList,
    isPending,
    isError,
  } = useQuery({
    queryKey: ACCOUNTS_KEY,
    queryFn: getAccounts,
    staleTime: 30_000,
  });
  const { data: poolData } = usePoolStatus();
  const [showAdd, setShowAdd] = useState(false);
  const [deleting, setDeleting] = useState<string | null>(null);
  const [editing, setEditing] = useState<string | null>(null);
  const [editLabel, setEditLabel] = useState("");

  const handleDelete = useCallback(
    async (name: string) => {
      if (!confirm(`确认删除账户 "${name}"？`)) return;
      setDeleting(name);
      try {
        await deleteAccount(name);
        await Promise.all([
          queryClient.invalidateQueries({ queryKey: ACCOUNTS_KEY }),
          queryClient.invalidateQueries({ queryKey: POOL_STATUS_KEY }),
        ]);
        toastSuccess(`已删除 "${name}"`);
      } catch (e) {
        toastError(e instanceof Error ? e.message : "删除失败");
      } finally {
        setDeleting(null);
      }
    },
    [queryClient],
  );

  const handleAdd = useCallback(
    async (name: string, auth: string, label: string) => {
      await addAccount(name, auth, label);
      await Promise.all([
        queryClient.invalidateQueries({ queryKey: ACCOUNTS_KEY }),
        queryClient.invalidateQueries({ queryKey: POOL_STATUS_KEY }),
      ]);
      toastSuccess(`已添加 "${label}"`);
    },
    [queryClient],
  );

  const startEdit = (name: string, label: string) => {
    setEditing(name);
    setEditLabel(label);
  };
  const saveEdit = useCallback(
    async (name: string) => {
      if (!editLabel.trim()) return;
      await editAccount(name, { label: editLabel.trim() });
      await queryClient.invalidateQueries({ queryKey: ACCOUNTS_KEY });
      setEditing(null);
      toastSuccess("已更新");
    },
    [editLabel, queryClient],
  );

  if (isPending) return <Skeleton className="h-48" />;
  if (isError)
    return (
      <div className="flex items-center justify-center h-48">
        <p className="text-sm text-terra-400">加载账户失败</p>
      </div>
    );

  return (
    <div className="max-w-4xl space-y-5">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-3">
          <div className="w-1 h-6 bg-harvest-300 rounded-full" />
          <div>
            <h2 className="text-lg font-semibold text-espresso-700 tracking-tight">
              账户管理
            </h2>
            <p className="text-xs text-espresso-400 mt-0.5">
              {accountList?.accounts.length ?? 0} 个账户
            </p>
          </div>
        </div>
        <Button size="sm" tone="primary" onClick={() => setShowAdd(true)}>
          + 添加账户
        </Button>
      </div>

      {/* Account cards */}
      <div className="space-y-4">
        {accountList?.accounts.map((acct, i) => {
          const poolAcct = poolData?.accounts.find(
            (a) => a.name === acct.name,
          );
          const workspaces = poolAcct?.workspaces ?? [];
          const keyCount = workspaces.reduce((s, w) => s + w.keys.length, 0);

          return (
            <motion.div
              key={acct.name}
              initial={{ opacity: 0, y: 12 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ delay: i * 0.06, duration: 0.3 }}
            >
              <Card size="sm" className="!p-0 overflow-hidden">
                {/* Account header */}
                <div className="flex items-center justify-between px-5 py-3 border-b border-cream-100">
                  <div className="flex items-center gap-3">
                    <span className="w-2 h-2 rounded-full bg-harvest-500" />
                    {editing === acct.name ? (
                      <div className="flex items-center gap-1.5">
                        <Input
                          size="xs"
                          value={editLabel}
                          onChange={(e) => setEditLabel(e.target.value)}
                          onKeyDown={(e) => {
                            if (e.key === "Enter") saveEdit(acct.name);
                            if (e.key === "Escape") setEditing(null);
                          }}
                          autoFocus
                          className="!w-28"
                        />
                        <Button
                          size="xs"
                          tone="primary"
                          onClick={() => saveEdit(acct.name)}
                        >
                          保存
                        </Button>
                        <Button size="xs" onClick={() => setEditing(null)}>
                          取消
                        </Button>
                      </div>
                    ) : (
                      <>
                        <span
                          className="text-sm font-semibold text-espresso-700 cursor-pointer hover:text-terra-500 transition-colors"
                          onClick={() => startEdit(acct.name, acct.label)}
                          title="点击编辑标签"
                        >
                          {acct.label}
                        </span>
                        <code className="text-xs text-espresso-400 font-mono">
                          {acct.auth_masked}
                        </code>
                      </>
                    )}
                  </div>
                  <div className="flex items-center gap-3">
                    <span className="text-xs text-espresso-400">
                      {workspaces.length} 工作区 · {keyCount} 密钥
                    </span>
                    {!editing && (
                      <Button
                        size="xs"
                        onClick={() =>
                          startEdit(acct.name, acct.label)
                        }
                      >
                        编辑
                      </Button>
                    )}
                    <Button
                      size="xs"
                      tone="danger"
                      onClick={() => handleDelete(acct.name)}
                      disabled={deleting === acct.name}
                    >
                      {deleting === acct.name ? "..." : "删除"}
                    </Button>
                  </div>
                </div>

                {/* Workspaces */}
                <div className="px-5 py-3">
                  {workspaces.length === 0 ? (
                    <p className="text-xs text-espresso-400 py-3 text-center">
                      暂无工作区数据，等待密钥池刷新
                    </p>
                  ) : (
                    <div className="space-y-3">
                      {workspaces.map((ws) => (
                        <WorkspaceSection key={ws.id} workspace={ws} />
                      ))}
                    </div>
                  )}
                </div>
              </Card>
            </motion.div>
          );
        })}
      </div>

      {accountList?.accounts.length === 0 && (
        <motion.div
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          className="text-center py-16"
        >
          <div className="w-20 h-20 mx-auto rounded-full bg-cream-100 flex items-center justify-center mb-4">
            <span className="text-3xl text-espresso-300">+</span>
          </div>
          <p className="text-sm text-espresso-500 mb-1">暂无账户</p>
          <p className="text-xs text-espresso-400 mb-4">
            添加账户以开始管理密钥池
          </p>
          <Button size="sm" tone="primary" onClick={() => setShowAdd(true)}>
            添加第一个账户
          </Button>
        </motion.div>
      )}

      <AddAccountDialog
        open={showAdd}
        onClose={() => setShowAdd(false)}
        onAdd={handleAdd}
      />
    </div>
  );
}
