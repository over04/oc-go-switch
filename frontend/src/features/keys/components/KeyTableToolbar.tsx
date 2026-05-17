import { Input } from "@/shared/ui/Input";
import { Select } from "@/shared/ui/Select";
import type { KeyStatus } from "@/shared/types/api";

interface KeyTableToolbarProps {
  search: string;
  onSearchChange: (v: string) => void;
  statusFilter: KeyStatus | "all";
  onStatusFilterChange: (v: KeyStatus | "all") => void;
}

const STATUS_OPTIONS = [
  { value: "all", label: "全部状态" },
  { value: "active", label: "活跃" },
  { value: "idle", label: "空闲" },
];

export function KeyTableToolbar({
  search,
  onSearchChange,
  statusFilter,
  onStatusFilterChange,
}: KeyTableToolbarProps) {
  return (
    <div className="flex items-center gap-2 mb-3">
      <Input
        placeholder="搜索 Key 或工作区..."
        value={search}
        onChange={(e) => onSearchChange(e.target.value)}
        className="max-w-[200px]"
      />
      <Select
        value={statusFilter}
        onChange={(v) => onStatusFilterChange(v as KeyStatus | "all")}
        options={STATUS_OPTIONS}
      />
    </div>
  );
}
