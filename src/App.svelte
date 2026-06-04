<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import "./app.css";
  import { isTauri } from "$lib/api";
  import { loadConfig, appConfig } from "$lib/stores/config";
  import { loadCameras } from "$lib/stores/cameras";
  import {
    startRefreshTimer,
    stopRefreshTimer,
    triggerRefreshAll,
    isLoadingCameras,
  } from "$lib/stores/refresh";
  import CameraGrid from "$lib/components/CameraGrid.svelte";
  import StatusBar from "$lib/components/StatusBar.svelte";
  import SettingsPanel from "$lib/components/SettingsPanel.svelte";

  let settingsOpen = $state(false);
  let initError = $state<string | null>(null);

  let bgStyle = $derived(
    $appConfig
      ? `background: rgb(${$appConfig.app_background_rgb[0]},${$appConfig.app_background_rgb[1]},${$appConfig.app_background_rgb[2]})`
      : "background: rgb(24,24,24)"
  );

  onMount(async () => {
    if (isTauri) {
      console.log("[app] running in Tauri desktop mode");
    } else {
      console.log("[app] running in web mode");
    }

    try {
      await loadConfig();
      isLoadingCameras.set(true);
      await loadCameras();
      isLoadingCameras.set(false);
      await triggerRefreshAll();
      startRefreshTimer();
    } catch (err) {
      initError = err instanceof Error ? err.message : String(err);
    } finally {
      if (isTauri) {
        try {
          const { getCurrentWindow } = await import("@tauri-apps/api/window");
          await getCurrentWindow().show();
        } catch (e) {
          console.error("[app] failed to show window:", e);
        }
      }
    }
  });

  onDestroy(() => {
    stopRefreshTimer();
  });
</script>

<div class="app-shell" style={bgStyle}>
  {#if initError}
    <div class="error-banner">Failed to initialise: {initError}</div>
  {/if}

  <div class="main-area">
    <CameraGrid />
    {#if settingsOpen}
      <SettingsPanel onclose={() => (settingsOpen = false)} />
    {/if}
  </div>

  <StatusBar
    onrefresh={() => triggerRefreshAll()}
    onsettingstoggle={() => (settingsOpen = !settingsOpen)}
    {settingsOpen}
  />
</div>

<style>
  .app-shell {
    height: 100%;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .main-area {
    flex: 1;
    display: flex;
    overflow: hidden;
    position: relative;
  }

  .error-banner {
    background: #8b0000;
    color: #fff;
    padding: 8px 16px;
    font-size: 13px;
    text-align: center;
  }
</style>
