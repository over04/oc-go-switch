import { useState, useCallback } from "react";
import { Outlet, useLocation } from "react-router-dom";
import { AnimatePresence, motion } from "framer-motion";
import { Sidebar } from "@/shared/ui/Sidebar";
import { HealthDot } from "@/features/pool/components/HealthDot";
import { ToastContainer } from "@/shared/ui/ToastContainer";
import { useDashboardStatus } from "@/features/dashboard/logic/useDashboardStatus";

export function AppLayout() {
  const { isError } = useDashboardStatus();
  const location = useLocation();
  const [sidebarOpen, setSidebarOpen] = useState(false);

  const closeSidebar = useCallback(() => setSidebarOpen(false), []);

  return (
    <div className="h-screen flex bg-mcm-pattern">
      <Sidebar open={sidebarOpen} onClose={closeSidebar} />

      <div className="flex-1 flex flex-col min-w-0">
        <header className="shrink-0 h-11 flex items-center justify-between px-4 md:px-5 border-b border-cream-200 bg-white/80 backdrop-blur-sm">
          <button
            onClick={() => setSidebarOpen((v) => !v)}
            className="md:hidden p-1.5 -ml-1 rounded-lg text-espresso-600 hover:bg-cream-100 transition-colors"
            aria-label="Toggle menu"
          >
            <svg width="20" height="20" viewBox="0 0 20 20" fill="none">
              <path
                d="M3 5h14M3 10h14M3 15h14"
                stroke="currentColor"
                strokeWidth="1.8"
                strokeLinecap="round"
              />
            </svg>
          </button>
          <div className="flex-1 md:hidden" />
          <HealthDot healthy={!isError} />
        </header>

        <main className="flex-1 overflow-y-auto p-4 md:p-5">
          <AnimatePresence mode="wait">
            <motion.div
              key={location.pathname}
              initial={{ opacity: 0, y: 6 }}
              animate={{ opacity: 1, y: 0 }}
              exit={{ opacity: 0, y: -6 }}
              transition={{ duration: 0.25, ease: "easeOut" }}
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
