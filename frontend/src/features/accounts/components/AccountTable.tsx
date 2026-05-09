import { useState } from "react";
import { motion } from "framer-motion";
import { Button } from "@/shared/ui/Button";
import { Input } from "@/shared/ui/Input";
import type { AccountListEntry } from "@/shared/types/api";

interface AccountTableProps {
  accounts: AccountListEntry[];
  onDelete: (name: string) => void;
  deleting: string | null;
  onAddClick: () => void;
  onEdit: (name: string, patch: { label?: string }) => Promise<void>;
}

export function AccountTable({
  accounts,
  onDelete,
  deleting,
  onAddClick,
  onEdit,
}: AccountTableProps) {
  const [editing, setEditing] = useState<string | null>(null);
  const [editLabel, setEditLabel] = useState("");

  const startEdit = (name: string, currentLabel: string) => {
    setEditing(name);
    setEditLabel(currentLabel);
  };

  const saveEdit = async (name: string) => {
    if (editLabel.trim()) {
      await onEdit(name, { label: editLabel.trim() });
    }
    setEditing(null);
  };

  return (
    <div>
      <div className="flex items-center justify-between mb-3">
        <h2 className="text-xs font-semibold">账户 · {accounts.length}</h2>
        <Button size="xs" tone="primary" onClick={onAddClick}>
          添加账户
        </Button>
      </div>

      <div className="bg-white dark:bg-gray-800 rounded border border-gray-200 dark:border-gray-700 overflow-hidden">
        <table className="w-full">
          <thead>
            <tr className="border-b border-gray-100 dark:border-gray-700">
              <th className="px-2.5 py-1.5 text-left text-2xs font-medium text-gray-400 dark:text-gray-500">标签</th>
              <th className="px-2.5 py-1.5 text-left text-2xs font-medium text-gray-400 dark:text-gray-500">名称</th>
              <th className="px-2.5 py-1.5 text-left text-2xs font-medium text-gray-400 dark:text-gray-500">Auth</th>
              <th className="px-2.5 py-1.5 text-right text-2xs font-medium text-gray-400 dark:text-gray-500 w-28">操作</th>
            </tr>
          </thead>
          <tbody>
            {accounts.map((a) => (
              <motion.tr
                key={a.name}
                initial={{ opacity: 0 }}
                animate={{ opacity: 1 }}
                whileHover={{ backgroundColor: "rgba(0,0,0,0.02)" }}
                className="border-b border-gray-50 dark:border-gray-800/50 last:border-0"
              >
                <td className="px-2.5 py-1.5">
                  {editing === a.name ? (
                    <Input
                      size="xs"
                      value={editLabel}
                      onChange={(e) => setEditLabel(e.target.value)}
                      onKeyDown={(e) => {
                        if (e.key === "Enter") saveEdit(a.name);
                        if (e.key === "Escape") setEditing(null);
                      }}
                      autoFocus
                      className="!h-5 !text-2xs w-24"
                    />
                  ) : (
                    <span
                      className="text-2xs font-medium text-gray-700 dark:text-gray-200 cursor-pointer hover:text-blue-500"
                      onClick={() => startEdit(a.name, a.label)}
                      title="点击编辑"
                    >
                      {a.label}
                    </span>
                  )}
                </td>
                <td className="px-2.5 py-1.5 text-2xs text-gray-500 dark:text-gray-400">
                  {a.name}
                </td>
                <td className="px-2.5 py-1.5">
                  <code className="text-2xs font-mono text-gray-400">{a.auth_masked}</code>
                </td>
                <td className="px-2.5 py-1.5 text-right">
                  <div className="flex items-center justify-end gap-1">
                    {editing === a.name ? (
                      <>
                        <Button size="xs" tone="primary" onClick={() => saveEdit(a.name)}>保存</Button>
                        <Button size="xs" onClick={() => setEditing(null)}>取消</Button>
                      </>
                    ) : (
                      <>
                        <Button size="xs" onClick={() => startEdit(a.name, a.label)}>编辑</Button>
                        <Button size="xs" tone="danger" onClick={() => onDelete(a.name)} disabled={deleting === a.name}>
                          {deleting === a.name ? "..." : "删除"}
                        </Button>
                      </>
                    )}
                  </div>
                </td>
              </motion.tr>
            ))}
          </tbody>
        </table>
        {accounts.length === 0 && (
          <motion.p initial={{ opacity: 0 }} animate={{ opacity: 1 }} className="text-2xs text-gray-400 text-center py-8">
            暂无账户，点击"添加账户"开始
          </motion.p>
        )}
      </div>
    </div>
  );
}
