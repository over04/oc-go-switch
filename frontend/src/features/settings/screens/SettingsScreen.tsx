import { useState, useCallback, useEffect } from "react";
import { useQuery, useQueryClient } from "@tanstack/react-query";
import { motion } from "framer-motion";
import { GeneralForm } from "../components/GeneralForm";
import { ImageFilterForm } from "../components/ImageFilterForm";
import { getConfig, updateConfig } from "../service/settingsService";
import { toastSuccess, toastError } from "@/shared/lib/toast";
import { Button } from "@/shared/ui/Button";
import { Skeleton } from "@/shared/ui/Skeleton";
import type { ImageFilterModel } from "@/shared/types/api";

const CONFIG_KEY = ["api", "config"] as const;

export function SettingsScreen() {
  const queryClient = useQueryClient();
  const { data, isPending, isError } = useQuery({
    queryKey: CONFIG_KEY,
    queryFn: getConfig,
    staleTime: 30_000,
  });
  const [refreshInterval, setRefreshInterval] = useState(300);
  const [maxRetries, setMaxRetries] = useState(10);
  const [baseUrl, setBaseUrl] = useState("");
  const [connectTimeout, setConnectTimeout] = useState(90);
  const [requestTimeout, setRequestTimeout] = useState(90);
  const [imageFilterModels, setImageFilterModels] = useState<ImageFilterModel[]>([]);
  const [saving, setSaving] = useState(false);

  useEffect(() => {
    if (data) {
      setRefreshInterval(data.refresh_interval_secs);
      setMaxRetries(data.max_retries);
      setBaseUrl(data.go.base_url);
      setConnectTimeout(data.go.connect_timeout_secs);
      setRequestTimeout(data.go.request_timeout_secs);
      setImageFilterModels(data.image_filter?.models ?? []);
    }
  }, [data]);

  const handleSave = useCallback(async () => {
    setSaving(true);
    try {
      await updateConfig({
        refresh_interval_secs: refreshInterval,
        max_retries: maxRetries,
        go: {
          base_url: baseUrl,
          connect_timeout_secs: connectTimeout,
          request_timeout_secs: requestTimeout,
        },
        image_filter: { models: imageFilterModels },
      });
      queryClient.invalidateQueries({ queryKey: CONFIG_KEY });
      toastSuccess("配置已保存");
    } catch (e) {
      toastError(e instanceof Error ? e.message : "保存失败");
    } finally {
      setSaving(false);
    }
  }, [refreshInterval, maxRetries, baseUrl, connectTimeout, requestTimeout, imageFilterModels, queryClient]);

  if (isPending)
    return (
      <div className="space-y-4 max-w-lg">
        <Skeleton className="h-48" />
      </div>
    );
  if (isError || !data)
    return (
      <div className="flex items-center justify-center h-48">
        <p className="text-sm text-terra-400">加载配置失败</p>
      </div>
    );

  return (
    <div className="max-w-xl space-y-6">
      <div className="flex items-center gap-3">
        <div className="w-1 h-6 bg-harvest-300 rounded-full" />
        <h2 className="text-lg font-semibold text-espresso-700 tracking-tight">
          系统设置
        </h2>
      </div>

      <motion.div
        initial={{ opacity: 0, y: 8 }}
        animate={{ opacity: 1, y: 0 }}
        className="bg-white rounded-mcm-lg border border-cream-200 shadow-mcm overflow-hidden"
      >
        <div className="px-5 py-3 border-b border-cream-100">
          <h3 className="text-sm font-semibold text-espresso-700">通用配置</h3>
        </div>
        <div className="px-5 py-4">
          <GeneralForm
            listen={data.listen}
            refreshIntervalSecs={refreshInterval}
            maxRetries={maxRetries}
            baseUrl={baseUrl}
            connectTimeoutSecs={connectTimeout}
            requestTimeoutSecs={requestTimeout}
            onChangeRefreshInterval={setRefreshInterval}
            onChangeMaxRetries={setMaxRetries}
            onChangeConnectTimeout={setConnectTimeout}
            onChangeRequestTimeout={setRequestTimeout}
          />
        </div>
      </motion.div>

      <motion.div
        initial={{ opacity: 0, y: 8 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ delay: 0.1 }}
        className="bg-white rounded-mcm-lg border border-cream-200 shadow-mcm"
      >
        <div className="px-5 py-3 border-b border-cream-100">
          <h3 className="text-sm font-semibold text-espresso-700">图片过滤</h3>
        </div>
        <div className="px-5 py-4">
          <ImageFilterForm
            models={imageFilterModels}
            onChange={setImageFilterModels}
          />
        </div>
      </motion.div>

      <Button size="sm" tone="primary" onClick={handleSave} disabled={saving}>
        {saving ? "保存中..." : "保存配置"}
      </Button>
    </div>
  );
}
