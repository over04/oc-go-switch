import { Input } from "@/shared/ui/Input";

interface GeneralFormProps {
  listen: string;
  refreshIntervalSecs: number;
  maxRetries: number;
  baseUrl: string;
  connectTimeoutSecs: number;
  requestTimeoutSecs: number;
  onChangeRefreshInterval: (v: number) => void;
  onChangeMaxRetries: (v: number) => void;
  onChangeConnectTimeout: (v: number) => void;
  onChangeRequestTimeout: (v: number) => void;
}

export function GeneralForm({
  listen,
  refreshIntervalSecs,
  maxRetries,
  baseUrl,
  connectTimeoutSecs,
  requestTimeoutSecs,
  onChangeRefreshInterval,
  onChangeMaxRetries,
  onChangeConnectTimeout,
  onChangeRequestTimeout,
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

      <div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
        <div>
          <label className="block text-xs font-medium text-espresso-500 mb-1.5">
            连接超时（秒）
          </label>
          <Input
            type="number"
            value={String(connectTimeoutSecs)}
            onChange={(e) =>
              onChangeConnectTimeout(Number(e.target.value) || 90)
            }
          />
        </div>
        <div>
          <label className="block text-xs font-medium text-espresso-500 mb-1.5">
            请求超时（秒）
          </label>
          <Input
            type="number"
            value={String(requestTimeoutSecs)}
            onChange={(e) =>
              onChangeRequestTimeout(Number(e.target.value) || 90)
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
