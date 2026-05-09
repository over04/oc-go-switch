import { Badge } from "@/shared/ui/Badge";
import { UsageBar } from "./UsageBar";
import { KeyRow } from "./KeyRow";
import type { WorkspaceStatus } from "@/shared/types/api";

interface WorkspaceSectionProps {
  workspace: WorkspaceStatus;
}

export function WorkspaceSection({ workspace }: WorkspaceSectionProps) {
  return (
    <div className="mb-2.5 last:mb-0">
      <div className="flex items-center gap-1.5 mb-1.5">
        <span className="text-2xs font-medium text-gray-700 dark:text-gray-200">
          {workspace.name}
        </span>
        {workspace.subscribed ? (
          <Badge size="xs" tone="go">Go</Badge>
        ) : (
          <Badge size="xs" tone="default">-</Badge>
        )}
      </div>
      {workspace.go_usage && (
        <div className="space-y-0.5 mb-1.5 px-1">
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
      <div className="space-y-px">
        {workspace.keys.map((k) => (
          <KeyRow key={k.id} keyEntry={k} />
        ))}
      </div>
    </div>
  );
}
