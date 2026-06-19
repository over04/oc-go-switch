import { Input } from "@/shared/ui/Input";
import type { WorkspaceStatusKind } from "@/shared/types/api";

interface WorkspaceFilterBarProps {
  search: string;
  onSearchChange: (v: string) => void;
  workspaceFilter: WorkspaceStatusKind | "all";
  onWorkspaceFilterChange: (v: WorkspaceStatusKind | "all") => void;
}

const workspaceOptions: { value: WorkspaceStatusKind | "all"; label: string }[] = [
  { value: "all", label: "全部工作区" },
  { value: "available", label: "可用" },
  { value: "exhausted", label: "当前无额度" },
  { value: "unsubscribed", label: "无订阅" },
];

export function WorkspaceFilterBar({
  search,
  onSearchChange,
  workspaceFilter,
  onWorkspaceFilterChange,
}: WorkspaceFilterBarProps) {
  return (
    <div className="flex flex-col sm:flex-row sm:items-center gap-2 sm:gap-3">
      <Input
        placeholder="搜索工作区或凭证..."
        value={search}
        onChange={(e) => onSearchChange(e.target.value)}
        className="max-w-full sm:max-w-[240px]"
      />
      <div className="flex items-center gap-1 bg-cream-100 rounded-full p-0.5 overflow-x-auto">
        {workspaceOptions.map((opt) => (
          <button
            key={opt.value}
            onClick={() => onWorkspaceFilterChange(opt.value)}
            className={`px-3 py-1 text-xs rounded-full font-medium transition-all duration-200 whitespace-nowrap ${
              workspaceFilter === opt.value
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
