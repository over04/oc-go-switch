import { fetchJson, postEmpty } from "@/shared/lib/http";
import type { AccountListResponse, ConfigResponse } from "@/shared/types/api";

export function getAccounts(): Promise<AccountListResponse> {
  return fetchJson<AccountListResponse>("/api/accounts");
}

export function addAccount(
  name: string,
  auth: string,
  label: string,
): Promise<AccountListResponse> {
  return fetchJson<AccountListResponse>("/api/accounts", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ name, auth, label }),
  });
}

export function deleteAccount(name: string): Promise<AccountListResponse> {
  return fetchJson<AccountListResponse>(`/api/accounts/${name}`, {
    method: "DELETE",
  });
}

export function getConfig(): Promise<ConfigResponse> {
  return fetchJson<ConfigResponse>("/api/config");
}

export function updateConfig(
  patch: Partial<{
    runtime: Partial<{
      refresh_interval_secs: number;
      max_retries: number;
      go: import("@/shared/types/api").RuntimeConfigResponse["go"];
      image_filter: import("@/shared/types/api").ImageFilterConfig;
    }>;
  }>,
): Promise<ConfigResponse> {
  return fetchJson<ConfigResponse>("/api/config", {
    method: "PUT",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(patch),
  });
}

export function refreshWorkspaces(): Promise<void> {
  return postEmpty("/api/workspaces/refresh");
}

export function editAccount(
  name: string,
  patch: { auth?: string; label?: string },
): Promise<AccountListResponse> {
  return fetchJson<AccountListResponse>(`/api/accounts/${name}`, {
    method: "PUT",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(patch),
  });
}

export function setCurrentWorkspace(
  workspaceId: string,
): Promise<{ status: string }> {
  return fetchJson<{ status: string }>("/api/workspaces/current", {
    method: "PUT",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ workspace_id: workspaceId }),
  });
}

export function clearCurrentWorkspace(): Promise<{ status: string }> {
  return fetchJson<{ status: string }>("/api/workspaces/current", {
    method: "DELETE",
  });
}
