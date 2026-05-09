import { fetchJson } from "@/shared/lib/http";
import type { ModelListResult } from "@/shared/types/api";

export function getOpenaiModels(): Promise<ModelListResult> {
  return fetchJson<ModelListResult>("/api/models/openai");
}

export function getClaudeModels(): Promise<ModelListResult> {
  return fetchJson<ModelListResult>("/api/models/claude");
}
