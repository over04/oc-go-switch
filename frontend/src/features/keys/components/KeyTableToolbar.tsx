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
        onChange={(e) => onStatusFilterChange(e.target.value as KeyStatus | "all")}
      >
        <option value="all">全部状态</option>
        <option value="active">活跃</option>
        <option value="idle">空闲</option>
        <option value="depleted">耗尽</option>
      </Select>
      <Select
        value={goFilter}
        onChange={(e) => onGoFilterChange(e.target.value as "all" | "go" | "nongo")}
      >
        <option value="all">全部类型</option>
        <option value="go">Go 订阅</option>
        <option value="nongo">非 Go</option>
      </Select>
    </div>
  );
}
