import { useState } from "react";
import { Dialog } from "@/shared/ui/Dialog";
import { Button } from "@/shared/ui/Button";
import { Input } from "@/shared/ui/Input";

interface AddAccountDialogProps {
  open: boolean;
  onClose: () => void;
  onAdd: (name: string, auth: string, label: string) => Promise<void>;
}

export function AddAccountDialog({
  open,
  onClose,
  onAdd,
}: AddAccountDialogProps) {
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
      <div className="space-y-3.5">
        <div>
          <label className="block text-xs font-medium text-espresso-500 mb-1">
            名称
          </label>
          <Input
            value={name}
            onChange={(e) => setName(e.target.value)}
            placeholder="my-account"
          />
        </div>
        <div>
          <label className="block text-xs font-medium text-espresso-500 mb-1">
            标签
          </label>
          <Input
            value={label}
            onChange={(e) => setLabel(e.target.value)}
            placeholder="主账号"
          />
        </div>
        <div>
          <label className="block text-xs font-medium text-espresso-500 mb-1">
            Auth Cookie
          </label>
          <textarea
            value={auth}
            onChange={(e) => setAuth(e.target.value)}
            rows={3}
            className="w-full px-3 py-2 text-xs rounded-[10px] border border-cream-300 bg-white focus:outline-none focus:ring-[3px] focus:ring-terra-500/15 focus:border-terra-500 font-mono transition-all duration-200"
            placeholder="Fe26.2**..."
          />
        </div>
        {error && (
          <p className="text-xs text-terra-400 font-medium">{error}</p>
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
