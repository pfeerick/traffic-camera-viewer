import { clearCache, getCacheInfo } from "../services/disk";

export async function handleDiskInfo(url: URL): Promise<Response> {
  const path = url.searchParams.get("path") ?? "";
  if (!path) return Response.json({ error: "missing path" }, { status: 400 });
  try {
    const info = await getCacheInfo(path);
    return Response.json(info);
  } catch (err) {
    return Response.json({ error: String(err) }, { status: 500 });
  }
}

export async function handleDiskClear(req: Request): Promise<Response> {
  try {
    const { path } = (await req.json()) as { path: string };
    if (!path) return Response.json({ error: "missing path" }, { status: 400 });
    await clearCache(path);
    return Response.json({ ok: true });
  } catch (err) {
    return Response.json({ error: String(err) }, { status: 500 });
  }
}
