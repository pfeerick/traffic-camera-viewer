import type { CameraInfo } from "./types";

const GEOJSON_URL = "https://data.qldtraffic.qld.gov.au/webcameras.geojson";

interface GeoJsonRoot {
  features: Array<{
    properties: {
      id: number;
      description: string;
      district: string;
      locality: string;
      image_url: string;
    };
  }>;
}

// In-memory cache — refreshed on demand or when stale.
let cachedCameras: CameraInfo[] | null = null;
let cacheTs = 0;
const CACHE_TTL_MS = 5 * 60 * 1000; // 5 minutes

export async function getCameraList(force = false): Promise<CameraInfo[]> {
  const now = Date.now();
  if (!force && cachedCameras && now - cacheTs < CACHE_TTL_MS) {
    return cachedCameras;
  }

  const res = await fetch(GEOJSON_URL);
  if (!res.ok) throw new Error(`GeoJSON fetch failed: ${res.status}`);
  const root: GeoJsonRoot = (await res.json()) as GeoJsonRoot;

  cachedCameras = root.features.map((f) => ({
    id: f.properties.id,
    name: f.properties.description,
    district: f.properties.district,
    locality: f.properties.locality,
    image_url: f.properties.image_url,
  }));
  cacheTs = now;
  return cachedCameras;
}

export type ImageFetchResult =
  | { type: "unchanged"; hash: number }
  | { type: "changed"; hash: number; bytes: Uint8Array };

export async function fetchCameraImage(
  imageUrl: string,
  previousHash: number | null,
): Promise<ImageFetchResult> {
  const ts = Date.now();
  const url = `${imageUrl}?${ts}`;

  const res = await fetch(url);
  if (!res.ok) throw new Error(`Image fetch failed: ${res.status}`);

  const buffer = await res.arrayBuffer();
  const bytes = new Uint8Array(buffer);

  // murmur32v3 returns a plain JS number (safe to JSON-serialize); Bun.hash() returns bigint
  const hash = Bun.hash.murmur32v3(bytes);

  if (previousHash !== null && previousHash === hash) {
    return { type: "unchanged", hash };
  }

  return { type: "changed", hash, bytes };
}
