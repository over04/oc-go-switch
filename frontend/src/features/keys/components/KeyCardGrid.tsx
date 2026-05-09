import { useMemo, useState } from "react";
import { motion } from "framer-motion";
import clsx from "clsx";
import { Badge } from "@/shared/ui/Badge";
import { CopyButton } from "@/shared/ui/CopyButton";
import { Button } from "@/shared/ui/Button";
import { UsageBar } from "@/features/pool/components/UsageBar";
import type {
  AccountStatus,
  KeyStatusEntry,
  KeyStatus,
} from "@/shared/types/api";

interface KeyCardGridProps {
  accounts: AccountStatus[];
  currentKeyId: string | null;
  onSetActive: (keyId: string) => void;
  search: string;
  statusFilter: KeyStatus | "all";
}

const statusLabel: Record<KeyStatus, string> = {
  active: "活跃",
  depleted: "耗尽",
  idle: "空闲",
};
const statusDot: Record<KeyStatus, string> = {
  active: "bg-harvest-500",
  depleted: "bg-terra-400",
  idle: "bg-espresso-300",
};

function matchesSearch(key: KeyStatusEntry, workspaceName: string, search: string): boolean {
  if (!search) return true;
  const v = search.toLowerCase();
  return (
    key.masked.toLowerCase().includes(v) ||
    workspaceName.toLowerCase().includes(v)
  );
}

export function KeyCardGrid({
  accounts,
  currentKeyId,
  onSetActive,
  search,
  statusFilter,
}: KeyCardGridProps) {
  // Track which workspace cards are expanded
  const [expanded, setExpanded] = useState<Set<string>>(() => {
    // Default: expand workspaces that have the active key
    const s = new Set<string>();
    for (const acct of accounts) {
      for (const ws of acct.workspaces) {
        if (ws.keys.some((k) => k.id === currentKeyId)) {
          s.add(ws.id);
        }
      }
    }
    return s;
  });

  const toggle = (wsId: string) => {
    setExpanded((prev) => {
      const next = new Set(prev);
      if (next.has(wsId)) next.delete(wsId);
      else next.add(wsId);
      return next;
    });
  };

  return (
    <div className="space-y-5">
      {accounts.map((acct) => (
        <div key={acct.name}>
          {/* Account header */}
          <div className="flex items-center gap-2 mb-3">
            <span className="text-xs font-semibold text-espresso-500 uppercase tracking-wider">
              {acct.label}
            </span>
            <span className="text-xs text-espresso-300">·</span>
            <span className="text-xs text-espresso-400">{acct.workspaces.length} 工作区</span>
          </div>

          {/* Workspace cards */}
          <div className="space-y-3">
            {acct.workspaces.map((ws, wi) => {
              const filteredKeys = ws.keys.filter(
                (k) =>
                  matchesSearch(k, ws.name, search) &&
                  (statusFilter === "all" || k.status === statusFilter),
              );
              const isExpanded = expanded.has(ws.id);
              const hasActive = ws.keys.some((k) => k.id === currentKeyId);

              return (
                <motion.div
                  key={ws.id}
                  initial={{ opacity: 0, y: 8 }}
                  animate={{ opacity: 1, y: 0 }}
                  transition={{ delay: wi * 0.04, duration: 0.3 }}
                  className={clsx(
                    "bg-white rounded-mcm-lg border shadow-mcm overflow-hidden transition-shadow duration-200 hover:shadow-mcm-md",
                    hasActive
                      ? "border-terra-500/30"
                      : "border-cream-200",
                  )}
                >
                  {/* Workspace header — clickable to expand */}
                  <button
                    onClick={() => toggle(ws.id)}
                    className="w-full flex items-center justify-between px-5 py-3 hover:bg-cream-50/50 transition-colors text-left"
                  >
                    <div className="flex items-center gap-3">
                      <motion.span
                        animate={{ rotate: isExpanded ? 90 : 0 }}
                        transition={{ duration: 0.2 }}
                        className="text-xs text-espresso-400"
                      >
                        ▸
                      </motion.span>
                      <span className="text-sm font-medium text-espresso-700">
                        {ws.name}
                      </span>
                      <Badge size="xs" tone="go">Go</Badge>
                      {hasActive && (
                        <span className="text-xs text-terra-500 font-medium flex items-center gap-1">
                          <span className="w-1.5 h-1.5 rounded-full bg-terra-500" />
                          活跃
                        </span>
                      )}
                    </div>
                    <div className="flex items-center gap-3 text-xs text-espresso-400">
                      <span>{ws.keys.length} 密钥</span>
                      {ws.go_usage && (
                        <span className="tabular-nums">
                          {ws.go_usage.hourly_percent}% 小时
                        </span>
                      )}
                    </div>
                  </button>

                  {/* Expanded content: usage + keys */}
                  {isExpanded && (
                    <div className="px-5 pb-4 border-t border-cream-100">
                      {/* Usage bars */}
                      {ws.go_usage && (
                        <div className="py-3 space-y-1.5 bg-cream-50/50 rounded-mcm px-3 my-3 -mx-1">
                          <UsageBar
                            label="小时"
                            percent={ws.go_usage.hourly_percent}
                            resetSec={ws.go_usage.hourly_reset_sec}
                          />
                          <UsageBar
                            label="本周"
                            percent={ws.go_usage.weekly_percent}
                            resetSec={ws.go_usage.weekly_reset_sec}
                          />
                          <UsageBar
                            label="本月"
                            percent={ws.go_usage.monthly_percent}
                            resetSec={ws.go_usage.monthly_reset_sec}
                          />
                        </div>
                      )}

                      {/* Key chips */}
                      {filteredKeys.length === 0 ? (
                        <p className="text-xs text-espresso-300 text-center py-4">
                          无匹配密钥
                        </p>
                      ) : (
                        <div className="flex flex-wrap gap-2">
                          {filteredKeys.map((k) => {
                            const isActive = k.id === currentKeyId;
                            return (
                              <motion.div
                                key={k.id}
                                initial={{ opacity: 0, scale: 0.9 }}
                                animate={{ opacity: 1, scale: 1 }}
                                whileHover={{ y: -1 }}
                                className={clsx(
                                  "flex items-center gap-2 px-3 py-1.5 rounded-full border transition-all duration-200",
                                  isActive
                                    ? "bg-terra-500/5 border-terra-500/30 ring-1 ring-terra-500/10"
                                    : "bg-cream-50 border-cream-200 hover:border-cream-300",
                                )}
                              >
                                <span
                                  className={clsx(
                                    "w-1.5 h-1.5 rounded-full",
                                    statusDot[k.status],
                                  )}
                                />
                                <code className="text-xs font-mono text-espresso-600 truncate max-w-[140px]">
                                  {k.masked}
                                </code>
                                <CopyButton value={k.id} />
                                <span className="text-[0.625rem] text-espresso-400">
                                  {statusLabel[k.status]}
                                </span>
                                {!isActive && k.status !== "depleted" && (
                                  <Button
                                    size="xs"
                                    tone="primary"
                                    onClick={(e) => {
                                      e.stopPropagation();
                                      onSetActive(k.id);
                                    }}
                                  >
                                    启用
                                  </Button>
                                )}
                                {isActive && (
                                  <span className="text-xs text-terra-500 font-semibold">
                                    ◈
                                  </span>
                                )}
                              </motion.div>
                            );
                          })}
                        </div>
                      )}
                    </div>
                  )}
                </motion.div>
              );
            })}
          </div>
        </div>
      ))}
    </div>
  );
}
