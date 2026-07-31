<script lang="ts">
  import { pendingConfig } from "$lib/stores/config";
  import { api, isTauri } from "$lib/api";
  import type { CacheInfo } from "$lib/api";

  let cacheInfo = $state<CacheInfo | null>(null);
  let clearing = $state(false);

  async function loadCacheInfo() {
    const cfg = $pendingConfig;
    if (!cfg?.save_path) return;
    try {
      cacheInfo = await api.getCacheInfo(cfg.save_path);
    } catch {
      cacheInfo = null;
    }
  }

  async function browsePath() {
    if (!isTauri) return;
    try {
      const { open } = await import("@tauri-apps/plugin-dialog");
      const dir = await open({ directory: true, multiple: false });
      if (dir && typeof dir === "string") {
        pendingConfig.update((cfg) => (cfg ? { ...cfg, save_path: dir } : cfg));
      }
    } catch (err) {
      console.error("Browse failed:", err);
    }
  }

  async function clearCache() {
    const cfg = $pendingConfig;
    if (!cfg?.save_path) return;
    clearing = true;
    try {
      await api.clearCache(cfg.save_path);
      cacheInfo = null;
    } finally {
      clearing = false;
    }
  }

  function formatBytes(bytes: number): string {
    if (bytes < 1024) return `${bytes} B`;
    if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
    return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
  }
</script>

<details
  class="section"
  ontoggle={(e) => {
    if (e.currentTarget.open) loadCacheInfo();
  }}
>
  <summary class="section-title">Disk Save</summary>
  {#if $pendingConfig}
    {@const cfg = $pendingConfig}
    <div class="section-body">
      <div class="row">
        <span class="lbl">Enable</span>
        <input
          type="checkbox"
          checked={cfg.save_to_disk}
          onchange={(e) =>
            pendingConfig.update((c) => (c ? { ...c, save_to_disk: e.currentTarget.checked } : c))}
        />
      </div>

      {#if cfg.save_to_disk}
        <div class="path-row">
          <input
            type="text"
            class="path-input"
            value={cfg.save_path}
            oninput={(e) =>
              pendingConfig.update((c) => (c ? { ...c, save_path: e.currentTarget.value } : c))}
            placeholder="Save directory…"
          />
          {#if isTauri}
            <button class="btn" onclick={browsePath}>Browse…</button>
          {/if}
        </div>

        <div class="row">
          <span class="lbl">Max snapshots</span>
          <div class="input-group">
            <input
              type="range"
              min="1"
              max="20"
              step="1"
              value={cfg.max_snapshots}
              oninput={(e) =>
                pendingConfig.update((c) =>
                  c ? { ...c, max_snapshots: +e.currentTarget.value } : c,
                )}
            />
            <span class="val">{cfg.max_snapshots}</span>
          </div>
        </div>

        {#if cacheInfo?.exists}
          <p class="cache-info">
            {cacheInfo.file_count} files · {formatBytes(cacheInfo.total_bytes)}
          </p>
          <button class="btn danger" onclick={clearCache} disabled={clearing}>
            {clearing ? "Clearing…" : "Clear Cache"}
          </button>
        {/if}
      {/if}
    </div>
  {/if}
</details>

<style>
  .section {
    border-bottom: 1px solid #333;
  }

  .section-title {
    padding: 8px 14px;
    font-size: 12px;
    font-weight: 600;
    color: #aaa;
    text-transform: uppercase;
    letter-spacing: 0.5px;
    cursor: pointer;
    user-select: none;
    list-style: none;
  }

  .section-title::-webkit-details-marker {
    display: none;
  }

  .section-body {
    padding: 4px 14px 10px;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 8px;
  }

  .row .lbl {
    font-size: 12px;
    color: #aaa;
    min-width: 90px;
  }

  .input-group {
    display: flex;
    align-items: center;
    gap: 6px;
    flex: 1;
  }

  .input-group input[type="range"] {
    flex: 1;
    min-width: 0;
  }

  .val {
    font-size: 11px;
    color: #888;
    min-width: 20px;
    text-align: right;
  }

  .path-row {
    display: flex;
    gap: 6px;
    align-items: center;
  }

  .path-input {
    flex: 1;
    font-size: 12px;
    min-width: 0;
  }

  .btn {
    background: #2a2a2a;
    border: 1px solid #444;
    color: #e0e0e0;
    border-radius: 4px;
    padding: 4px 10px;
    font-size: 12px;
    white-space: nowrap;
    cursor: pointer;
  }

  .btn:hover:not(:disabled) {
    background: #3a3a3a;
  }
  .btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .btn.danger {
    border-color: #804040;
    color: #ff8080;
  }

  .btn.danger:hover:not(:disabled) {
    background: #3a1a1a;
  }

  .cache-info {
    font-size: 11px;
    color: #888;
  }
</style>
