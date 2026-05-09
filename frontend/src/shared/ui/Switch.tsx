import clsx from "clsx";

interface SwitchProps {
  checked: boolean;
  onChange: (checked: boolean) => void;
  disabled?: boolean;
  size?: "xs" | "sm";
}

export function Switch({
  checked,
  onChange,
  disabled = false,
  size = "xs",
}: SwitchProps) {
  const trackH = size === "xs" ? "h-5" : "h-6";
  const trackW = size === "xs" ? "w-9" : "w-10";
  const thumb = size === "xs" ? "h-3.5 w-3.5" : "h-4 w-4";

  return (
    <button
      type="button"
      role="switch"
      aria-checked={checked}
      disabled={disabled}
      onClick={() => onChange(!checked)}
      className={clsx(
        "relative inline-flex shrink-0 rounded-full transition-colors",
        "focus:outline-none focus-visible:ring-1 focus-visible:ring-blue-400",
        trackH,
        trackW,
        checked
          ? "bg-blue-600"
          : "bg-gray-300 dark:bg-gray-600",
        disabled && "opacity-40 cursor-not-allowed",
      )}
    >
      <span
        className={clsx(
          "absolute top-0.5 rounded-full bg-white shadow transition-transform",
          thumb,
          checked ? "translate-x-[14px]" : "translate-x-0.5",
        )}
      />
    </button>
  );
}
