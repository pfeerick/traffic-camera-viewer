<script lang="ts">
  import { applyConfig, cancelConfig, pendingConfig } from "$lib/stores/config";
  import { loadCameras } from "$lib/stores/cameras";
  import { triggerRefreshAll } from "$lib/stores/refresh";
  import DistrictsSection from "./settings/DistrictsSection.svelte";
  import CamerasSection from "./settings/CamerasSection.svelte";
  import DisplaySection from "./settings/DisplaySection.svelte";
  import DiskSaveSection from "./settings/DiskSaveSection.svelte";

  interface Props {
    onclose: () => void;
  }

  let { onclose }: Props = $props();

  let applying = $state(false);

  async function handleApply() {
    applying = true;
    try {
      await applyConfig();
      await loadCameras();
      await triggerRefreshAll();
    } finally {
      applying = false;
      onclose();
    }
  }

  function handleCancel() {
    cancelConfig();
    onclose();
  }
</script>

<div class="settings-panel">
  <div class="panel-header">
    <h2>Settings</h2>
    <button class="close-btn" onclick={onclose} aria-label="Close">✕</button>
  </div>

  <div class="panel-body">
    {#if $pendingConfig}
      <DistrictsSection />
      <CamerasSection />
      <DisplaySection />
      <DiskSaveSection />
    {/if}
  </div>

  <div class="panel-footer">
    <button class="btn btn-primary" onclick={handleApply} disabled={applying}>
      {applying ? "Applying…" : "Apply"}
    </button>
    <button class="btn" onclick={handleCancel}>Cancel</button>
  </div>
</div>

<style>
  .settings-panel {
    width: 300px;
    min-width: 280px;
    display: flex;
    flex-direction: column;
    background: #1e1e1e;
    border-left: 1px solid #333;
    overflow: hidden;
  }

  .panel-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 10px 14px;
    border-bottom: 1px solid #333;
    flex-shrink: 0;
  }

  h2 {
    font-size: 14px;
    font-weight: 600;
    color: #e0e0e0;
  }

  .close-btn {
    background: none;
    border: none;
    color: #888;
    font-size: 16px;
    padding: 2px 6px;
  }

  .close-btn:hover {
    color: #e0e0e0;
  }

  .panel-body {
    flex: 1;
    overflow-y: auto;
    padding: 8px 0;
  }

  .panel-footer {
    display: flex;
    gap: 8px;
    padding: 10px 14px;
    border-top: 1px solid #333;
    flex-shrink: 0;
  }

  .btn {
    flex: 1;
    padding: 6px 12px;
    border-radius: 4px;
    border: 1px solid #444;
    background: #2a2a2a;
    color: #e0e0e0;
    font-size: 13px;
  }

  .btn:hover:not(:disabled) {
    background: #3a3a3a;
  }

  .btn-primary {
    background: #1a4080;
    border-color: #4a9eff;
    color: #fff;
  }

  .btn-primary:hover:not(:disabled) {
    background: #1a50a0;
  }

  .btn-primary:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }
</style>
