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
    refresh_interval_secs: number;
    max_retries: number;
  }>,
): Promise<ConfigResponse> {
  return fetchJson<ConfigResponse>("/api/config", {
    method: "PUT",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(patch),
  });
}

export function refreshPool(): Promise<void> {
  return postEmpty("/api/pool/refresh");
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

export function setActiveKey(keyId: string): Promise<string> {
  return fetchJson<string>("/api/pool/active-key", {
    method: "PUT",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ key_id: keyId }),
  });
}

export function clearActiveKey(): Promise<string> {
  return fetchJson<string>("/api/pool/active-key", {
    method: "DELETE",
  });
}
