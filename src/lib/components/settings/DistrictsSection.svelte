<script lang="ts">
  import { pendingConfig } from "$lib/stores/config";
  import { allDistricts } from "$lib/stores/cameras";

  function toggle(district: string, checked: boolean) {
    pendingConfig.update((cfg) => {
      if (!cfg) return cfg;
      const selected = new Set(cfg.selected_districts);
      if (checked) {
        selected.add(district);
      } else {
        selected.delete(district);
      }
      return { ...cfg, selected_districts: [...selected].sort() };
    });
  }
</script>

<details class="section" open>
  <summary class="section-title">Districts</summary>
  <div class="section-body">
    {#if $allDistricts.length === 0}
      <p class="hint">No camera data loaded yet.</p>
    {:else}
      {#each $allDistricts as district}
        {@const checked = $pendingConfig?.selected_districts.includes(district) ?? false}
        <label class="district-row">
          <input
            type="checkbox"
            {checked}
            onchange={(e) => toggle(district, e.currentTarget.checked)}
          />
          {district}
        </label>
      {/each}
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
    gap: 4px;
  }

  .district-row {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 13px;
    cursor: pointer;
    padding: 2px 0;
  }

  .district-row:hover {
    color: #fff;
  }

  .hint {
    font-size: 12px;
    color: #666;
  }
</style>
