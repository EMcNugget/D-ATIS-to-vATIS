<script setup lang="ts">
import Settings from "./settings/Settings.vue";
import EditContraction from "./settings/EditContraction.vue";
import FacilityConfig from "./settings/FacilityConfig.vue";
</script>

<template>
  <dialog class="modal" id="modal_1">
    <div class="modal-box">
      <form method="dialog">
        <button class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2">
          ✕
        </button>
      </form>
      <div role="tablist" class="tabs tabs-lifted">
        <input
          type="radio"
          name="my_tabs_2"
          role="tab"
          class="tab"
          checked
          aria-label="Settings"
        />
        <div
          role="tabpanel"
          class="tab-content border-base-300 rounded-box p-4"
        >
          <Settings />
        </div>
        <input
          type="radio"
          name="my_tabs_2"
          role="tab"
          class="tab"
          aria-label="Contractions"
        />
        <div
          role="tabpanel"
          class="tab-content border-base-300 rounded-box p-4"
        >
          <EditContraction />
        </div>
        <input
          type="radio"
          name="my_tabs_2"
          role="tab"
          class="tab whitespace-nowrap"
          aria-label="Facility Config"
        />
        <div
          role="tabpanel"
          class="tab-content border-base-300 rounded-box p-6"
        >
          <FacilityConfig />
        </div>
      </div>
    </div>
  </dialog>
</template>
