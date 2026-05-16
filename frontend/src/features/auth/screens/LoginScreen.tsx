import { useState } from "react";
import { motion } from "framer-motion";
import { useAuth } from "../logic/AuthProvider";

export function LoginScreen() {
  const { login, error } = useAuth();
  const [value, setValue] = useState("");
  const [pending, setPending] = useState(false);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    const trimmed = value.trim();
    if (!trimmed) return;
    setPending(true);
    try {
      await login(trimmed);
    } finally {
      setPending(false);
    }
  };

  return (
    <div className="min-h-screen flex items-center justify-center bg-mcm-pattern px-4">
      <motion.div
        initial={{ opacity: 0, y: 16 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ duration: 0.4, ease: "easeOut" }}
        className="w-full max-w-sm"
      >
        <div className="text-center mb-8">
          <h1 className="text-2xl font-bold text-espresso-700 tracking-tight">
            Go Switch
          </h1>
          <p className="text-sm text-espresso-400 mt-1">输入 API Token 以访问管理面板</p>
        </div>

        <form
          onSubmit={handleSubmit}
          className="bg-white rounded-mcm-xl border border-cream-200 shadow-mcm p-6 space-y-4"
        >
          <div>
            <label
              htmlFor="token"
              className="block text-xs font-semibold text-espresso-500 mb-1.5 uppercase tracking-wider"
            >
              API Token
            </label>
            <input
              id="token"
              type="password"
              autoFocus
              autoComplete="off"
              value={value}
              onChange={(e) => setValue(e.target.value)}
              placeholder="输入你的 API Token"
              className="w-full px-3 py-2 text-sm border border-cream-200 rounded-mcm bg-cream-50/50
                         placeholder:text-espresso-300 text-espresso-700
                         focus:outline-none focus:ring-2 focus:ring-harvest-500/30 focus:border-harvest-400
                         transition-colors"
            />
          </div>

          {error && (
            <motion.p
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              className="text-xs text-terra-500 bg-terra-400/5 rounded-mcm px-3 py-2"
            >
              {error}
            </motion.p>
          )}

          <button
            type="submit"
            disabled={pending || !value.trim()}
            className="w-full py-2.5 text-sm font-semibold text-white bg-espresso-700 rounded-mcm
                       hover:bg-espresso-800 disabled:opacity-40 disabled:cursor-not-allowed
                       transition-colors"
          >
            {pending ? "验证中..." : "登录"}
          </button>
        </form>
      </motion.div>
    </div>
  );
}
