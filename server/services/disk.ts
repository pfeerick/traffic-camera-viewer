import { join } from "node:path";
import { mkdir, readdir, stat, unlink, writeFile } from "node:fs/promises";

function urlStem(imageUrl: string): string {
  const path = imageUrl.split("?")[0] ?? imageUrl;
  const filename = path.split("/").at(-1) ?? "camera";
  return filename.endsWith(".jpg") ? filename.slice(0, -4) : filename;
}

export async function saveRolling(
  bytes: Uint8Array,
  imageUrl: string,
  savePath: string,
  maxSnapshots: number
): Promise<void> {
  await mkdir(savePath, { recursive: true });

  const stem = urlStem(imageUrl);
  const ts = Math.floor(Date.now() / 1000);
  await writeFile(join(savePath, `${stem}_${ts}.jpg`), bytes);

  // Enforce rolling limit
  const prefix = `${stem}_`;
  let entries = await readdir(savePath);
  const matches = entries.filter(
    (e) => e.startsWith(prefix) && e.endsWith(".jpg")
  );

  if (matches.length > maxSnapshots) {
    // Sort by mtime ascending (oldest first)
    const withMtime = await Promise.all(
      matches.map(async (name) => {
        const s = await stat(join(savePath, name)).catch(() => null);
        return { name, mtime: s?.mtimeMs ?? 0 };
      })
    );
    withMtime.sort((a, b) => a.mtime - b.mtime);
    const toDelete = withMtime.slice(0, withMtime.length - maxSnapshots);
    await Promise.all(
      toDelete.map(({ name }) =>
        unlink(join(savePath, name)).catch(() => {})
      )
    );
  }
}

export async function getCacheInfo(
  savePath: string
): Promise<{ exists: boolean; file_count: number; total_bytes: number }> {
  try {
    const entries = await readdir(savePath);
    let total = 0;
    let count = 0;
    for (const name of entries) {
      const s = await stat(join(savePath, name)).catch(() => null);
      if (s?.isFile()) {
        count++;
        total += s.size;
      }
    }
    return { exists: true, file_count: count, total_bytes: total };
  } catch {
    return { exists: false, file_count: 0, total_bytes: 0 };
  }
}

export async function clearCache(savePath: string): Promise<void> {
  const entries = await readdir(savePath).catch(() => [] as string[]);
  await Promise.all(
    entries.map((name) =>
      unlink(join(savePath, name)).catch(() => {})
    )
  );
}
