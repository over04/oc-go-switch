import { useQuery } from "@tanstack/react-query";
import { getPoolStatus } from "../service/poolService";
import { REFRESH_INTERVAL_MS } from "@/shared/config";

export const POOL_STATUS_KEY = ["pool", "status"] as const;

export function usePoolStatus() {
  return useQuery({
    queryKey: POOL_STATUS_KEY,
    queryFn: getPoolStatus,
    refetchInterval: REFRESH_INTERVAL_MS,
    staleTime: REFRESH_INTERVAL_MS * 0.8,
  });
}
