<script lang="ts">
  import { pendingConfig } from "$lib/stores/config";
  import type { TitleAlign } from "$lib/api";

  const ASPECT_RATIOS: [number, number][] = [
    [5, 4],
    [4, 3],
    [16, 9],
  ];

  function ratioLabel(r: [number, number]): string {
    return `${r[0]}:${r[1]}`;
  }

  function ratioMatches(a: [number, number], b: [number, number]): boolean {
    return a[0] === b[0] && a[1] === b[1];
  }

  function setAlign(align: TitleAlign) {
    pendingConfig.update((cfg) => (cfg ? { ...cfg, camera_title_align: align } : cfg));
  }

  function setTitleRgbChannel(channel: 0 | 1 | 2, val: number) {
    pendingConfig.update((cfg) => {
      if (!cfg) return cfg;
      const rgb: [number, number, number] = [...cfg.camera_title_rgb] as [number, number, number];
      rgb[channel] = val;
      return { ...cfg, camera_title_rgb: rgb };
    });
  }

  function setBgRgbChannel(channel: 0 | 1 | 2, val: number) {
    pendingConfig.update((cfg) => {
      if (!cfg) return cfg;
      const rgb: [number, number, number] = [...cfg.app_background_rgb] as [number, number, number];
      rgb[channel] = val;
      return { ...cfg, app_background_rgb: rgb };
    });
  }
</script>

<details class="section">
  <summary class="section-title">Display</summary>
  {#if $pendingConfig}
    {@const cfg = $pendingConfig}
    <div class="section-body">

      <!-- Refresh interval -->
      <div class="row">
        <span class="lbl">Refresh interval</span>
        <div class="input-group">
          <input
            type="range" min="10" max="300" step="10"
            value={cfg.refresh_interval_secs}
            oninput={(e) =>
              pendingConfig.update((c) =>
                c ? { ...c, refresh_interval_secs: +e.currentTarget.value } : c
              )}
          />
          <span class="val">{cfg.refresh_interval_secs}s</span>
        </div>
      </div>

      <!-- Column count -->
      <div class="row">
        <span class="lbl">Columns</span>
        <div class="input-group">
          <input
            type="range" min="1" max="6" step="1"
            value={cfg.column_count}
            oninput={(e) =>
              pendingConfig.update((c) =>
                c ? { ...c, column_count: +e.currentTarget.value } : c
              )}
          />
          <span class="val">{cfg.column_count}</span>
        </div>
      </div>

      <!-- Grid spacing -->
      <div class="row">
        <span class="lbl">Grid spacing</span>
        <div class="input-group">
          <input
            type="range" min="0" max="16" step="1"
            value={cfg.grid_spacing}
            oninput={(e) =>
              pendingConfig.update((c) =>
                c ? { ...c, grid_spacing: +e.currentTarget.value } : c
              )}
          />
          <span class="val">{cfg.grid_spacing}px</span>
        </div>
      </div>

      <!-- Aspect ratio -->
      <div class="row">
        <span class="lbl">Aspect ratio</span>
        <div class="ratio-btns">
          {#each ASPECT_RATIOS as ratio}
            <button
              class="ratio-btn"
              class:active={ratioMatches(cfg.camera_aspect_ratio as [number,number], ratio)}
              onclick={() =>
                pendingConfig.update((c) =>
                  c ? { ...c, camera_aspect_ratio: ratio } : c
                )}
            >{ratioLabel(ratio)}</button>
          {/each}
        </div>
      </div>

      <!-- Camera titles -->
      <div class="row">
        <span class="lbl">Camera titles</span>
        <input
          type="checkbox"
          checked={cfg.show_camera_titles}
          onchange={(e) =>
            pendingConfig.update((c) =>
              c ? { ...c, show_camera_titles: e.currentTarget.checked } : c
            )}
        />
      </div>

      {#if cfg.show_camera_titles}
        <!-- Title font size -->
        <div class="row">
          <span class="lbl">Title size</span>
          <div class="input-group">
            <input
              type="range" min="8" max="24" step="1"
              value={cfg.camera_title_font_size}
              oninput={(e) =>
                pendingConfig.update((c) =>
                  c ? { ...c, camera_title_font_size: +e.currentTarget.value } : c
                )}
            />
            <span class="val">{cfg.camera_title_font_size}pt</span>
          </div>
        </div>

        <!-- Title alignment -->
        <div class="row">
          <span class="lbl">Title align</span>
          <div class="ratio-btns">
            {#each (["left", "center", "right"] as TitleAlign[]) as align}
              <button
                class="ratio-btn"
                class:active={cfg.camera_title_align === align}
                onclick={() => setAlign(align)}
              >{align[0].toUpperCase()}</button>
            {/each}
          </div>
        </div>

        <!-- Title color -->
        <div class="row">
          <span class="lbl">Title color</span>
          <div class="color-group">
            <div class="swatch" style="background: rgb({cfg.camera_title_rgb[0]},{cfg.camera_title_rgb[1]},{cfg.camera_title_rgb[2]})"></div>
            <div class="rgb-sliders">
              <input type="range" min="0" max="255" value={cfg.camera_title_rgb[0]}
                oninput={(e) => setTitleRgbChannel(0, +e.currentTarget.value)} title="Red" />
              <input type="range" min="0" max="255" value={cfg.camera_title_rgb[1]}
                oninput={(e) => setTitleRgbChannel(1, +e.currentTarget.value)} title="Green" />
              <input type="range" min="0" max="255" value={cfg.camera_title_rgb[2]}
                oninput={(e) => setTitleRgbChannel(2, +e.currentTarget.value)} title="Blue" />
            </div>
          </div>
        </div>
      {/if}

      <!-- Background color -->
      <div class="row">
        <span class="lbl">Background</span>
        <div class="color-group">
          <div class="swatch" style="background: rgb({cfg.app_background_rgb[0]},{cfg.app_background_rgb[1]},{cfg.app_background_rgb[2]})"></div>
          <div class="rgb-sliders">
            <input type="range" min="0" max="255" value={cfg.app_background_rgb[0]}
              oninput={(e) => setBgRgbChannel(0, +e.currentTarget.value)} title="Red" />
            <input type="range" min="0" max="255" value={cfg.app_background_rgb[1]}
              oninput={(e) => setBgRgbChannel(1, +e.currentTarget.value)} title="Green" />
            <input type="range" min="0" max="255" value={cfg.app_background_rgb[2]}
              oninput={(e) => setBgRgbChannel(2, +e.currentTarget.value)} title="Blue" />
          </div>
        </div>
      </div>

    </div>
  {/if}
</details>

<style>
  .section { border-bottom: 1px solid #333; }

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

  .section-title::-webkit-details-marker { display: none; }

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
    white-space: nowrap;
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
    min-width: 28px;
    text-align: right;
  }

  .ratio-btns {
    display: flex;
    gap: 4px;
  }

  .ratio-btn {
    background: #2a2a2a;
    border: 1px solid #444;
    color: #aaa;
    border-radius: 4px;
    padding: 3px 8px;
    font-size: 11px;
  }

  .ratio-btn.active {
    background: #1a4080;
    border-color: #4a9eff;
    color: #fff;
  }

  .color-group {
    display: flex;
    align-items: center;
    gap: 8px;
    flex: 1;
  }

  .swatch {
    width: 20px;
    height: 20px;
    border-radius: 4px;
    border: 1px solid #444;
    flex-shrink: 0;
  }

  .rgb-sliders {
    display: flex;
    flex-direction: column;
    gap: 2px;
    flex: 1;
  }

  .rgb-sliders input[type="range"] {
    width: 100%;
  }
</style>
