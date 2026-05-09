import { useState, useCallback, useEffect } from "react";
import { useQuery, useQueryClient } from "@tanstack/react-query";
import { GeneralForm } from "../components/GeneralForm";
import { getConfig, updateConfig } from "../service/settingsService";
import { toastSuccess, toastError } from "@/shared/lib/toast";
import { Button } from "@/shared/ui/Button";
import { Skeleton } from "@/shared/ui/Skeleton";

const CONFIG_KEY = ["api", "config"] as const;

export function SettingsScreen() {
  const queryClient = useQueryClient();
  const { data, isPending, isError } = useQuery({ queryKey: CONFIG_KEY, queryFn: getConfig, staleTime: 30_000 });
  const [refreshInterval, setRefreshInterval] = useState(300);
  const [maxRetries, setMaxRetries] = useState(10);
  const [saving, setSaving] = useState(false);

  useEffect(() => {
    if (data) { setRefreshInterval(data.refresh_interval_secs); setMaxRetries(data.max_retries); }
  }, [data]);

  const handleSave = useCallback(async () => {
    setSaving(true);
    try {
      await updateConfig({ refresh_interval_secs: refreshInterval, max_retries: maxRetries });
      toastSuccess("配置已保存");
    } catch (e) { toastError(e instanceof Error ? e.message : "保存失败"); }
    finally { setSaving(false); }
  }, [refreshInterval, maxRetries]);

  if (isPending) return <div className="space-y-4"><Skeleton className="h-48" /></div>;
  if (isError || !data) return <p className="text-xs text-red-500">加载配置失败</p>;

  return (
    <div className="space-y-5 max-w-lg">
      <div className="bg-white dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700 px-4 py-3">
        <h3 className="text-sm font-medium mb-2 text-gray-500 dark:text-gray-400">通用配置</h3>
        <GeneralForm
          listen={data.listen}
          refreshIntervalSecs={refreshInterval}
          maxRetries={maxRetries}
          baseUrl={data.upstream.base_url}
          onChangeRefreshInterval={setRefreshInterval}
          onChangeMaxRetries={setMaxRetries}
        />
      </div>
      <Button size="xs" tone="primary" onClick={handleSave} disabled={saving}>
        {saving ? "保存中..." : "保存配置"}
      </Button>
    </div>
  );
}
