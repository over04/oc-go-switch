import { fetchJson } from "@/shared/lib/http";
import type { PoolStatusResponse } from "@/shared/types/api";

export function getPoolStatus(): Promise<PoolStatusResponse> {
  return fetchJson<PoolStatusResponse>("/pool/status");
}
