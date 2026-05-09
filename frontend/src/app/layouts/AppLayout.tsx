import { Outlet, useLocation } from "react-router-dom";
import { AnimatePresence, motion } from "framer-motion";
import { Sidebar } from "@/shared/ui/Sidebar";
import { HealthDot } from "@/features/pool/components/HealthDot";
import { ToastContainer } from "@/shared/ui/ToastContainer";
import { usePoolStatus } from "@/features/pool/logic/usePoolStatus";

export function AppLayout() {
  const { data, isError } = usePoolStatus();
  const location = useLocation();

  return (
    <div className="h-screen flex bg-mcm-pattern">
      <Sidebar />

      <div className="flex-1 flex flex-col min-w-0">
        {/* Top bar */}
        <header className="shrink-0 h-11 flex items-center justify-end px-5 border-b border-cream-200 bg-white/80 backdrop-blur-sm">
          <HealthDot healthy={!isError} />
        </header>

        {/* Page content */}
        <main className="flex-1 overflow-y-auto p-5">
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
