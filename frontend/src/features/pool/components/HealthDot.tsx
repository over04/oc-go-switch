import clsx from "clsx";
import { motion } from "framer-motion";

interface HealthDotProps {
  healthy: boolean;
}

export function HealthDot({ healthy }: HealthDotProps) {
  return (
    <div className="flex items-center gap-1.5">
      <motion.span
        className={clsx(
          "relative flex h-2 w-2",
        )}
        animate={{ opacity: healthy ? [0.5, 1, 0.5] : 1 }}
        transition={healthy ? { repeat: Infinity, duration: 2 } : {}}
      >
        <span
          className={clsx(
            "inline-flex h-full w-full rounded-full",
            healthy ? "bg-harvest-500" : "bg-terra-400",
          )}
        />
      </motion.span>
      <span className="text-[0.625rem] text-espresso-400 font-medium uppercase tracking-wider">
        {healthy ? "在线" : "离线"}
      </span>
    </div>
  );
}
