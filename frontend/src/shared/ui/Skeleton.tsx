import { motion } from "framer-motion";
import clsx from "clsx";

interface SkeletonProps {
  className?: string;
}

export function Skeleton({ className }: SkeletonProps) {
  return (
    <motion.div
      className={clsx(
        "rounded-mcm bg-cream-200 overflow-hidden",
        className,
      )}
    >
      <motion.div
        className="h-full w-full"
        style={{
          background:
            "linear-gradient(90deg, transparent 0%, rgba(245,230,211,0.6) 50%, transparent 100%)",
          backgroundSize: "200% 100%",
        }}
        animate={{ backgroundPosition: ["200% 0", "-200% 0"] }}
        transition={{ repeat: Infinity, duration: 1.8, ease: "easeInOut" }}
      />
    </motion.div>
  );
}
