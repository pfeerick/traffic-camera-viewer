<script lang="ts">
  import { pendingConfig } from "$lib/stores/config";
  import { allCameras } from "$lib/stores/cameras";
  import type { CameraInfo } from "$lib/api";

  // Build ordered camera list for the settings panel — shows all cameras in selected districts
  // (including hidden ones, so they can be re-enabled). The main grid excludes hidden cameras.
  function orderedCameras(
    cameras: CameraInfo[],
    order: number[],
    selectedDistricts: string[],
  ): CameraInfo[] {
    const districtSet = new Set(selectedDistricts);
    const orderMap = new Map(order.map((id, i) => [id, i] as [number, number]));
    const available = cameras.filter((c) => districtSet.has(c.district));
    return [...available].sort(
      (a, b) => (orderMap.get(a.id) ?? Infinity) - (orderMap.get(b.id) ?? Infinity),
    );
  }

  function moveUp(id: number) {
    pendingConfig.update((cfg) => {
      if (!cfg) return cfg;
      const cameras = orderedCameras($allCameras, cfg.camera_order, [...cfg.selected_districts]);
      const ids = cameras.map((c) => c.id);
      const idx = ids.indexOf(id);
      if (idx <= 0) return cfg;
      [ids[idx - 1], ids[idx]] = [ids[idx], ids[idx - 1]];
      return { ...cfg, camera_order: ids };
    });
  }

  function moveDown(id: number) {
    pendingConfig.update((cfg) => {
      if (!cfg) return cfg;
      const cameras = orderedCameras($allCameras, cfg.camera_order, [...cfg.selected_districts]);
      const ids = cameras.map((c) => c.id);
      const idx = ids.indexOf(id);
      if (idx < 0 || idx >= ids.length - 1) return cfg;
      [ids[idx], ids[idx + 1]] = [ids[idx + 1], ids[idx]];
      return { ...cfg, camera_order: ids };
    });
  }

  function toggleHide(id: number, hide: boolean) {
    pendingConfig.update((cfg) => {
      if (!cfg) return cfg;
      const hiddenSet = new Set(cfg.hidden_camera_ids);
      if (hide) hiddenSet.add(id);
      else hiddenSet.delete(id);
      return { ...cfg, hidden_camera_ids: [...hiddenSet] };
    });
  }
</script>

<details class="section">
  <summary class="section-title">Cameras</summary>
  <div class="section-body">
    {#if $pendingConfig}
      {@const cfg = $pendingConfig}
      {@const cameras = orderedCameras($allCameras, cfg.camera_order, [...cfg.selected_districts])}
      {#if cameras.length === 0}
        <p class="hint">No cameras match the selected districts.</p>
      {:else}
        {#each cameras as camera, idx}
          {@const isHidden = cfg.hidden_camera_ids.includes(camera.id)}
          <div class="camera-row" class:hidden-cam={isHidden}>
            <div class="cam-info">
              <span class="cam-name">{camera.locality}</span>
              {#if camera.district !== camera.locality}
                <span class="cam-district">{camera.district}</span>
              {/if}
            </div>
            <div class="cam-actions">
              <button
                class="icon-btn"
                onclick={() => moveUp(camera.id)}
                disabled={idx === 0}
                title="Move up">▲</button
              >
              <button
                class="icon-btn"
                onclick={() => moveDown(camera.id)}
                disabled={idx === cameras.length - 1}
                title="Move down">▼</button
              >
              <input
                type="checkbox"
                checked={!isHidden}
                title="Show/hide"
                onchange={(e) => toggleHide(camera.id, !e.currentTarget.checked)}
              />
            </div>
          </div>
        {/each}
      {/if}
    {/if}
  </div>
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
    gap: 2px;
    max-height: 280px;
    overflow-y: auto;
  }

  .camera-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 3px 0;
    gap: 4px;
  }

  .camera-row.hidden-cam {
    opacity: 0.5;
  }

  .cam-info {
    flex: 1;
    overflow: hidden;
    display: flex;
    flex-direction: column;
  }

  .cam-name {
    font-size: 12px;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .cam-district {
    font-size: 10px;
    color: #888;
  }

  .cam-actions {
    display: flex;
    align-items: center;
    gap: 4px;
    flex-shrink: 0;
  }

  .icon-btn {
    background: none;
    border: 1px solid #444;
    color: #aaa;
    padding: 1px 5px;
    font-size: 10px;
    border-radius: 3px;
  }

  .icon-btn:hover:not(:disabled) {
    color: #fff;
    border-color: #888;
  }

  .icon-btn:disabled {
    opacity: 0.3;
    cursor: default;
  }

  .hint {
    font-size: 12px;
    color: #666;
  }
</style>
