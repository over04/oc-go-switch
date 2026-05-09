import { Badge } from "@/shared/ui/Badge";
import type { KeyStatus } from "@/shared/types/api";

const toneMap: Record<KeyStatus, "success" | "danger" | "default"> = {
  active: "success",
  depleted: "danger",
  idle: "default",
};

const labelMap: Record<KeyStatus, string> = {
  active: "活跃",
  depleted: "耗尽",
  idle: "空闲",
};

interface StatusBadgeProps {
  status: KeyStatus;
}

export function StatusBadge({ status }: StatusBadgeProps) {
  return <Badge tone={toneMap[status]}>{labelMap[status]}</Badge>;
}
