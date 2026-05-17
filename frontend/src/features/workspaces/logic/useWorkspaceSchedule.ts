import { useQuery } from "@tanstack/react-query";
import { REFRESH_INTERVAL_MS } from "@/shared/config";
import { getWorkspaceSchedule } from "../service/workspacesService";

export const WORKSPACE_SCHEDULE_KEY = ["workspaces", "schedule"] as const;

export function useWorkspaceSchedule() {
  return useQuery({
    queryKey: WORKSPACE_SCHEDULE_KEY,
    queryFn: getWorkspaceSchedule,
    refetchInterval: REFRESH_INTERVAL_MS,
    staleTime: REFRESH_INTERVAL_MS * 0.8,
  });
}
