import { createContext, useCallback, useContext, useEffect, useMemo, useState, type ReactNode } from "react";
import { useQueryClient } from "@tanstack/react-query";
import { getToken, setToken, clearToken, hasToken } from "@/shared/lib/auth";
import { fetchJson } from "@/shared/lib/http";
import type { ConfigResponse } from "@/shared/types/api";

interface AuthState {
  status: "loading" | "authenticated" | "unauthenticated";
  token: string | null;
  error: string | null;
  login: (token: string) => Promise<void>;
  logout: () => void;
}

const AuthContext = createContext<AuthState | null>(null);

export function useAuth(): AuthState {
  const ctx = useContext(AuthContext);
  if (!ctx) throw new Error("useAuth must be used within AuthProvider");
  return ctx;
}

export function AuthProvider({ children }: { children: ReactNode }) {
  const [status, setStatus] = useState<AuthState["status"]>("loading");
  const [error, setError] = useState<string | null>(null);
  const [token, setTokenState] = useState<string | null>(getToken);
  const queryClient = useQueryClient();

  const verify = useCallback(async (t: string): Promise<boolean> => {
    setToken(t);
    try {
      // fetchJson reads token from localStorage via getToken(), so we need to set it first
      setTokenState(t);
      await fetchJson<ConfigResponse>("/api/config");
      setStatus("authenticated");
      setError(null);
      return true;
    } catch {
      clearToken();
      setTokenState(null);
      setStatus("unauthenticated");
      return false;
    }
  }, []);

  const login = useCallback(async (t: string) => {
    setStatus("loading");
    setError(null);
    setToken(t);
    setTokenState(t);
    try {
      await fetchJson<ConfigResponse>("/api/config");
      setStatus("authenticated");
      setError(null);
    } catch (e) {
      clearToken();
      setTokenState(null);
      setStatus("unauthenticated");
      setError(e instanceof Error ? e.message : "认证失败");
    }
  }, []);

  const logout = useCallback(() => {
    clearToken();
    setTokenState(null);
    setStatus("unauthenticated");
    setError(null);
    queryClient.clear();
  }, [queryClient]);

  // On mount: verify existing token
  useEffect(() => {
    const existing = getToken();
    if (existing) {
      verify(existing);
    } else {
      setStatus("unauthenticated");
    }
  }, [verify]);

  // Listen for forced logout (e.g. 401 from QueryCache.onError)
  useEffect(() => {
    const handler = () => {
      setTokenState(null);
      setStatus("unauthenticated");
      setError(null);
      queryClient.clear();
    };
    window.addEventListener("auth:logout", handler);
    return () => window.removeEventListener("auth:logout", handler);
  }, [queryClient]);

  const value = useMemo<AuthState>(
    () => ({ status, token, error, login, logout }),
    [status, token, error, login, logout],
  );

  return <AuthContext.Provider value={value}>{children}</AuthContext.Provider>;
}
