import { type AppConfig, loadConfig, saveConfig } from "../services/config";

export async function handleConfig(req: Request): Promise<Response> {
  if (req.method === "GET") {
    try {
      const cfg = await loadConfig();
      return Response.json(cfg);
    } catch (err) {
      return Response.json({ error: String(err) }, { status: 500 });
    }
  }

  if (req.method === "POST") {
    try {
      const cfg = await req.json() as AppConfig;
      await saveConfig(cfg);
      return Response.json({ ok: true });
    } catch (err) {
      return Response.json({ error: String(err) }, { status: 400 });
    }
  }

  return new Response("Method not allowed", { status: 405 });
}
