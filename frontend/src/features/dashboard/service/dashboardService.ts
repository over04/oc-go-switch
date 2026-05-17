import { fetchJson } from "@/shared/lib/http";
import type { DashboardStatusResponse } from "@/shared/types/api";

export function getDashboardStatus(): Promise<DashboardStatusResponse> {
  return fetchJson<DashboardStatusResponse>("/api/dashboard/status");
}
