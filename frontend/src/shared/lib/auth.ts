const TOKEN_KEY = "oc-go-switch:api-token";

export function getToken(): string | null {
  return localStorage.getItem(TOKEN_KEY);
}

export function setToken(token: string): void {
  localStorage.setItem(TOKEN_KEY, token);
}

export function clearToken(): void {
  localStorage.removeItem(TOKEN_KEY);
  window.dispatchEvent(new CustomEvent("auth:logout"));
}

export function hasToken(): boolean {
  return getToken() !== null;
}
