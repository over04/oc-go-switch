import { Input } from "@/shared/ui/Input";
import { Select } from "@/shared/ui/Select";
import type { KeyStatus } from "@/shared/types/api";

interface KeyTableToolbarProps {
  search: string;
  onSearchChange: (v: string) => void;
  statusFilter: KeyStatus | "all";
  onStatusFilterChange: (v: KeyStatus | "all") => void;
  goFilter: "all" | "go" | "nongo";
  onGoFilterChange: (v: "all" | "go" | "nongo") => void;
}

const STATUS_OPTIONS = [
  { value: "all", label: "全部状态" },
  { value: "active", label: "活跃" },
  { value: "idle", label: "空闲" },
  { value: "depleted", label: "耗尽" },
];

const GO_OPTIONS = [
  { value: "all", label: "全部类型" },
  { value: "go", label: "Go 订阅" },
  { value: "nongo", label: "非 Go" },
];

export function KeyTableToolbar({
  search,
  onSearchChange,
  statusFilter,
  onStatusFilterChange,
  goFilter,
  onGoFilterChange,
}: KeyTableToolbarProps) {
  return (
    <div className="flex items-center gap-2 mb-3">
      <Input
        placeholder="搜索密钥或工作区..."
        value={search}
        onChange={(e) => onSearchChange(e.target.value)}
        className="max-w-[200px]"
      />
      <Select
        value={statusFilter}
        onChange={(v) => onStatusFilterChange(v as KeyStatus | "all")}
        options={STATUS_OPTIONS}
      />
      <Select
        value={goFilter}
        onChange={(v) => onGoFilterChange(v as "all" | "go" | "nongo")}
        options={GO_OPTIONS}
      />
    </div>
  );
}
