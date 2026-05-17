import { Badge } from "@/shared/ui/Badge";
import type { KeyStatus } from "@/shared/types/api";

const toneMap: Record<KeyStatus, "success" | "default"> = {
  active: "success",
  idle: "default",
};

const labelMap: Record<KeyStatus, string> = {
  active: "活跃",
  idle: "空闲",
};

interface StatusBadgeProps {
  status: KeyStatus;
}

export function StatusBadge({ status }: StatusBadgeProps) {
  return <Badge tone={toneMap[status]}>{labelMap[status]}</Badge>;
}
