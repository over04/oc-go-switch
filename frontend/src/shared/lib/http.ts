import { API_BASE } from "@/shared/config";
import { getToken, clearToken } from "@/shared/lib/auth";

function authHeaders(): Record<string, string> {
  const token = getToken();
  return token ? { Authorization: `Bearer ${token}` } : {};
}

export class AuthError extends Error {
  constructor(
    message: string,
    public status: number,
  ) {
    super(message);
    this.name = "AuthError";
  }
}

export async function fetchJson<T>(path: string, init?: RequestInit): Promise<T> {
  const res = await fetch(`${API_BASE}${path}`, {
    ...init,
    headers: { ...authHeaders(), ...init?.headers },
  });
  if (!res.ok) {
    if (res.status === 401) {
      clearToken();
      throw new AuthError("认证失败，请重新登录", res.status);
    }
    const text = await res.text().catch(() => "");
    throw new Error(text || `HTTP ${res.status}`);
  }
  return res.json();
}

export async function postEmpty(path: string): Promise<void> {
  const res = await fetch(`${API_BASE}${path}`, {
    method: "POST",
    headers: authHeaders(),
  });
  if (!res.ok) {
    if (res.status === 401) {
      clearToken();
      throw new AuthError("认证失败，请重新登录", res.status);
    }
    throw new Error(`HTTP ${res.status}`);
  }
}
