import { useState, useCallback } from "react";
import { useQuery, useQueryClient } from "@tanstack/react-query";
import { motion, AnimatePresence } from "framer-motion";
import { Button } from "@/shared/ui/Button";
import { Input } from "@/shared/ui/Input";
import { Skeleton } from "@/shared/ui/Skeleton";
import { getLogs, clearLogs } from "../service/logsService";
import { toastSuccess, toastError } from "@/shared/lib/toast";

const LOGS_KEY = ["api", "logs"] as const;

export function LogsScreen() {
  const queryClient = useQueryClient();
  const [search, setSearch] = useState("");
  const [dirFilter, setDirFilter] = useState<"all" | "openai" | "claude">(
    "all",
  );
  const [successFilter, setSuccessFilter] = useState<"all" | true | false>(
    "all",
  );

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
    if (
      search &&
      !e.model?.toLowerCase().includes(search.toLowerCase()) &&
      !e.key_masked.toLowerCase().includes(search.toLowerCase())
    )
      return false;
    return true;
  });

  return (
    <div className="max-w-5xl space-y-5">
      {/* Header */}
      <div className="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-3">
        <div className="flex items-center gap-3">
          <div className="w-1 h-6 bg-espresso-500 rounded-full" />
          <div>
            <h2 className="text-lg font-semibold text-espresso-700 tracking-tight">
              请求日志
            </h2>
            <p className="text-xs text-espresso-400 mt-0.5">
              {filtered.length} 条记录
            </p>
          </div>
        </div>
        <Button size="sm" tone="danger" onClick={handleClear}>
          清空日志
        </Button>
      </div>

      {/* Filters */}
      <div className="flex flex-wrap items-center gap-2 md:gap-3">
        <Input
          placeholder="搜索模型或密钥..."
          value={search}
          onChange={(e) => setSearch(e.target.value)}
          className="max-w-full sm:max-w-[240px]"
        />
        <div className="flex items-center gap-1 bg-cream-100 rounded-full p-0.5">
          {(
            [
              ["all", "全部协议"],
              ["openai", "OpenAI"],
              ["claude", "Claude"],
            ] as const
          ).map(([v, label]) => (
            <button
              key={v}
              onClick={() => setDirFilter(v)}
              className={`px-3 py-1 text-xs rounded-full font-medium transition-all duration-200 ${
                dirFilter === v
                  ? "bg-white text-espresso-700 shadow-sm"
                  : "text-espresso-400 hover:text-espresso-600"
              }`}
            >
              {label}
            </button>
          ))}
        </div>
        <div className="flex items-center gap-1 bg-cream-100 rounded-full p-0.5">
          {(
            [
              ["all", "全部"],
              ["true", "成功"],
              ["false", "失败"],
            ] as const
          ).map(([v, label]) => (
            <button
              key={v}
              onClick={() =>
                setSuccessFilter(
                  v === "all" ? "all" : v === "true" ? true : false,
                )
              }
              className={`px-3 py-1 text-xs rounded-full font-medium transition-all duration-200 ${
                (v === "all" && successFilter === "all") ||
                (v === "true" && successFilter === true) ||
                (v === "false" && successFilter === false)
                  ? "bg-white text-espresso-700 shadow-sm"
                  : "text-espresso-400 hover:text-espresso-600"
              }`}
            >
              {label}
            </button>
          ))}
        </div>
      </div>

      {/* Log table */}
      {isPending ? (
        <Skeleton className="h-64" />
      ) : isError ? (
        <div className="flex items-center justify-center h-48">
          <p className="text-sm text-terra-400">加载日志失败</p>
        </div>
      ) : (
        <div className="bg-white rounded-mcm-lg border border-cream-200 shadow-mcm overflow-hidden">
          <div className="overflow-x-auto">
            <table className="w-full">
              <thead>
                <tr className="border-b border-cream-100">
                  <th className="px-4 py-2.5 text-left text-xs font-semibold text-espresso-400 uppercase tracking-wider">
                    时间
                  </th>
                  <th className="px-4 py-2.5 text-left text-xs font-semibold text-espresso-400 uppercase tracking-wider">
                    协议
                  </th>
                  <th className="px-4 py-2.5 text-left text-xs font-semibold text-espresso-400 uppercase tracking-wider">
                    模型
                  </th>
                  <th className="px-4 py-2.5 text-left text-xs font-semibold text-espresso-400 uppercase tracking-wider">
                    状态
                  </th>
                  <th className="px-4 py-2.5 text-left text-xs font-semibold text-espresso-400 uppercase tracking-wider">
                    耗时
                  </th>
                  <th className="px-4 py-2.5 text-left text-xs font-semibold text-espresso-400 uppercase tracking-wider">
                    密钥
                  </th>
                  <th className="px-4 py-2.5 text-left text-xs font-semibold text-espresso-400 uppercase tracking-wider">
                    备注
                  </th>
                </tr>
              </thead>
              <tbody>
                <AnimatePresence>
                  {filtered.map((e, i) => (
                    <motion.tr
                      key={`${e.timestamp}-${i}`}
                      initial={{ opacity: 0, x: -8 }}
                      animate={{ opacity: 1, x: 0 }}
                      transition={{ delay: i * 0.005, duration: 0.15 }}
                      className="border-b border-cream-50 last:border-0 hover:bg-cream-50/30 transition-colors"
                    >
                      <td className="px-4 py-2.5 text-xs text-espresso-500 font-mono whitespace-nowrap">
                        {new Date(e.timestamp).toLocaleTimeString()}
                      </td>
                      <td className="px-4 py-2.5">
                        <span
                          className={`inline-flex text-xs font-medium px-2 py-0.5 rounded-full ${
                            e.direction === "openai"
                              ? "bg-sky-400/10 text-sky-400"
                              : "bg-harvest-500/10 text-harvest-500"
                          }`}
                        >
                          {e.direction}
                        </span>
                      </td>
                      <td className="px-4 py-2.5 text-xs text-espresso-600 font-mono max-w-[140px] truncate">
                        {e.model ?? "-"}
                      </td>
                      <td className="px-4 py-2.5">
                        <span
                          className={`inline-flex items-center gap-1 text-xs font-medium px-2 py-0.5 rounded-full ${
                            e.success
                              ? "bg-harvest-500/10 text-harvest-500"
                              : "bg-terra-400/10 text-terra-400"
                          }`}
                        >
                          <span
                            className={`w-1 h-1 rounded-full ${
                              e.success ? "bg-harvest-500" : "bg-terra-400"
                            }`}
                          />
                          {e.status_code}
                        </span>
                      </td>
                      <td className="px-4 py-2.5 text-xs text-espresso-500 font-mono tabular-nums">
                        {e.duration_ms}ms
                      </td>
                      <td className="px-4 py-2.5 text-xs text-espresso-400 font-mono">
                        {e.key_masked}
                      </td>
                      <td className="px-4 py-2.5 text-xs text-espresso-400 max-w-[160px] truncate">
                        {e.error_message ?? "-"}
                      </td>
                    </motion.tr>
                  ))}
                </AnimatePresence>
              </tbody>
            </table>
          </div>
          {filtered.length === 0 && (
            <div className="text-center py-12">
              <div className="w-12 h-12 mx-auto rounded-full bg-cream-100 flex items-center justify-center mb-2">
                <span className="text-lg text-espresso-300">◎</span>
              </div>
              <p className="text-sm text-espresso-500">暂无日志</p>
            </div>
          )}
        </div>
      )}
    </div>
  );
}
