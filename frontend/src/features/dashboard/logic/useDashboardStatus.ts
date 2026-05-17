import { useQuery } from "@tanstack/react-query";
import { REFRESH_INTERVAL_MS } from "@/shared/config";
import { getDashboardStatus } from "../service/dashboardService";

export const DASHBOARD_STATUS_KEY = ["dashboard", "status"] as const;

export function useDashboardStatus() {
  return useQuery({
    queryKey: DASHBOARD_STATUS_KEY,
    queryFn: getDashboardStatus,
    refetchInterval: REFRESH_INTERVAL_MS,
    staleTime: REFRESH_INTERVAL_MS * 0.8,
  });
}
