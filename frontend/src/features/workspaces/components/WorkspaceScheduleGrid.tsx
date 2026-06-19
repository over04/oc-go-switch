import { useState } from "react";
import { motion } from "framer-motion";
import clsx from "clsx";
import { Badge } from "@/shared/ui/Badge";
import { Button } from "@/shared/ui/Button";
import { UsageBar } from "@/features/pool/components/UsageBar";
import type {
  AccountStatus,
  WorkspaceStatusKind,
} from "@/shared/types/api";

interface WorkspaceScheduleGridProps {
  accounts: AccountStatus[];
  affinityWorkspaceId: string | null;
  onSetAffinity: (workspaceId: string) => void;
  search: string;
  workspaceFilter: WorkspaceStatusKind | "all";
}

const workspaceLabel = {
  available: "可用",
  exhausted: "当前无额度",
  unsubscribed: "无 Go 订阅",
} as const;

const workspaceTone = {
  available: "success",
  exhausted: "danger",
  unsubscribed: "default",
} as const;

function matchesSearch(
  workspaceName: string,
  credentialMasked: string,
  search: string,
): boolean {
  if (!search) return true;
  const v = search.toLowerCase();
  return (
    workspaceName.toLowerCase().includes(v) ||
    credentialMasked.toLowerCase().includes(v)
  );
}

export function WorkspaceScheduleGrid({
  accounts,
  affinityWorkspaceId,
  onSetAffinity,
  search,
  workspaceFilter,
}: WorkspaceScheduleGridProps) {
  const [expanded, setExpanded] = useState<Set<string>>(() => {
    const s = new Set<string>();
    for (const acct of accounts) {
      for (const ws of acct.workspaces) {
        if (ws.id === affinityWorkspaceId || ws.is_current) {
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
          <div className="flex items-center gap-2 mb-3">
            <span className="text-xs font-semibold text-espresso-500 uppercase tracking-wider">
              {acct.label}
            </span>
            <span className="text-xs text-espresso-300">·</span>
            <span className="text-xs text-espresso-400">{acct.workspaces.length} 工作区</span>
          </div>

          <div className="space-y-3">
            {acct.workspaces.map((ws, wi) => {
              if (workspaceFilter !== "all" && ws.status !== workspaceFilter) {
                return null;
              }
              if (!matchesSearch(ws.name, ws.credential_masked, search)) {
                return null;
              }

              const isExpanded = expanded.has(ws.id);
              const hasAffinity = ws.id === affinityWorkspaceId;
              const canSchedule = ws.status === "available";

              return (
                <motion.div
                  key={ws.id}
                  initial={{ opacity: 0, y: 8 }}
                  animate={{ opacity: 1, y: 0 }}
                  transition={{ delay: wi * 0.04, duration: 0.3 }}
                  className={clsx(
                    "bg-white rounded-mcm-lg border shadow-mcm overflow-hidden transition-shadow duration-200 hover:shadow-mcm-md",
                    hasAffinity
                      ? "border-terra-500/30"
                      : "border-cream-200",
                  )}
                >
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
                      <Badge size="xs" tone={ws.status === "unsubscribed" ? "default" : "go"}>
                        {ws.status === "unsubscribed" ? "无订阅" : "Go"}
                      </Badge>
                      <Badge size="xs" tone={workspaceTone[ws.status]}>
                        {ws.status === "available" && ws.is_affinity
                          ? "亲和中"
                          : ws.status === "available" && ws.is_current
                            ? "最近使用"
                            : ws.status === "available"
                              ? `可用 #${ws.queue_position ?? "-"}`
                              : workspaceLabel[ws.status]}
                      </Badge>
                    </div>
                    <div className="flex items-center gap-3 text-xs text-espresso-400">
                      <code>{ws.credential_masked}</code>
                      {ws.go_usage && (
                        <span className="tabular-nums">
                          {ws.go_usage.hourly_percent}% 小时
                        </span>
                      )}
                    </div>
                  </button>

                  {isExpanded && (
                    <div className="px-5 pb-4 border-t border-cream-100">
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

                      <div className="flex flex-wrap items-center gap-2">
                        <div className="flex items-center gap-2 px-3 py-1.5 rounded-full bg-cream-50 border border-cream-200">
                          <span className="text-[0.625rem] text-espresso-400">
                            凭证
                          </span>
                          <code className="text-xs font-mono text-espresso-600">
                            {ws.credential_masked}
                          </code>
                        </div>

                        {!hasAffinity && canSchedule && (
                          <Button
                            size="xs"
                            tone="primary"
                            onClick={(e) => {
                              e.stopPropagation();
                              onSetAffinity(ws.id);
                            }}
                          >
                            设为亲和
                          </Button>
                        )}
                        {!hasAffinity && !canSchedule && (
                          <span className="text-[0.625rem] text-espresso-300">
                            不参与调度
                          </span>
                        )}
                        {hasAffinity && (
                          <span className="text-xs text-terra-500 font-semibold">
                            亲和工作区
                          </span>
                        )}
                      </div>
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
