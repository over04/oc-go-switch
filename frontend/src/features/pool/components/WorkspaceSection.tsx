import { Badge } from "@/shared/ui/Badge";
import { UsageBar } from "./UsageBar";
import type { WorkspaceStatus } from "@/shared/types/api";

interface WorkspaceSectionProps {
  workspace: WorkspaceStatus;
}

export function WorkspaceSection({ workspace }: WorkspaceSectionProps) {
  return (
    <div className="bg-cream-50/50 rounded-mcm px-4 py-3 border border-cream-100">
      <div className="flex items-center gap-2 mb-2.5">
        <span className="text-xs font-semibold text-espresso-700">
          {workspace.name}
        </span>
        <Badge size="xs" tone={workspace.status === "unsubscribed" ? "default" : "go"}>
          {workspace.status === "unsubscribed" ? "无订阅" : "Go"}
        </Badge>
        <Badge size="xs" tone={workspace.status === "available" ? "success" : workspace.status === "exhausted" ? "danger" : "default"}>
          {workspace.status === "available"
            ? workspace.is_affinity
              ? "亲和中"
              : workspace.is_current
                ? "最近使用"
              : `可用 #${workspace.queue_position ?? "-"}`
            : workspace.status === "exhausted"
              ? "当前无额度"
              : "无 Go 订阅"}
        </Badge>
      </div>

      {workspace.go_usage && (
        <div className="space-y-1.5 mb-2.5">
          <UsageBar
            label="小时"
            percent={workspace.go_usage.hourly_percent}
            resetSec={workspace.go_usage.hourly_reset_sec}
          />
          <UsageBar
            label="本周"
            percent={workspace.go_usage.weekly_percent}
            resetSec={workspace.go_usage.weekly_reset_sec}
          />
          <UsageBar
            label="本月"
            percent={workspace.go_usage.monthly_percent}
            resetSec={workspace.go_usage.monthly_reset_sec}
          />
        </div>
      )}

      <div className="flex items-center justify-between py-1.5 px-3 rounded-full bg-white border border-cream-100">
        <span className="text-xs text-espresso-400">凭证</span>
        <code className="text-xs text-espresso-600 font-mono truncate">
          {workspace.credential_masked}
        </code>
      </div>
    </div>
  );
}
