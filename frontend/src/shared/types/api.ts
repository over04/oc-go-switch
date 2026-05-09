export type KeyStatus = "active" | "idle" | "depleted";

export interface GoUsage {
  hourly_percent: number;
  hourly_reset_sec: number;
  weekly_percent: number;
  weekly_reset_sec: number;
  monthly_percent: number;
  monthly_reset_sec: number;
}

export interface KeyStatusEntry {
  id: string;
  masked: string;
  status: KeyStatus;
}

export interface WorkspaceStatus {
  id: string;
  name: string;
  subscribed: boolean;
  plan: string | null;
  go_usage: GoUsage | null;
  keys: KeyStatusEntry[];
}

export interface AccountStatus {
  name: string;
  label: string;
  workspaces: WorkspaceStatus[];
}

export interface PoolStatusResponse {
  total_keys: number;
  available_keys: number;
  depleted_keys: number;
  current_key_id: string | null;
  accounts: AccountStatus[];
}

export interface AccountListEntry {
  name: string;
  label: string;
  auth_masked: string;
}

export interface AccountListResponse {
  accounts: AccountListEntry[];
}

export type FilterAction = "pass_through" | "remove" | "replace";

export interface ImageFilterModel {
  model: string;
  action: FilterAction;
  replacement?: string | null;
}

export interface ImageFilterConfig {
  models: ImageFilterModel[];
}

export interface ConfigResponse {
  listen: string;
  refresh_interval_secs: number;
  max_retries: number;
  go: { base_url: string };
  accounts: AccountListEntry[];
  image_filter: ImageFilterConfig;
}

export interface ModelInfo {
  id: string;
  owned_by: string;
}

export interface ModelListResponse {
  object: string;
  data: ModelInfo[];
  error?: never;
}

export interface ModelFetchError {
  error: string;
  object?: never;
}

export type ModelListResult = ModelListResponse | ModelFetchError;

export interface LogEntry {
  timestamp: string;
  direction: "openai" | "claude";
  model: string | null;
  status_code: number;
  duration_ms: number;
  key_masked: string;
  success: boolean;
  error_message: string | null;
}
