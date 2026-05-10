import { useState, useEffect, useRef, useMemo } from "react";
import { useQuery } from "@tanstack/react-query";
import { motion, AnimatePresence } from "framer-motion";
import clsx from "clsx";
import { Input } from "@/shared/ui/Input";
import { Select } from "@/shared/ui/Select";
import { Button } from "@/shared/ui/Button";
import { getOpenaiModels, getClaudeModels } from "@/features/models/service/modelsService";
import type { ImageFilterModel, FilterAction, ModelListResult } from "@/shared/types/api";

const ACTION_OPTIONS: { value: FilterAction; label: string }[] = [
  { value: "pass_through", label: "透传（不处理）" },
  { value: "remove", label: "移除图片" },
  { value: "replace", label: "替换为文本" },
];

function isError(r: ModelListResult): r is { error: string } {
  return "error" in r;
}

interface ImageFilterFormProps {
  models: ImageFilterModel[];
  onChange: (models: ImageFilterModel[]) => void;
}

export function ImageFilterForm({ models, onChange }: ImageFilterFormProps) {
  // 从两个 provider 获取可用模型列表
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

  const availableModels = useMemo(() => {
    const ids = new Set<string>();
    const add = (r: ModelListResult | undefined) => {
      if (r && !isError(r)) {
        r.data.forEach((m) => ids.add(m.id));
      }
    };
    add(openaiQuery.data);
    add(claudeQuery.data);
    return Array.from(ids).sort();
  }, [openaiQuery.data, claudeQuery.data]);

  function addRule() {
    onChange([
      ...models,
      { model: "", action: "pass_through", replacement: null },
    ]);
  }

  function removeRule(idx: number) {
    onChange(models.filter((_, i) => i !== idx));
  }

  function updateRule(idx: number, patch: Partial<ImageFilterModel>) {
    onChange(models.map((m, i) => (i === idx ? { ...m, ...patch } : m)));
  }

  return (
    <div className="space-y-3">
      <div className="flex items-center justify-between">
        <p className="text-xs text-espresso-400">
          为不支持图片的模型配置自动处理策略
        </p>
        <Button size="xs" tone="primary" onClick={addRule}>
          添加规则
        </Button>
      </div>

      {models.length === 0 && (
        <p className="text-xs text-espresso-300 py-2">暂无过滤规则</p>
      )}

      {models.map((rule, idx) => (
        <div
          key={idx}
          className="grid grid-cols-1 sm:grid-cols-[1fr_120px_1fr_40px] gap-2 items-start p-3 bg-cream-50/50 rounded-mcm border border-cream-100"
        >
          <ModelPicker
            value={rule.model}
            suggestions={availableModels}
            onChange={(v) => updateRule(idx, { model: v })}
          />

          <div>
            <label className="block text-[11px] font-medium text-espresso-400 mb-1">
              处理行为
            </label>
            <Select
              size="xs"
              value={rule.action}
              onChange={(v) =>
                updateRule(idx, { action: v as FilterAction })
              }
              options={ACTION_OPTIONS}
            />
          </div>

          <div>
            <label className="block text-[11px] font-medium text-espresso-400 mb-1">
              {rule.action === "replace" ? "替换文本" : "替换文本"}
            </label>
            <Input
              size="xs"
              placeholder={
                rule.action === "replace" ? "[Image removed]" : "—"
              }
              disabled={rule.action !== "replace"}
              value={
                rule.action === "replace"
                  ? (rule.replacement ?? "")
                  : ""
              }
              onChange={(e) =>
                updateRule(idx, {
                  replacement: e.target.value || null,
                })
              }
            />
          </div>

          <div className="pt-[22px]">
            <Button
              size="xs"
              tone="danger"
              onClick={() => removeRule(idx)}
            >
              删除
            </Button>
          </div>
        </div>
      ))}
    </div>
  );
}

/** Model picker: free-text input with dropdown suggestions from API. */
function ModelPicker({
  value,
  suggestions,
  onChange,
}: {
  value: string;
  suggestions: string[];
  onChange: (v: string) => void;
}) {
  const [open, setOpen] = useState(false);
  const [focusIdx, setFocusIdx] = useState(-1);
  const ref = useRef<HTMLDivElement>(null);

  const filtered = useMemo(() => {
    if (!value) return suggestions.slice(0, 50);
    const lower = value.toLowerCase();
    return suggestions.filter((s) => s.toLowerCase().includes(lower)).slice(0, 50);
  }, [suggestions, value]);

  useEffect(() => {
    if (!open) return;
    const handler = (e: MouseEvent) => {
      if (ref.current && !ref.current.contains(e.target as Node)) setOpen(false);
    };
    document.addEventListener("mousedown", handler);
    return () => document.removeEventListener("mousedown", handler);
  }, [open]);

  function handleKey(e: React.KeyboardEvent) {
    switch (e.key) {
      case "ArrowDown":
        e.preventDefault();
        if (!open) setOpen(true);
        setFocusIdx((i) => Math.min(i + 1, filtered.length - 1));
        break;
      case "ArrowUp":
        e.preventDefault();
        setFocusIdx((i) => Math.max(i - 1, 0));
        break;
      case "Enter":
        e.preventDefault();
        if (open && focusIdx >= 0 && focusIdx < filtered.length) {
          onChange(filtered[focusIdx]!);
          setOpen(false);
        }
        break;
      case "Escape":
        setOpen(false);
        break;
    }
  }

  return (
    <div ref={ref} className="relative" onKeyDown={handleKey}>
      <label className="block text-[11px] font-medium text-espresso-400 mb-1">
        模型 ID
      </label>
      <Input
        size="xs"
        placeholder="搜索或输入模型 ID..."
        value={value}
        onChange={(e) => {
          onChange(e.target.value);
          setOpen(true);
          setFocusIdx(-1);
        }}
        onFocus={() => setOpen(true)}
      />

      <AnimatePresence>
        {open && (
          <motion.div
            initial={{ opacity: 0, y: -4, scale: 0.96 }}
            animate={{ opacity: 1, y: 0, scale: 1 }}
            exit={{ opacity: 0, y: -4, scale: 0.96 }}
            transition={{ duration: 0.15 }}
            className={clsx(
              "absolute z-50 mt-1 w-full bg-white rounded-[10px] border border-cream-200",
              "shadow-mcm-md overflow-hidden max-h-48 overflow-y-auto py-1",
            )}
          >
            {filtered.length === 0 ? (
              <p className="px-3 py-2 text-xs text-espresso-300">
                {suggestions.length === 0 ? "加载模型列表中..." : "无匹配模型"}
              </p>
            ) : (
              filtered.map((id, i) => (
                <button
                  key={id}
                  type="button"
                  onClick={() => {
                    onChange(id);
                    setOpen(false);
                  }}
                  onMouseEnter={() => setFocusIdx(i)}
                  className={clsx(
                    "w-full text-left px-3 py-1.5 text-xs font-mono transition-colors",
                    focusIdx === i
                      ? "bg-cream-100 text-espresso-700"
                      : "text-espresso-600 hover:bg-cream-50",
                  )}
                >
                  {id}
                </button>
              ))
            )}
          </motion.div>
        )}
      </AnimatePresence>
    </div>
  );
}
