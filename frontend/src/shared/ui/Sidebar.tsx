import { NavLink, useLocation } from "react-router-dom";
import { motion } from "framer-motion";
import clsx from "clsx";
import { ThemeToggle } from "./ThemeToggle";

const linkBase = "relative flex items-center h-8 px-3 text-sm rounded transition-colors z-10";

export function Sidebar() {
  const location = useLocation();

  return (
    <aside className="w-52 shrink-0 flex flex-col bg-gray-900 dark:bg-gray-950 border-r border-gray-800">
      <motion.div
        className="h-11 flex items-center px-3 border-b border-gray-800"
        initial={{ opacity: 0, x: -12 }}
        animate={{ opacity: 1, x: 0 }}
      >
        <span className="text-sm font-semibold text-gray-100 tracking-tight">oc-go-switch</span>
      </motion.div>

      <nav className="flex-1 py-2 px-2 space-y-0.5 overflow-y-auto">
        {/* Dashboard */}
        <NavItem to="/" label="仪表盘" location={location} index={0} />

        {/* Go section */}
        <div className="pt-2 pb-0.5">
          <span className="text-xs font-medium text-gray-500 uppercase tracking-wider px-2">Go</span>
        </div>
        <NavItem to="/keys" label="Go订阅密钥池" location={location} index={1} />
        <NavItem to="/models" label="模型" location={location} index={2} />

        <div className="pt-2 pb-0.5">
          <span className="text-xs font-medium text-gray-500 uppercase tracking-wider px-2">系统</span>
        </div>
        <NavItem to="/logs" label="日志" location={location} index={3} />
        <NavItem to="/accounts" label="账户" location={location} index={4} />
        <NavItem to="/settings" label="设置" location={location} index={5} />
      </nav>

      <div className="px-2 py-2 border-t border-gray-800">
        <ThemeToggle />
      </div>
    </aside>
  );
}

function NavItem({ to, label, location, index }: { to: string; label: string; location: ReturnType<typeof useLocation>; index: number }) {
  const active = to === "/" ? location.pathname === "/" : location.pathname.startsWith(to);
  return (
    <motion.div
      initial={{ opacity: 0, x: -16 }}
      animate={{ opacity: 1, x: 0 }}
      transition={{ delay: 0.05 * index, duration: 0.25 }}
    >
      <NavLink to={to} className={clsx(linkBase, active ? "bg-blue-600 text-white" : "text-gray-400 hover:text-gray-200 hover:bg-gray-800")}>
        {active && <motion.div layoutId="nav-active" className="absolute inset-0 bg-blue-600 rounded z-0" transition={{ type: "spring", stiffness: 500, damping: 35 }} />}
        <span className="relative z-10">{label}</span>
      </NavLink>
    </motion.div>
  );
}
