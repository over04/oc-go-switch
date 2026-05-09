import { useMemo, useState } from "react";
import {
  useReactTable,
  getCoreRowModel,
  getSortedRowModel,
  getFilteredRowModel,
  createColumnHelper,
  type SortingState,
} from "@tanstack/react-table";
import { motion, AnimatePresence } from "framer-motion";
import { Badge } from "@/shared/ui/Badge";
import { CopyButton } from "@/shared/ui/CopyButton";
import { Button } from "@/shared/ui/Button";
import { KeyTableToolbar } from "./KeyTableToolbar";
import type { KeyStatusEntry, WorkspaceStatus, AccountStatus, KeyStatus } from "@/shared/types/api";

interface FlatKeyRow {
  key: KeyStatusEntry;
  accountName: string;
  accountLabel: string;
  workspaceName: string;
  workspaceId: string;
  subscribed: boolean;
  hourlyPercent: number | null;
  weeklyPercent: number | null;
  monthlyPercent: number | null;
}

function flattenKeys(accounts: AccountStatus[]): FlatKeyRow[] {
  const rows: FlatKeyRow[] = [];
  for (const acct of accounts) {
    for (const ws of acct.workspaces) {
      const u = ws.go_usage;
      for (const k of ws.keys) {
        rows.push({
          key: k,
          accountName: acct.name,
          accountLabel: acct.label,
          workspaceName: ws.name,
          workspaceId: ws.id,
          subscribed: ws.subscribed,
          hourlyPercent: u?.hourly_percent ?? null,
          weeklyPercent: u?.weekly_percent ?? null,
          monthlyPercent: u?.monthly_percent ?? null,
        });
      }
    }
  }
  return rows;
}

const columnHelper = createColumnHelper<FlatKeyRow>();

const statusLabel: Record<KeyStatus, string> = { active: "活跃", depleted: "耗尽", idle: "空闲" };
const statusTone: Record<KeyStatus, "success" | "danger" | "default"> = { active: "success", depleted: "danger", idle: "default" };

interface KeyTableProps {
  accounts: AccountStatus[];
  currentKeyId: string | null;
  onSetActive: (keyId: string) => void;
}

export function KeyTable({ accounts, currentKeyId, onSetActive }: KeyTableProps) {
  const data = useMemo(() => flattenKeys(accounts), [accounts]);
  const [sorting, setSorting] = useState<SortingState>([]);
  const [search, setSearch] = useState("");
  const [statusFilter, setStatusFilter] = useState<KeyStatus | "all">("all");
  const [goFilter, setGoFilter] = useState<"all" | "go" | "nongo">("all");

  const columns = useMemo(
    () => [
      columnHelper.accessor("key.masked", {
        header: "密钥",
        cell: (info) => (
          <div className="flex items-center gap-1.5 min-w-0">
            <code className="text-2xs font-mono text-gray-500 dark:text-gray-400 truncate">
              {info.getValue()}
            </code>
            <CopyButton value={info.row.original.key.id} />
          </div>
        ),
        size: 220,
      }),
      columnHelper.accessor("accountLabel", {
        header: "账户",
        cell: (info) => (
          <span className="text-2xs text-gray-600 dark:text-gray-300">{info.getValue()}</span>
        ),
        size: 80,
      }),
      columnHelper.accessor("workspaceName", {
        header: "工作区",
        cell: (info) => (
          <div className="flex items-center gap-1">
            <span className="text-2xs">{info.getValue()}</span>
            {info.row.original.subscribed && <Badge size="xs" tone="go">Go</Badge>}
          </div>
        ),
        size: 100,
      }),
      columnHelper.accessor("hourlyPercent", {
        header: "用量",
        cell: (info) => {
          const row = info.row.original;
          if (row.hourlyPercent === null) {
            return <span className="text-2xs text-gray-400">-</span>;
          }
          return (
            <span className="text-2xs font-mono tabular-nums text-gray-600 dark:text-gray-300">
              {row.hourlyPercent}%/{row.weeklyPercent}%/{row.monthlyPercent}%
            </span>
          );
        },
        size: 100,
      }),
      columnHelper.accessor("key.status", {
        header: "状态",
        cell: (info) => (
          <Badge size="xs" tone={statusTone[info.getValue()]}>{statusLabel[info.getValue()]}</Badge>
        ),
        size: 60,
      }),
      columnHelper.display({
        id: "actions",
        header: "",
        cell: (info) => {
          const keyId = info.row.original.key.id;
          const isCurrent = keyId === currentKeyId;
          return isCurrent ? (
            <motion.span
              className="text-2xs text-green-600 dark:text-green-400 font-medium flex items-center gap-1"
              animate={{ opacity: [0.6, 1, 0.6] }}
              transition={{ repeat: Infinity, duration: 2 }}
            >
              <span className="w-1 h-1 rounded-full bg-green-500 inline-block" />
              活跃
            </motion.span>
          ) : (
            <Button size="xs" tone="primary" onClick={() => onSetActive(keyId)}>
              设为活跃
            </Button>
          );
        },
        size: 80,
      }),
    ],
    [currentKeyId, onSetActive],
  );

  const table = useReactTable({
    data,
    columns,
    state: { sorting, globalFilter: search },
    onSortingChange: setSorting,
    getCoreRowModel: getCoreRowModel(),
    getSortedRowModel: getSortedRowModel(),
    getFilteredRowModel: getFilteredRowModel(),
    globalFilterFn: (row, _columnId, filterValue: string) => {
      const v = filterValue.toLowerCase();
      return (
        row.original.key.masked.toLowerCase().includes(v) ||
        row.original.workspaceName.toLowerCase().includes(v) ||
        row.original.accountLabel.toLowerCase().includes(v)
      );
    },
  });

  const filteredRows = table.getRowModel().rows.filter((row) => {
    if (statusFilter !== "all" && row.original.key.status !== statusFilter) return false;
    if (goFilter === "go" && !row.original.subscribed) return false;
    if (goFilter === "nongo" && row.original.subscribed) return false;
    return true;
  });

  return (
    <div>
      <KeyTableToolbar
        search={search}
        onSearchChange={setSearch}
        statusFilter={statusFilter}
        onStatusFilterChange={setStatusFilter}
        goFilter={goFilter}
        onGoFilterChange={setGoFilter}
      />

      <div className="bg-white dark:bg-gray-800 rounded border border-gray-200 dark:border-gray-700 overflow-hidden">
        <table className="w-full">
          <thead>
            {table.getHeaderGroups().map((hg) => (
              <tr key={hg.id} className="border-b border-gray-100 dark:border-gray-700">
                {hg.headers.map((h) => (
                  <th
                    key={h.id}
                    style={{ width: h.getSize() }}
                    className="px-2.5 py-1.5 text-left text-2xs font-medium text-gray-400 dark:text-gray-500 cursor-pointer select-none hover:text-gray-600 dark:hover:text-gray-300"
                    onClick={h.column.getToggleSortingHandler()}
                  >
                    {h.column.columnDef.header as string}
                    {{ asc: " ↑", desc: " ↓" }[h.column.getIsSorted() as string] ?? ""}
                  </th>
                ))}
              </tr>
            ))}
          </thead>
          <tbody>
            <AnimatePresence>
              {filteredRows.map((row) => (
                <motion.tr
                  key={row.id}
                  initial={{ opacity: 0, y: -4 }}
                  animate={{ opacity: 1, y: 0 }}
                  exit={{ opacity: 0 }}
                  transition={{ duration: 0.15 }}
                  whileHover={{ backgroundColor: "rgba(0,0,0,0.02)" }}
                  className="border-b border-gray-50 dark:border-gray-800/50 last:border-0 transition-colors"
                >
                  {row.getVisibleCells().map((cell) => (
                    <td key={cell.id} className="px-2.5 py-1.5">
                      {cell.column.columnDef.cell
                        ? (cell.column.columnDef.cell as Function)(cell.getContext())
                        : null}
                    </td>
                  ))}
                </motion.tr>
              ))}
            </AnimatePresence>
          </tbody>
        </table>
        {filteredRows.length === 0 && (
          <motion.p
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            className="text-2xs text-gray-400 text-center py-8"
          >
            无匹配结果
          </motion.p>
        )}
      </div>
    </div>
  );
}
