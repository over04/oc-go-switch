import { Outlet, useLocation } from "react-router-dom";
import { AnimatePresence, motion } from "framer-motion";
import { Sidebar } from "@/shared/ui/Sidebar";
import { HealthDot } from "@/features/pool/components/HealthDot";
import { ActiveKeyBar } from "@/features/keys/components/ActiveKeyBar";
import { ToastContainer } from "@/shared/ui/ToastContainer";
import { usePoolStatus } from "@/features/pool/logic/usePoolStatus";

export function AppLayout() {
  const { data, isError } = usePoolStatus();
  const location = useLocation();

  const activeKeyId = data?.current_key_id
    ? data.current_key_id.split("/").pop()
    : null;

  return (
    <div className="h-screen flex bg-gray-50 dark:bg-gray-900 text-gray-900 dark:text-gray-100">
      <Sidebar />
      <div className="flex-1 flex flex-col min-w-0">
        {/* Top bar */}
        <header className="shrink-0 h-10 flex items-center justify-between px-4 border-b border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-800">
          <div className="flex items-center gap-3">
            <span className="text-2xs text-gray-400 dark:text-gray-500">
              OpenCode API Key Proxy
            </span>
            {activeKeyId && (
              <motion.code
                initial={{ opacity: 0, scale: 0.9 }}
                animate={{ opacity: 1, scale: 1 }}
                className="text-2xs font-mono text-green-600 dark:text-green-400 bg-green-50 dark:bg-green-900/30 px-1.5 py-0.5 rounded flex items-center gap-1"
              >
                <motion.span
                  className="w-1 h-1 rounded-full bg-green-500 inline-block"
                  animate={{ scale: [1, 1.3, 1] }}
                  transition={{ repeat: Infinity, duration: 2 }}
                />
                {activeKeyId}
              </motion.code>
            )}
          </div>
          <div className="flex items-center gap-2">
            <HealthDot healthy={!isError} />
          </div>
        </header>

        {/* Page content with transitions */}
        <main className="flex-1 overflow-y-auto p-4">
          <AnimatePresence mode="wait">
            <motion.div
              key={location.pathname}
              initial={{ opacity: 0, y: 8 }}
              animate={{ opacity: 1, y: 0 }}
              exit={{ opacity: 0, y: -8 }}
              transition={{ duration: 0.2, ease: "easeOut" }}
            >
              <Outlet />
            </motion.div>
          </AnimatePresence>
        </main>
      </div>
      <ToastContainer />
    </div>
  );
}
