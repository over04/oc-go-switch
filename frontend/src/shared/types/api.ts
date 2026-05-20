export type KeyStatus = "active" | "idle";
export type WorkspaceStatusKind = "available" | "exhausted" | "unsubscribed";

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
  status: WorkspaceStatusKind;
  is_current: boolean;
  queue_position: number | null;
  plan: string | null;
  go_usage: GoUsage | null;
  keys: KeyStatusEntry[];
}

export interface AccountStatus {
  name: string;
  label: string;
  workspaces: WorkspaceStatus[];
}

export interface WorkspaceScheduleResponse {
  current_key_id: string | null;
  last_refresh_at: string | null;
  accounts: AccountStatus[];
}

export interface DashboardStatusResponse {
  total_keys: number;
  available_keys: number;
  available_workspaces: number;
  exhausted_workspaces: number;
  unsubscribed_workspaces: number;
  last_refresh_at: string | null;
  go_workspaces: WorkspaceStatus[];
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

interface ImageFilterModelBase {
  model: string;
}

export type ImageFilterModel =
  | (ImageFilterModelBase & { action: "pass_through" })
  | (ImageFilterModelBase & { action: "remove" })
  | (ImageFilterModelBase & { action: "replace"; replacement: string });

export interface ImageFilterConfig {
  models: ImageFilterModel[];
}

export interface FixedConfigResponse {
  listen: string;
}

export interface RuntimeConfigResponse {
  refresh_interval_secs: number;
  max_retries: number;
  go: {
    base_url: string;
    connect_timeout_secs: number;
    request_timeout_secs: number;
  };
  accounts: AccountListEntry[];
  image_filter: ImageFilterConfig;
  api_token_set: boolean;
}

export interface ConfigResponse {
  fixed: FixedConfigResponse;
  runtime: RuntimeConfigResponse;
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
