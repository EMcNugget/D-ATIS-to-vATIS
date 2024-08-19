<script setup lang="ts">
import { computed, ref, watch } from "vue";
import CLabel from "./CLabel.vue";
import Dropdown from "./Dropdown.vue";
import { use_store } from "../lib/stores";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { listen, TauriEvent } from "@tauri-apps/api/event";
import type { TAlert } from "../lib/types";

const props = defineProps<{
  showModal: boolean;
}>();
defineEmits(["close"]);

const store = use_store();

const open_path = () => {
  open({
    multiple: false,
    directory: true,
    filters: [],
  }).then((path) => {
    store.set_file_path(path ? (path as string) : store.get_file_path());
  });
};

const file_path = computed({
  get: () => store.get_file_path(),
  set: (v) => store.set_file_path(v),
});

const custom_path = computed({
  get: () => store.get_custom_path(),
  set: (v) => store.set_custom_path(v),
});

const save_facility = computed({
  get: () => store.get_save_facility(),
  set: (v) => store.set_save_facility(v),
});

const profile = computed({
  get: () => store.get_profile(),
  set: (v) => store.set_profile(v),
});

const alert = computed({
  get: () => store.get_alert(),
  set: (v) => store.set_alert(v),
});

const check_update = computed({
  get: () => store.get_check_update(),
  set: (v) => store.set_check_update(v),
});

const check_update_freq = computed({
  get: () => store.get_check_update_freq(),
  set: (v) => store.set_check_update_freq(v),
});

const update_time = computed({
  get: () => store.get_update_time(),
  set: (v) => store.set_update_time(v),
});

const open_vatis_on_fetch = computed({
  get: () => store.get_open_vatis_on_fetch(),
  set: (v) => store.set_open_vatis_on_fetch(v),
});

const showAlert = ref(false);
const showDropdown = ref(false);
const showDropdownInterval = ref(false);

const handle_theme = (v: string) => {
  showDropdown.value = false;
  switch (v) {
    case "system":
      theme.value = "system";
      break;
    case "light":
      theme.value = "light";
      break;
    case "dark":
      theme.value = "dark";
      break;
  }
};

const toggle_dropdown = () => {
  showDropdown.value = !showDropdown.value;
};

const toggle_dropdown_interval = () => {
  showDropdownInterval.value = !showDropdownInterval.value;
};

const handle_interval = (v: number) => {
  showDropdownInterval.value = false;
  update_time.value = v;
};

const window = getCurrentWindow();
const system_theme = ref(await window.theme());

await listen(TauriEvent.WINDOW_THEME_CHANGED, (event) => {
  system_theme.value = event.payload as "light" | "dark";
});

const theme = computed({
  get: () => store.get_theme(),
  set: (v) => store.set_theme(v),
});

watch(
  () => system_theme.value,
  () => {
    handle_theme(theme.value);
  }
);

watch(
  () => alert.value,
  () => {
    showAlert.value = true;
  }
);


const save_settings = () => {
  invoke("write_settings", {
    settings: store.get_all(),
  }).then((k) => {
    alert.value = k as TAlert;
  });
};
</script>

<template>
  <dialog class="modal" :open="props.showModal">
    <div class="modal-box">
      <form method="dialog">
        <button
          class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2"
          @click="$emit('close')"
        >
          ✕
        </button>
      </form>
      <h3 class="text-2xl mb-6 font-bold">Settings</h3>
      <input
        type="text"
        v-model="profile"
        placeholder="Profile..."
        class="input input-bordered w-full mr-4 mb-4"
      />
      <CLabel title="Custom vATIS Installation">
        <input type="checkbox" class="toggle" v-model="custom_path" />
      </CLabel>
      <div class="flex flex-row" v-if="custom_path">
        <input
          type="text"
          v-model="file_path"
          readonly
          placeholder="File Path..."
          class="input input-bordered w-full mr-4 mb-4"
        />
        <button class="btn" @click="open_path()">Browse</button>
      </div>
      <CLabel title="Save Facility">
        <input type="checkbox" class="toggle" v-model="save_facility" />
      </CLabel>
      <CLabel title="Check for ATIS Updates">
        <input type="checkbox" class="checkbox" v-model="check_update" />
      </CLabel>

      <div v-if="check_update">
        <CLabel title="Automatically Change Interval based on Zulu Time">
          <input type="checkbox" class="checkbox" v-model="check_update_freq" />
        </CLabel>
        <CLabel title="Update Interval">
          <Dropdown
            :name="`${update_time}m`"
            :click="toggle_dropdown_interval"
            :show="showDropdownInterval"
            :tab_index="1"
          >
            <li><a @click="handle_interval(15)">15m</a></li>
            <li><a @click="handle_interval(30)">30m</a></li>
            <li><a @click="handle_interval(45)">45m</a></li>
            <li><a @click="handle_interval(60)">60m</a></li>
          </Dropdown>
        </CLabel>
      </div>
      <CLabel title="Open vATIS on Fetch">
        <input type="checkbox" class="checkbox" v-model="open_vatis_on_fetch" />
      </CLabel>
      <CLabel title="Theme">
        <Dropdown
          :name="theme.charAt(0).toUpperCase() + theme.slice(1)"
          :click="toggle_dropdown"
          :show="showDropdown"
          :tab_index="0"
        >
          <li><a @click="handle_theme('system')">System</a></li>
          <li><a @click="handle_theme('light')">Light</a></li>
          <li><a @click="handle_theme('dark')">Dark</a></li>
        </Dropdown>
      </CLabel>
      <button
        class="btn btn-active btn-primary mt-8 w-full"
        @click="save_settings()"
      >
        Save
      </button>
    </div>
  </dialog>
</template>
