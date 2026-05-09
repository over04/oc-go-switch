import { useState } from "react";
import { Dialog } from "@/shared/ui/Dialog";
import { Button } from "@/shared/ui/Button";

interface AddAccountDialogProps {
  open: boolean;
  onClose: () => void;
  onAdd: (name: string, auth: string, label: string) => Promise<void>;
}

export function AddAccountDialog({ open, onClose, onAdd }: AddAccountDialogProps) {
  const [name, setName] = useState("");
  const [auth, setAuth] = useState("");
  const [label, setLabel] = useState("");
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState("");

  const handleSubmit = async () => {
    if (!name.trim() || !auth.trim() || !label.trim()) {
      setError("所有字段必填");
      return;
    }
    setSaving(true);
    setError("");
    try {
      await onAdd(name.trim(), auth.trim(), label.trim());
      setName("");
      setAuth("");
      setLabel("");
      onClose();
    } catch (e) {
      setError(e instanceof Error ? e.message : "未知错误");
    } finally {
      setSaving(false);
    }
  };

  return (
    <Dialog open={open} title="添加账户" onClose={onClose}>
      <div className="space-y-2.5">
        <div>
          <label className="block text-2xs text-gray-500 dark:text-gray-400 mb-0.5">
            名称
          </label>
          <input
            value={name}
            onChange={(e) => setName(e.target.value)}
            className="w-full h-7 px-2 text-xs rounded border border-gray-200 dark:border-gray-600 bg-gray-50 dark:bg-gray-700 focus:outline-none focus:ring-1 focus:ring-blue-400"
            placeholder="my-account"
          />
        </div>
        <div>
          <label className="block text-2xs text-gray-500 dark:text-gray-400 mb-0.5">
            标签
          </label>
          <input
            value={label}
            onChange={(e) => setLabel(e.target.value)}
            className="w-full h-7 px-2 text-xs rounded border border-gray-200 dark:border-gray-600 bg-gray-50 dark:bg-gray-700 focus:outline-none focus:ring-1 focus:ring-blue-400"
            placeholder="主账号"
          />
        </div>
        <div>
          <label className="block text-2xs text-gray-500 dark:text-gray-400 mb-0.5">
            Auth Cookie
          </label>
          <textarea
            value={auth}
            onChange={(e) => setAuth(e.target.value)}
            rows={3}
            className="w-full px-2 py-1 text-2xs rounded border border-gray-200 dark:border-gray-600 bg-gray-50 dark:bg-gray-700 focus:outline-none focus:ring-1 focus:ring-blue-400 font-mono"
            placeholder="Fe26.2**..."
          />
        </div>
        {error && (
          <p className="text-2xs text-red-500">{error}</p>
        )}
        <div className="flex justify-end gap-2 pt-1">
          <Button size="xs" onClick={onClose} disabled={saving}>
            取消
          </Button>
          <Button size="xs" tone="primary" onClick={handleSubmit} disabled={saving}>
            {saving ? "添加中..." : "添加"}
          </Button>
        </div>
      </div>
    </Dialog>
  );
}
