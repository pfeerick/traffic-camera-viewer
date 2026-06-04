import { writable, get } from "svelte/store";
import { api } from "$lib/api";
import type { AppConfig } from "$lib/api";
import { visibleCameras, cameraImages } from "./cameras";
import { appConfig } from "./config";

export const lastRefresh = writable<Date | null>(null);
export const nextRefresh = writable<Date | null>(null);
export const refreshCountdown = writable<number>(0);
export const isLoadingCameras = writable<boolean>(false);

let timerHandle: ReturnType<typeof setInterval> | null = null;

export function startRefreshTimer(): void {
  if (timerHandle !== null) clearInterval(timerHandle);
  timerHandle = setInterval(tickTimer, 1000);
}

export function stopRefreshTimer(): void {
  if (timerHandle !== null) {
    clearInterval(timerHandle);
    timerHandle = null;
  }
}

function tickTimer(): void {
  const next = get(nextRefresh);
  if (!next) return;
  const remaining = Math.max(
    0,
    Math.floor((next.getTime() - Date.now()) / 1000)
  );
  refreshCountdown.set(remaining);
  if (remaining <= 0) {
    void triggerRefreshAll();
  }
}

export async function triggerRefreshAll(): Promise<void> {
  const cameras = get(visibleCameras);
  const cfg = get(appConfig);
  if (!cfg || cameras.length === 0) return;

  scheduleNextRefresh(cfg);

  await Promise.allSettled(
    cameras.map((cam) =>
      fetchSingleCamera(cam.id, cam.image_url, cfg)
    )
  );
}

export async function triggerRefreshSingle(cameraId: number): Promise<void> {
  const cfg = get(appConfig);
  const cameras = get(visibleCameras);
  const cam = cameras.find((c) => c.id === cameraId);
  if (!cfg || !cam) return;
  await fetchSingleCamera(cam.id, cam.image_url, cfg);
}

async function fetchSingleCamera(
  id: number,
  imageUrl: string,
  cfg: AppConfig
): Promise<void> {
  const current = get(cameraImages)[id];

  cameraImages.update((m) => ({
    ...m,
    [id]: { status: "loading", prev_src: current?.src },
  }));

  try {
    const result = await api.fetchImage({
      camera_id: id,
      image_url: imageUrl,
      previous_hash: current?.hash ?? null,
      save_to_disk: cfg.save_to_disk,
      save_path: cfg.save_path,
      max_snapshots: cfg.max_snapshots,
    });

    if (result.type === "changed") {
      cameraImages.update((m) => ({
        ...m,
        [id]: {
          status: "ready",
          src: `data:image/jpeg;base64,${result.jpeg_b64}`,
          hash: result.hash,
        },
      }));
    } else {
      cameraImages.update((m) => ({
        ...m,
        [id]: { ...m[id], status: "ready", hash: result.hash },
      }));
    }
  } catch (err) {
    cameraImages.update((m) => ({
      ...m,
      [id]: {
        ...m[id],
        status: "error",
        error: err instanceof Error ? err.message : String(err),
      },
    }));
  }
}

function scheduleNextRefresh(cfg: AppConfig): void {
  const now = new Date();
  lastRefresh.set(now);
  nextRefresh.set(new Date(now.getTime() + cfg.refresh_interval_secs * 1000));
  refreshCountdown.set(cfg.refresh_interval_secs);
}
