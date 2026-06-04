<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import { cameraImages } from "$lib/stores/cameras";
  import type { CameraInfo } from "$lib/api";

  interface Props {
    camera: CameraInfo;
    aspectRatio: [number, number];
    showTitle: boolean;
  }

  let { camera, aspectRatio, showTitle }: Props = $props();

  const dispatch = createEventDispatcher<{ action: { type: "refresh" | "hide"; id: number } }>();

  let contextMenuVisible = $state(false);
  let menuX = $state(0);
  let menuY = $state(0);

  const imageState = $derived($cameraImages[camera.id] ?? { status: "idle" as const });

  function paddingTop(): string {
    return `${(aspectRatio[1] / aspectRatio[0]) * 100}%`;
  }

  function onContextMenu(e: MouseEvent): void {
    e.preventDefault();
    menuX = e.clientX;
    menuY = e.clientY;
    contextMenuVisible = true;
  }

  function closeMenu(): void {
    contextMenuVisible = false;
  }

  function handleRefresh(): void {
    closeMenu();
    dispatch("action", { type: "refresh", id: camera.id });
  }

  function handleHide(): void {
    closeMenu();
    dispatch("action", { type: "hide", id: camera.id });
  }
</script>

<svelte:window onclick={closeMenu} />

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="cell" oncontextmenu={onContextMenu}>
  {#if showTitle}
    <div class="title">
      <span class="locality">{camera.locality}</span>
      {#if camera.name && camera.name !== camera.locality}
        <span class="name">{camera.name}</span>
      {/if}
    </div>
  {/if}

  <div class="image-wrap" style="padding-top: {paddingTop()}">
    {#if imageState.status === "idle"}
      <div class="placeholder idle"></div>
    {:else if imageState.status === "loading"}
      {#if imageState.prev_src}
        <img src={imageState.prev_src} alt={camera.locality} class="img faded" />
      {:else}
        <div class="placeholder loading"></div>
      {/if}
      <div class="spinner-overlay">
        <div class="spinner"></div>
      </div>
    {:else if imageState.status === "ready" && imageState.src}
      <img src={imageState.src} alt={camera.locality} class="img" />
    {:else if imageState.status === "error"}
      <div class="placeholder error">
        <span>{imageState.error ?? "Fetch failed"}</span>
      </div>
    {/if}
  </div>
</div>

{#if contextMenuVisible}
  <div
    class="context-menu"
    style="left: {menuX}px; top: {menuY}px"
    role="menu"
    tabindex="-1"
  >
    <button onclick={handleRefresh} role="menuitem">Refresh</button>
    <button onclick={handleHide} role="menuitem">Hide camera</button>
  </div>
{/if}

<style>
  .cell {
    display: flex;
    flex-direction: column;
    user-select: none;
  }

  .title {
    display: flex;
    flex-direction: column;
    padding: 2px 4px;
    font-size: var(--title-size, 12px);
    color: var(--title-color, #ddd);
    text-align: var(--title-align, left);
    line-height: 1.3;
    overflow: hidden;
  }

  .locality {
    font-weight: 600;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .name {
    font-size: 0.85em;
    opacity: 0.8;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .image-wrap {
    position: relative;
    width: 100%;
    overflow: hidden;
  }

  .img {
    position: absolute;
    inset: 0;
    width: 100%;
    height: 100%;
    object-fit: cover;
    display: block;
  }

  .img.faded {
    opacity: 0.6;
  }

  .placeholder {
    position: absolute;
    inset: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 12px;
    text-align: center;
    padding: 8px;
  }

  .placeholder.idle {
    background: #2a2a2a;
  }

  .placeholder.loading {
    background: #2a2a2a;
    animation: pulse 1.2s ease-in-out infinite;
  }

  .placeholder.error {
    background: #3a0000;
    color: #ff8080;
    word-break: break-word;
  }

  .spinner-overlay {
    position: absolute;
    bottom: 6px;
    right: 6px;
    pointer-events: none;
  }

  .spinner {
    width: 16px;
    height: 16px;
    border: 2px solid rgba(255,255,255,0.3);
    border-top-color: #fff;
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  .context-menu {
    position: fixed;
    z-index: 1000;
    background: #2a2a2a;
    border: 1px solid #444;
    border-radius: 4px;
    overflow: hidden;
    box-shadow: 0 4px 12px rgba(0,0,0,0.5);
    min-width: 140px;
  }

  .context-menu button {
    display: block;
    width: 100%;
    padding: 8px 16px;
    text-align: left;
    background: none;
    border: none;
    color: #e0e0e0;
    font-size: 13px;
    white-space: nowrap;
  }

  .context-menu button:hover {
    background: #3a3a3a;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }

  @keyframes pulse {
    0%, 100% { opacity: 0.5; }
    50% { opacity: 0.8; }
  }
</style>
