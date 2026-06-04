<script lang="ts">
  import { visibleCameras } from "$lib/stores/cameras";
  import { allDistricts } from "$lib/stores/cameras";
  import { appConfig } from "$lib/stores/config";
  import {
    lastRefresh,
    refreshCountdown,
    isLoadingCameras,
  } from "$lib/stores/refresh";

  interface Props {
    settingsOpen: boolean;
    onrefresh: () => void;
    onsettingstoggle: () => void;
  }

  let { settingsOpen, onrefresh, onsettingstoggle }: Props = $props();

  function formatTime(d: Date | null): string {
    if (!d) return "—";
    return d.toLocaleTimeString([], { hour: "2-digit", minute: "2-digit", second: "2-digit" });
  }

  function progressPct(): number {
    const cfg = $appConfig;
    if (!cfg || $refreshCountdown <= 0) return 100;
    return ((cfg.refresh_interval_secs - $refreshCountdown) / cfg.refresh_interval_secs) * 100;
  }

  let selectedDistrictCount = $derived(
    $appConfig ? $appConfig.selected_districts.length : 0
  );
</script>

<div class="status-bar">
  <button
    class="btn"
    onclick={onrefresh}
    disabled={$visibleCameras.length === 0}
  >
    Refresh Now
  </button>

  <span class="label">
    Last: {formatTime($lastRefresh)}
  </span>

  <div class="progress-wrap">
    {#if $isLoadingCameras}
      <div class="progress-bar indeterminate"></div>
      <span class="progress-label">Loading cameras…</span>
    {:else if $appConfig}
      <div class="progress-bar" style="width: {progressPct()}%"></div>
      <span class="progress-label">Next: {$refreshCountdown}s</span>
    {/if}
  </div>

  <span class="camera-count">
    {$visibleCameras.length} cameras
    {#if selectedDistrictCount > 0}
      | {selectedDistrictCount} district{selectedDistrictCount !== 1 ? "s" : ""}
    {/if}
  </span>

  <button
    class="btn settings-btn"
    class:active={settingsOpen}
    onclick={onsettingstoggle}
  >
    ⚙ Settings
  </button>
</div>

<style>
  .status-bar {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 6px 12px;
    background: #111;
    border-top: 1px solid #333;
    flex-shrink: 0;
    min-height: 36px;
    font-size: 13px;
  }

  .btn {
    background: #2a2a2a;
    color: #e0e0e0;
    border: 1px solid #444;
    border-radius: 4px;
    padding: 4px 10px;
    font-size: 13px;
    white-space: nowrap;
  }

  .btn:hover:not(:disabled) {
    background: #3a3a3a;
  }

  .btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .btn.active {
    background: #1a4080;
    border-color: #4a9eff;
    color: #fff;
  }

  .label {
    white-space: nowrap;
    color: #aaa;
  }

  .progress-wrap {
    flex: 1;
    position: relative;
    height: 18px;
    background: #2a2a2a;
    border-radius: 4px;
    overflow: hidden;
    min-width: 80px;
  }

  .progress-bar {
    position: absolute;
    left: 0;
    top: 0;
    height: 100%;
    background: #2a5080;
    transition: width 1s linear;
  }

  .progress-bar.indeterminate {
    width: 40%;
    animation: slide 1.5s ease-in-out infinite;
  }

  .progress-label {
    position: absolute;
    inset: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 11px;
    color: #ccc;
    pointer-events: none;
  }

  .camera-count {
    white-space: nowrap;
    color: #aaa;
    font-size: 12px;
  }

  .settings-btn {
    margin-left: auto;
  }

  @keyframes slide {
    0% { left: -40%; }
    100% { left: 100%; }
  }
</style>
