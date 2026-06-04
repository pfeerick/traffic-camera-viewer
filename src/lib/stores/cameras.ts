import { writable, derived, get } from "svelte/store";
import { api } from "$lib/api";
import type { CameraInfo } from "$lib/api";
import { appConfig } from "./config";

export interface CameraImageState {
  status: "idle" | "loading" | "ready" | "error";
  src?: string;
  prev_src?: string;
  hash?: number;
  error?: string;
}

export const allCameras = writable<CameraInfo[]>([]);
export const cameraImages = writable<Record<number, CameraImageState>>({});

export const allDistricts = derived(allCameras, ($cams) =>
  [...new Set($cams.map((c) => c.district))].sort()
);

export const visibleCameras = derived(
  [allCameras, appConfig],
  ([$cams, $cfg]) => {
    if (!$cfg) return [];
    const selectedSet = new Set($cfg.selected_districts);
    const hiddenSet = new Set($cfg.hidden_camera_ids);
    let filtered = $cams.filter(
      (c) => selectedSet.has(c.district) && !hiddenSet.has(c.id)
    );
    if ($cfg.camera_order.length > 0) {
      const orderMap = new Map(
        $cfg.camera_order.map((id, i) => [id, i] as [number, number])
      );
      filtered = [...filtered].sort(
        (a, b) =>
          (orderMap.get(a.id) ?? Infinity) - (orderMap.get(b.id) ?? Infinity)
      );
    }
    return filtered;
  }
);

export async function loadCameras(): Promise<void> {
  const cameras = await api.getCameraList();
  allCameras.set(cameras);
  cameraImages.set(
    Object.fromEntries(cameras.map((c) => [c.id, { status: "idle" as const }]))
  );
}

export async function refreshCameraList(): Promise<void> {
  const cameras = await api.refreshCameraList();
  allCameras.set(cameras);
}

export function hideCameraById(id: number): void {
  allCameras.update((cams) => cams.filter((c) => c.id !== id));
}

export function getCameraById(id: number): CameraInfo | undefined {
  return get(allCameras).find((c) => c.id === id);
}
