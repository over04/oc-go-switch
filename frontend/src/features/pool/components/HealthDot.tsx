import clsx from "clsx";

interface HealthDotProps {
  healthy: boolean;
}

export function HealthDot({ healthy }: HealthDotProps) {
  return (
    <span
      className={clsx(
        "w-2 h-2 rounded-full inline-block shrink-0",
        healthy ? "bg-green-500" : "bg-red-500",
      )}
    />
  );
}
