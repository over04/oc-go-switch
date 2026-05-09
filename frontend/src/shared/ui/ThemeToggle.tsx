import { useCallback } from "react";
import { Button } from "./Button";

export type ThemeMode = "light" | "dark" | "auto";

const MODE_CYCLE: ThemeMode[] = ["auto", "dark", "light"];
const STORAGE_KEY = "oc-go-switch-theme";

function resolveMode(mode: ThemeMode): "light" | "dark" {
  if (mode === "auto") {
    return window.matchMedia("(prefers-color-scheme: dark)").matches
      ? "dark"
      : "light";
  }
  return mode;
}

function applyTheme(mode: ThemeMode) {
  const resolved = resolveMode(mode);
  document.documentElement.classList.toggle("dark", resolved === "dark");
}

export function getStoredTheme(): ThemeMode {
  return (localStorage.getItem(STORAGE_KEY) as ThemeMode) ?? "auto";
}

export function ThemeToggle() {
  const handleToggle = useCallback(() => {
    const current = getStoredTheme();
    const idx = MODE_CYCLE.indexOf(current);
    const next = MODE_CYCLE[(idx + 1) % MODE_CYCLE.length]!;
    localStorage.setItem(STORAGE_KEY, next);
    applyTheme(next);
    // Force re-render via a small hack — we dispatch a custom event
    window.dispatchEvent(new Event("themechange"));
  }, []);

  return (
    <Button size="xs" onClick={handleToggle}>
      {getStoredTheme()}
    </Button>
  );
}

// Initialize theme on import
applyTheme(getStoredTheme());
window.matchMedia("(prefers-color-scheme: dark)").addEventListener("change", () => {
  if (getStoredTheme() === "auto") applyTheme("auto");
});
