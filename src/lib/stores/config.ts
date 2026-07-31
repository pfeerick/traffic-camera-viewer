import { get, writable } from "svelte/store";
import type { AppConfig } from "$lib/api";
import { api } from "$lib/api";

export const appConfig = writable<AppConfig | null>(null);
export const pendingConfig = writable<AppConfig | null>(null);

export async function loadConfig(): Promise<void> {
  const cfg = await api.getConfig();
  appConfig.set(cfg);
  pendingConfig.set(structuredClone(cfg));
}

export async function applyConfig(): Promise<void> {
  const cfg = get(pendingConfig);
  if (!cfg) return;
  await api.saveConfig(cfg);
  appConfig.set(structuredClone(cfg));
}

export function cancelConfig(): void {
  const cfg = get(appConfig);
  if (cfg) pendingConfig.set(structuredClone(cfg));
}
