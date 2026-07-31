import { getCameraList } from "../services/fetcher";

export async function handleCameras(url: URL): Promise<Response> {
  const force = url.searchParams.has("refresh");
  try {
    const cameras = await getCameraList(force);
    return Response.json(cameras);
  } catch (err) {
    return Response.json({ error: String(err) }, { status: 502 });
  }
}
