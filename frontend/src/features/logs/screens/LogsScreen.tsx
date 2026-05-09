import { useState, useCallback } from "react";
import { useQuery, useQueryClient } from "@tanstack/react-query";
import { motion } from "framer-motion";
import { Button } from "@/shared/ui/Button";
import { Input } from "@/shared/ui/Input";
import { Badge } from "@/shared/ui/Badge";
import { Skeleton } from "@/shared/ui/Skeleton";
import { getLogs, clearLogs } from "../service/logsService";
import { toastSuccess, toastError } from "@/shared/lib/toast";
import type { LogEntry } from "@/shared/types/api";

const LOGS_KEY = ["api", "logs"] as const;

export function LogsScreen() {
  const queryClient = useQueryClient();
  const [search, setSearch] = useState("");
  const [dirFilter, setDirFilter] = useState<"all" | "openai" | "claude">("all");
  const [successFilter, setSuccessFilter] = useState<"all" | true | false>("all");

  const { data, isPending, isError } = useQuery({
    queryKey: LOGS_KEY,
    queryFn: () => getLogs({ limit: 200 }),
    refetchInterval: 5000,
    staleTime: 4000,
  });

  const handleClear = useCallback(async () => {
    if (!confirm("确认清空所有日志？")) return;
    try {
      await clearLogs();
      queryClient.invalidateQueries({ queryKey: LOGS_KEY });
      toastSuccess("日志已清空");
    } catch (e) {
      toastError(e instanceof Error ? e.message : "清空失败");
    }
  }, [queryClient]);

  const filtered = (data ?? []).filter((e) => {
    if (dirFilter !== "all" && e.direction !== dirFilter) return false;
    if (successFilter !== "all" && e.success !== successFilter) return false;
    if (search && !e.model?.toLowerCase().includes(search.toLowerCase()) && !e.key_masked.toLowerCase().includes(search.toLowerCase())) return false;
    return true;
  });

  return (
    <div>
      <div className="flex items-center justify-between mb-3">
        <h2 className="text-sm font-semibold">请求日志 · {filtered.length} 条</h2>
        <Button size="xs" tone="danger" onClick={handleClear}>清空日志</Button>
      </div>

      <div className="flex items-center gap-2 mb-3">
        <Input placeholder="搜索模型或密钥..." value={search} onChange={(e) => setSearch(e.target.value)} className="max-w-[200px]" />
        <select value={dirFilter} onChange={(e) => setDirFilter(e.target.value as typeof dirFilter)} className="h-7 px-2 text-xs rounded border border-gray-200 dark:border-gray-600 bg-gray-50 dark:bg-gray-700">
          <option value="all">全部协议</option>
          <option value="openai">OpenAI</option>
          <option value="claude">Claude</option>
        </select>
        <select value={String(successFilter)} onChange={(e) => setSuccessFilter(e.target.value === "all" ? "all" : e.target.value === "true")} className="h-7 px-2 text-xs rounded border border-gray-200 dark:border-gray-600 bg-gray-50 dark:bg-gray-700">
          <option value="all">全部状态</option>
          <option value="true">成功</option>
          <option value="false">失败</option>
        </select>
      </div>

      {isPending ? <Skeleton className="h-64" /> : isError ? (
        <p className="text-xs text-red-500">加载日志失败</p>
      ) : (
        <div className="bg-white dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700 overflow-hidden">
          <table className="w-full">
            <thead>
              <tr className="border-b border-gray-100 dark:border-gray-700">
                <th className="px-3 py-2 text-left text-xs font-medium text-gray-400">时间</th>
                <th className="px-3 py-2 text-left text-xs font-medium text-gray-400">协议</th>
                <th className="px-3 py-2 text-left text-xs font-medium text-gray-400">模型</th>
                <th className="px-3 py-2 text-left text-xs font-medium text-gray-400">状态</th>
                <th className="px-3 py-2 text-left text-xs font-medium text-gray-400">耗时</th>
                <th className="px-3 py-2 text-left text-xs font-medium text-gray-400">密钥</th>
                <th className="px-3 py-2 text-left text-xs font-medium text-gray-400">消息</th>
              </tr>
            </thead>
            <tbody>
              {filtered.map((e, i) => (
                <motion.tr key={`${e.timestamp}-${i}`} initial={{ opacity: 0 }} animate={{ opacity: 1 }} transition={{ delay: i * 0.01 }}
                  className="border-b border-gray-50 dark:border-gray-800/50 last:border-0">
                  <td className="px-3 py-2 text-xs text-gray-500 font-mono whitespace-nowrap">{new Date(e.timestamp).toLocaleTimeString()}</td>
                  <td className="px-3 py-2"><Badge size="xs" tone={e.direction === "openai" ? "default" : "go"}>{e.direction}</Badge></td>
                  <td className="px-3 py-2 text-xs text-gray-600 dark:text-gray-300 max-w-[120px] truncate">{e.model ?? "-"}</td>
                  <td className="px-3 py-2"><Badge size="xs" tone={e.success ? "success" : "danger"}>{e.status_code}</Badge></td>
                  <td className="px-3 py-2 text-xs font-mono text-gray-500">{e.duration_ms}ms</td>
                  <td className="px-3 py-2 text-xs font-mono text-gray-400">{e.key_masked}</td>
                  <td className="px-3 py-2 text-xs text-gray-400 max-w-[150px] truncate">{e.error_message ?? "-"}</td>
                </motion.tr>
              ))}
            </tbody>
          </table>
          {filtered.length === 0 && <p className="text-xs text-gray-400 text-center py-8">暂无日志</p>}
        </div>
      )}
    </div>
  );
}
