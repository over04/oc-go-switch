import { NavLink, useLocation } from "react-router-dom";
import { motion } from "framer-motion";
import clsx from "clsx";

const linkBase =
  "relative flex items-center h-9 px-4 text-sm rounded-full transition-all duration-200 z-10 tracking-wide";

export function Sidebar() {
  const location = useLocation();

  return (
    <aside className="w-56 shrink-0 flex flex-col bg-espresso-800 border-r border-espresso-700/50">
      {/* Logo area with starburst accent */}
      <motion.div
        className="h-14 flex items-center px-5 border-b border-espresso-700/50 relative overflow-hidden"
        initial={{ opacity: 0 }}
        animate={{ opacity: 1 }}
      >
        {/* Decorative starburst */}
        <div className="absolute -top-4 -right-4 w-20 h-20 opacity-10">
          <div className="absolute inset-0 bg-terra-500 rounded-full" />
          <div className="absolute top-1/2 left-0 right-0 h-[2px] bg-terra-500 -translate-y-1/2" />
          <div className="absolute left-1/2 top-0 bottom-0 w-[2px] bg-terra-500 -translate-x-1/2" />
        </div>
        <span className="text-base font-semibold text-cream-100 tracking-tight">
          Go Switch
        </span>
      </motion.div>

      {/* Navigation */}
      <nav className="flex-1 py-4 px-3 space-y-1 overflow-y-auto">
        <NavItem
          to="/"
          label="仪表盘"
          location={location}
          index={0}
          icon="◇"
        />

        <div className="pt-3 pb-1">
          <span className="text-[0.6875rem] font-semibold text-espresso-300 uppercase tracking-[0.15em] px-4">
            Go 订阅
          </span>
        </div>
        <NavItem
          to="/keys"
          label="密钥池"
          location={location}
          index={1}
          icon="◆"
        />
        <NavItem
          to="/models"
          label="模型"
          location={location}
          index={2}
          icon="◈"
        />

        <div className="pt-3 pb-1">
          <span className="text-[0.6875rem] font-semibold text-espresso-300 uppercase tracking-[0.15em] px-4">
            系统
          </span>
        </div>
        <NavItem
          to="/logs"
          label="日志"
          location={location}
          index={3}
          icon="◎"
        />
        <NavItem
          to="/accounts"
          label="账户"
          location={location}
          index={4}
          icon="◉"
        />
        <NavItem
          to="/settings"
          label="设置"
          location={location}
          index={5}
          icon="◐"
        />
      </nav>

    </aside>
  );
}

function NavItem({
  to,
  label,
  location,
  index,
  icon,
}: {
  to: string;
  label: string;
  location: ReturnType<typeof useLocation>;
  index: number;
  icon: string;
}) {
  const active =
    to === "/"
      ? location.pathname === "/"
      : location.pathname.startsWith(to);
  return (
    <motion.div
      initial={{ opacity: 0, x: -12 }}
      animate={{ opacity: 1, x: 0 }}
      transition={{ delay: 0.04 * index, duration: 0.3, ease: "easeOut" }}
    >
      <NavLink
        to={to}
        className={clsx(
          linkBase,
          active
            ? "bg-terra-500 text-white shadow-sm"
            : "text-espresso-200 hover:text-cream-100 hover:bg-espresso-700",
        )}
      >
        {active && (
          <motion.div
            layoutId="nav-active"
            className="absolute inset-0 bg-terra-500 rounded-full z-0"
            transition={{ type: "spring", stiffness: 400, damping: 30 }}
          />
        )}
        <span className="relative z-10 flex items-center gap-2.5">
          <span className="text-xs opacity-70">{icon}</span>
          {label}
        </span>
      </NavLink>
    </motion.div>
  );
}
