import { fetchJson } from "@/shared/lib/http";
import type { WorkspaceScheduleResponse } from "@/shared/types/api";

export function getWorkspaceSchedule(): Promise<WorkspaceScheduleResponse> {
  return fetchJson<WorkspaceScheduleResponse>("/api/workspaces/status");
}
