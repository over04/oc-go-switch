import { fetchJson } from "@/shared/lib/http";
import type { LogEntry } from "@/shared/types/api";

export function getLogs(params?: { limit?: number; direction?: string; success?: boolean }): Promise<LogEntry[]> {
  const search = new URLSearchParams();
  if (params?.limit) search.set("limit", String(params.limit));
  if (params?.direction) search.set("direction", params.direction);
  if (params?.success !== undefined) search.set("success", String(params.success));
  const qs = search.toString();
  return fetchJson<LogEntry[]>(`/api/logs${qs ? `?${qs}` : ""}`);
}

export async function clearLogs(): Promise<void> {
  await fetch("/api/logs", { method: "DELETE" });
}
