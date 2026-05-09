import { Input } from "@/shared/ui/Input";

interface GeneralFormProps {
  listen: string;
  refreshIntervalSecs: number;
  maxRetries: number;
  baseUrl: string;
  onChangeRefreshInterval: (v: number) => void;
  onChangeMaxRetries: (v: number) => void;
}

export function GeneralForm({
  listen,
  refreshIntervalSecs,
  maxRetries,
  baseUrl,
  onChangeRefreshInterval,
  onChangeMaxRetries,
}: GeneralFormProps) {
  return (
    <div className="space-y-2.5">
      <div>
        <label className="block text-2xs text-gray-400 dark:text-gray-500 mb-0.5">监听地址</label>
        <Input value={listen} disabled className="font-mono" />
      </div>
      <div>
        <label className="block text-2xs text-gray-400 dark:text-gray-500 mb-0.5">刷新间隔 (秒)</label>
        <Input type="number" value={String(refreshIntervalSecs)} onChange={(e) => onChangeRefreshInterval(Number(e.target.value) || 60)} className="max-w-[120px]" />
      </div>
      <div>
        <label className="block text-2xs text-gray-400 dark:text-gray-500 mb-0.5">最大重试次数</label>
        <Input type="number" value={String(maxRetries)} onChange={(e) => onChangeMaxRetries(Number(e.target.value) || 1)} className="max-w-[120px]" />
      </div>
      <div>
        <label className="block text-2xs text-gray-400 dark:text-gray-500 mb-0.5">上游 Base URL</label>
        <Input value={baseUrl} disabled className="font-mono" />
      </div>
    </div>
  );
}
