<script setup lang="ts">
import { computed, ref, watch } from "vue";
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

const message = computed({
  get: () => store.get_message(),
  set: (v) => store.set_message(v),
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
  () => message.value,
  () => {
    showAlert.value = true;
  }
);

const save_settings = () => {
  invoke("write_settings", {
    settings: store.get_all(),
  }).then((k) => {
    message.value = k as TAlert;
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
      <label class="label cursor-pointer justify-start">
        <span class="label-text mr-6 text-base font-semibold"
          >Custom vATIS Installation</span
        >
        <input type="checkbox" class="toggle" v-model="custom_path" />
      </label>
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
      <label class="label cursor-pointer justify-start">
        <span class="label-text mr-6 text-base font-semibold"
          >Save Facility</span
        >
        <input type="checkbox" class="checkbox" v-model="save_facility" />
      </label>
      <label class="label cursor-pointer justify-start">
        <span class="label-text mr-6 text-base font-semibold"
          >Check for ATIS Updates</span
        >
        <input type="checkbox" class="checkbox" v-model="check_update" />
      </label>
      <div v-if="check_update">
        <label class="label cursor-pointer justify-start">
          <span class="label-text mr-6 text-base font-semibold"
            >Automatically Change Interval based on Zulu Time</span
          >
          <input type="checkbox" class="checkbox" v-model="check_update_freq" />
        </label>
        <label class="label cursor-pointer justify-start">
          <span class="label-text mr-6 text-base font-semibold"
            >Update Interval</span
          >
          <div class="dropdown">
            <label
              tabindex="1"
              class="btn m-1"
              @click="toggle_dropdown_interval"
            >
              {{ update_time }}m
              <img
                v-if="showDropdownInterval"
                src="/dropdown_up.svg"
                alt="Dropdown"
                class="h-auto w-auto max-h-6 max-w-6"
              />
              <img
                v-if="!showDropdownInterval"
                src="/dropdown_down.svg"
                alt="Dropdown"
                class="h-auto w-auto max-h-6 max-w-6"
              />
            </label>

            <ul
              v-if="showDropdownInterval"
              tabindex="1"
              class="dropdown-content menu p-2 shadow bg-base-200 rounded-box w-52 z-50"
            >
              <li><a @click="handle_interval(15)">15m</a></li>
              <li><a @click="handle_interval(30)">30m</a></li>
              <li><a @click="handle_interval(45)">45m</a></li>
              <li><a @click="handle_interval(60)">60m</a></li>
            </ul>
          </div>
        </label>
      </div>
      <label class="label cursor-pointer justify-start">
        <span class="label-text mr-6 text-base font-semibold"
          >Open vATIS on Fetch</span
        >
        <input type="checkbox" class="checkbox" v-model="open_vatis_on_fetch" />
      </label>
      <label class="label cursor-pointer justify-start">
        <span class="label-text mr-6 text-base font-semibold">Theme</span>
        <div class="dropdown">
          <label tabindex="0" class="btn m-1" @click="toggle_dropdown">
            {{ theme.charAt(0).toUpperCase() + theme.slice(1) }}
            <img
              v-if="showDropdown"
              src="/dropdown_up.svg"
              alt="Dropdown"
              class="h-auto w-auto max-h-6 max-w-6"
            />
            <img
              v-if="!showDropdown"
              src="/dropdown_down.svg"
              alt="Dropdown"
              class="h-auto w-auto max-h-6 max-w-6"
            />
          </label>

          <ul
            v-if="showDropdown"
            tabindex="0"
            class="dropdown-content menu p-2 shadow bg-base-200 rounded-box w-52"
          >
            <li><a @click="handle_theme('system')">System</a></li>
            <li><a @click="handle_theme('light')">Light</a></li>
            <li><a @click="handle_theme('dark')">Dark</a></li>
          </ul>
        </div>
      </label>
      <button
        class="btn btn-active btn-primary mt-8 w-full"
        @click="save_settings()"
      >
        Save
      </button>
    </div>
  </dialog>
</template>
