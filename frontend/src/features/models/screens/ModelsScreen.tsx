import { useState, useMemo } from "react";
import { useQuery } from "@tanstack/react-query";
import { motion, AnimatePresence } from "framer-motion";
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

  const openaiQuery = useQuery({
    queryKey: ["models", "openai"],
    queryFn: getOpenaiModels,
    staleTime: 300_000,
  });
  const claudeQuery = useQuery({
    queryKey: ["models", "claude"],
    queryFn: getClaudeModels,
    staleTime: 300_000,
  });

  const activeQuery = tab === "openai" ? openaiQuery : claudeQuery;
  const result = activeQuery.data;

  const filtered = useMemo(() => {
    if (!result || isError(result)) return [];
    return result.data.filter((m) =>
      m.id.toLowerCase().includes(search.toLowerCase()),
    );
  }, [result, search]);

  const errMsg: string | null = activeQuery.isError
    ? activeQuery.error instanceof Error
      ? activeQuery.error.message
      : "请求失败"
    : result && isError(result)
      ? result.error
      : null;

  const providerLabel = tab === "openai" ? "OpenAI 协议" : "Claude 协议";
  const providerIcon = tab === "openai" ? "◈" : "◆";

  return (
    <div className="max-w-5xl space-y-5">
      {/* Header */}
      <div className="flex items-center gap-3">
        <div className="w-1 h-6 bg-sky-400 rounded-full" />
        <h2 className="text-lg font-semibold text-espresso-700 tracking-tight">
          模型列表
        </h2>
      </div>

      {/* Tab switcher — pill-style */}
      <div className="flex items-center gap-2">
        <div className="flex items-center bg-cream-100 rounded-full p-0.5">
          <button
            onClick={() => setTab("openai")}
            className={`px-4 py-1.5 text-sm rounded-full font-medium transition-all duration-200 ${
              tab === "openai"
                ? "bg-white text-espresso-700 shadow-sm"
                : "text-espresso-400 hover:text-espresso-600"
            }`}
          >
            OpenAI
          </button>
          <button
            onClick={() => setTab("claude")}
            className={`px-4 py-1.5 text-sm rounded-full font-medium transition-all duration-200 ${
              tab === "claude"
                ? "bg-white text-espresso-700 shadow-sm"
                : "text-espresso-400 hover:text-espresso-600"
            }`}
          >
            Claude
          </button>
        </div>
      </div>

      {/* Search */}
      <Input
        placeholder="搜索模型 ID..."
        value={search}
        onChange={(e) => setSearch(e.target.value)}
        className="max-w-[280px]"
      />

      {/* Content */}
      {activeQuery.isPending ? (
        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3">
          {[1, 2, 3, 4, 5, 6].map((i) => (
            <Skeleton key={i} className="h-20" />
          ))}
        </div>
      ) : errMsg ? (
        <motion.div
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          className="bg-terra-400/5 border border-terra-400/15 rounded-mcm-lg px-5 py-4"
        >
          <p className="text-sm font-medium text-terra-400">
            上游模型列表不可用
          </p>
          <p className="text-xs text-terra-400/70 mt-1 font-mono">{errMsg}</p>
        </motion.div>
      ) : (
        <>
          <p className="text-xs text-espresso-400">
            {providerIcon} {providerLabel} · {filtered.length} 个模型
          </p>

          <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3">
            <AnimatePresence>
              {filtered.map((m, i) => (
                <motion.div
                  key={m.id}
                  initial={{ opacity: 0, y: 8 }}
                  animate={{ opacity: 1, y: 0 }}
                  transition={{ delay: i * 0.02, duration: 0.25 }}
                  whileHover={{ y: -2 }}
                  className="bg-white rounded-mcm-lg border border-cream-200 shadow-mcm px-4 py-3.5 transition-shadow duration-200 hover:shadow-mcm-md"
                >
                  <div className="flex items-start justify-between">
                    <div className="min-w-0 flex-1">
                      <p className="text-sm font-medium text-espresso-700 font-mono truncate">
                        {m.id}
                      </p>
                      <p className="text-xs text-espresso-400 mt-1">
                        {m.owned_by}
                      </p>
                    </div>
                    <span className="text-lg text-cream-300 shrink-0 ml-2">
                      {providerIcon}
                    </span>
                  </div>
                </motion.div>
              ))}
            </AnimatePresence>
          </div>

          {filtered.length === 0 && (
            <motion.div
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              className="text-center py-12"
            >
              <div className="w-14 h-14 mx-auto rounded-full bg-cream-100 flex items-center justify-center mb-3">
                <span className="text-xl text-espresso-300">◇</span>
              </div>
              <p className="text-sm text-espresso-500">无匹配模型</p>
            </motion.div>
          )}
        </>
      )}
    </div>
  );
}
