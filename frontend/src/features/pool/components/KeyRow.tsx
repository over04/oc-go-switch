import { CopyButton } from "@/shared/ui/CopyButton";
import { StatusBadge } from "./StatusBadge";
import type { KeyStatusEntry } from "@/shared/types/api";

interface KeyRowProps {
  keyEntry: KeyStatusEntry;
}

export function KeyRow({ keyEntry }: KeyRowProps) {
  return (
    <div className="flex items-center justify-between py-1.5 px-3 rounded-full bg-white border border-cream-100">
      <div className="flex items-center gap-2 min-w-0">
        <code className="text-xs text-espresso-600 font-mono truncate">
          {keyEntry.masked}
        </code>
        <CopyButton value={keyEntry.id} />
      </div>
      <StatusBadge status={keyEntry.status} />
    </div>
  );
}
