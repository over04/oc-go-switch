type ToastType = "success" | "error" | "info";

interface Toast {
  id: number;
  message: string;
  type: ToastType;
}

let nextId = 0;
const listeners = new Set<() => void>();
let toasts: Toast[] = [];

export function subscribe(fn: () => void) {
  listeners.add(fn);
  return () => listeners.delete(fn);
}

export function getToasts(): Toast[] {
  return toasts;
}

function addToast(message: string, type: ToastType) {
  const id = nextId++;
  toasts = [...toasts, { id, message, type }];
  listeners.forEach((fn) => fn());
  setTimeout(() => {
    toasts = toasts.filter((t) => t.id !== id);
    listeners.forEach((fn) => fn());
  }, 3000);
}

export function toastSuccess(msg: string) {
  addToast(msg, "success");
}

export function toastError(msg: string) {
  addToast(msg, "error");
}

export function toastInfo(msg: string) {
  addToast(msg, "info");
}
