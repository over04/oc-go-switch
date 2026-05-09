import { CopyButton } from "@/shared/ui/CopyButton";
import { StatusBadge } from "./StatusBadge";
import type { KeyStatusEntry } from "@/shared/types/api";

interface KeyRowProps {
  keyEntry: KeyStatusEntry;
}

export function KeyRow({ keyEntry }: KeyRowProps) {
  return (
    <div className="flex items-center justify-between py-1 px-2 rounded bg-gray-50 dark:bg-gray-800/50">
      <div className="flex items-center gap-1.5 min-w-0">
        <code className="text-2xs text-gray-500 dark:text-gray-400 font-mono truncate">
          {keyEntry.masked}
        </code>
        <CopyButton value={keyEntry.id} />
      </div>
      <StatusBadge status={keyEntry.status} />
    </div>
  );
}
