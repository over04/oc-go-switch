import { Input } from "@/shared/ui/Input";
import { Button } from "@/shared/ui/Button";
import type { KeyStatus } from "@/shared/types/api";

interface KeyFilterBarProps {
  search: string;
  onSearchChange: (v: string) => void;
  statusFilter: KeyStatus | "all";
  onStatusFilterChange: (v: KeyStatus | "all") => void;
}

const statusOptions: { value: KeyStatus | "all"; label: string }[] = [
  { value: "all", label: "全部" },
  { value: "active", label: "活跃" },
  { value: "idle", label: "空闲" },
  { value: "depleted", label: "耗尽" },
];

export function KeyFilterBar({
  search,
  onSearchChange,
  statusFilter,
  onStatusFilterChange,
}: KeyFilterBarProps) {
  return (
    <div className="flex items-center gap-3">
      <Input
        placeholder="搜索密钥或工作区..."
        value={search}
        onChange={(e) => onSearchChange(e.target.value)}
        className="max-w-[240px]"
      />
      <div className="flex items-center gap-1 bg-cream-100 rounded-full p-0.5">
        {statusOptions.map((opt) => (
          <button
            key={opt.value}
            onClick={() => onStatusFilterChange(opt.value)}
            className={`px-3 py-1 text-xs rounded-full font-medium transition-all duration-200 ${
              statusFilter === opt.value
                ? "bg-white text-espresso-700 shadow-sm"
                : "text-espresso-400 hover:text-espresso-600"
            }`}
          >
            {opt.label}
          </button>
        ))}
      </div>
    </div>
  );
}
