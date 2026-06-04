<script lang="ts">
  import { appConfig } from "$lib/stores/config";
  import { visibleCameras } from "$lib/stores/cameras";
  import { triggerRefreshSingle } from "$lib/stores/refresh";
  import CameraCell from "./CameraCell.svelte";
  import { appConfig as cfgStore } from "$lib/stores/config";

  function handleAction(event: CustomEvent<{ type: "refresh" | "hide"; id: number }>) {
    const { type, id } = event.detail;
    if (type === "refresh") {
      void triggerRefreshSingle(id);
    } else if (type === "hide") {
      // Add to hidden_camera_ids via pending config update applied immediately
      cfgStore.update((cfg) => {
        if (!cfg) return cfg;
        const hidden = [...cfg.hidden_camera_ids, id];
        return { ...cfg, hidden_camera_ids: hidden };
      });
    }
  }
</script>

{#if $appConfig}
  {@const cfg = $appConfig}
  {@const cameras = $visibleCameras}
  {@const cols = cfg.column_count}
  {@const gap = cfg.grid_spacing}
  <div
    class="grid-container"
    style="
      --cols: {cols};
      --gap: {gap}px;
      --title-color: rgb({cfg.camera_title_rgb[0]},{cfg.camera_title_rgb[1]},{cfg.camera_title_rgb[2]});
      --title-size: {cfg.camera_title_font_size}px;
      --title-align: {cfg.camera_title_align};
    "
  >
    {#if cameras.length === 0}
      <div class="empty-state">
        No cameras to display. Check your district filter in Settings.
      </div>
    {:else}
      <div class="grid" style="grid-template-columns: repeat({cols}, 1fr); gap: {gap}px;">
        {#each cameras as camera (camera.id)}
          <CameraCell
            {camera}
            aspectRatio={cfg.camera_aspect_ratio}
            showTitle={cfg.show_camera_titles}
            on:action={handleAction}
          />
        {/each}
      </div>
    {/if}
  </div>
{/if}

<style>
  .grid-container {
    flex: 1;
    overflow-y: auto;
    overflow-x: hidden;
    padding: var(--gap);
  }

  .grid {
    display: grid;
    width: 100%;
  }

  .empty-state {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 200px;
    color: #888;
    font-size: 14px;
  }
</style>
