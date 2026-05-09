import { useState, useMemo } from "react";
import { useQuery } from "@tanstack/react-query";
import { motion } from "framer-motion";
import { Button } from "@/shared/ui/Button";
import { Input } from "@/shared/ui/Input";
import { Skeleton } from "@/shared/ui/Skeleton";
import { getOpenaiModels, getClaudeModels } from "../service/modelsService";
import type { ModelListResult } from "@/shared/types/api";

type Tab = "openai" | "claude";

function isError(r: ModelListResult): r is { error: string } {
  return "error" in r;
}

export function ModelsScreen() {
  const [tab, setTab] = useState<Tab>("openai");
  const [search, setSearch] = useState("");

  const openaiQuery = useQuery({ queryKey: ["models", "openai"], queryFn: getOpenaiModels, staleTime: 300_000 });
  const claudeQuery = useQuery({ queryKey: ["models", "claude"], queryFn: getClaudeModels, staleTime: 300_000 });

  const activeQuery = tab === "openai" ? openaiQuery : claudeQuery;
  const result = activeQuery.data;

  const filtered = useMemo(() => {
    if (!result || isError(result)) return [];
    return result.data.filter((m) => m.id.toLowerCase().includes(search.toLowerCase()));
  }, [result, search]);

  const errMsg: string | null =
    activeQuery.isError ? (activeQuery.error instanceof Error ? activeQuery.error.message : "请求失败")
    : result && isError(result) ? result.error
    : null;

  return (
    <div>
      <h2 className="text-sm font-semibold mb-3">模型列表</h2>

      <div className="flex items-center gap-2 mb-3">
        <Button size="xs" tone={tab === "openai" ? "primary" : "default"} onClick={() => setTab("openai")}>OpenAI 协议</Button>
        <Button size="xs" tone={tab === "claude" ? "primary" : "default"} onClick={() => setTab("claude")}>Claude 协议</Button>
      </div>

      <Input placeholder="搜索模型 ID..." value={search} onChange={(e) => setSearch(e.target.value)} className="max-w-[240px] mb-3" />

      {activeQuery.isPending ? <Skeleton className="h-64" /> : errMsg ? (
        <motion.div initial={{ opacity: 0 }} animate={{ opacity: 1 }}
          className="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg px-4 py-3">
          <p className="text-xs text-red-600 dark:text-red-400 font-medium">上游模型列表不可用</p>
          <p className="text-xs text-red-500 dark:text-red-400 mt-1 font-mono">{errMsg}</p>
        </motion.div>
      ) : (
        <div className="bg-white dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700 overflow-hidden">
          <table className="w-full">
            <thead>
              <tr className="border-b border-gray-100 dark:border-gray-700">
                <th className="px-3 py-2 text-left text-xs font-medium text-gray-400">模型 ID</th>
                <th className="px-3 py-2 text-left text-xs font-medium text-gray-400">提供商</th>
              </tr>
            </thead>
            <tbody>
              {filtered.map((m, i) => (
                <motion.tr key={m.id} initial={{ opacity: 0, y: -4 }} animate={{ opacity: 1, y: 0 }} transition={{ delay: i * 0.02 }}
                  className="border-b border-gray-50 dark:border-gray-800/50 last:border-0">
                  <td className="px-3 py-2 text-xs font-mono text-gray-700 dark:text-gray-200">{m.id}</td>
                  <td className="px-3 py-2 text-xs text-gray-500">{m.owned_by}</td>
                </motion.tr>
              ))}
            </tbody>
          </table>
          {filtered.length === 0 && <p className="text-xs text-gray-400 text-center py-8">无匹配结果</p>}
        </div>
      )}
    </div>
  );
}
