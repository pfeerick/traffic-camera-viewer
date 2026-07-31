import { saveRolling } from "../services/disk";
import { fetchCameraImage } from "../services/fetcher";

export async function handleImage(url: URL): Promise<Response> {
  const imageUrl = url.searchParams.get("image_url");
  const saveToDisk = url.searchParams.get("save_to_disk") === "true";
  const savePath = url.searchParams.get("save_path") ?? "";
  const maxSnapshots = parseInt(url.searchParams.get("max_snapshots") ?? "5", 10);
  const prevHashStr = url.searchParams.get("previous_hash");
  const previousHash = prevHashStr ? parseInt(prevHashStr, 10) : null;

  if (!imageUrl) {
    return Response.json({ error: "missing image_url" }, { status: 400 });
  }

  try {
    const result = await fetchCameraImage(imageUrl, previousHash);

    if (result.type === "unchanged") {
      return Response.json({ type: "unchanged", hash: result.hash });
    }

    const bytes = result.bytes;

    if (saveToDisk && savePath) {
      // Fire and forget
      saveRolling(bytes, imageUrl, savePath, maxSnapshots).catch((e) =>
        console.warn("Disk save failed:", e),
      );
    }

    const jpeg_b64 = Buffer.from(bytes).toString("base64");
    return Response.json({ type: "changed", hash: result.hash, jpeg_b64 });
  } catch (err) {
    return Response.json({ error: String(err) }, { status: 502 });
  }
}
