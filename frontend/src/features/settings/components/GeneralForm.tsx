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
    <div className="space-y-4">
      <div>
        <label className="block text-xs font-medium text-espresso-500 mb-1.5">
          监听地址
        </label>
        <Input value={listen} disabled className="font-mono opacity-60" />
      </div>

      <div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
        <div>
          <label className="block text-xs font-medium text-espresso-500 mb-1.5">
            刷新间隔（秒）
          </label>
          <Input
            type="number"
            value={String(refreshIntervalSecs)}
            onChange={(e) =>
              onChangeRefreshInterval(Number(e.target.value) || 60)
            }
          />
        </div>
        <div>
          <label className="block text-xs font-medium text-espresso-500 mb-1.5">
            最大重试次数
          </label>
          <Input
            type="number"
            value={String(maxRetries)}
            onChange={(e) =>
              onChangeMaxRetries(Number(e.target.value) || 1)
            }
          />
        </div>
      </div>

      <div>
        <label className="block text-xs font-medium text-espresso-500 mb-1.5">
          上游 Base URL
        </label>
        <Input value={baseUrl} disabled className="font-mono opacity-60" />
      </div>
    </div>
  );
}
