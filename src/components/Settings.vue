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

const showAlert = ref(false);
const showDropdown = ref(false);

const handleTheme = (v: string) => {
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

const toggleDropdown = () => {
  showDropdown.value = !showDropdown.value;
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
    handleTheme(theme.value);
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
      <div class="flex flex-row">
        <input
          type="text"
          v-model="file_path"
          readonly
          placeholder="File Path..."
          class="input input-bordered w-full mr-4 mb-4"
        />
        <button class="btn" @click="open_path()">Browse</button>
      </div>
      <input
        type="text"
        v-model="profile"
        placeholder="Profile..."
        class="input input-bordered w-full mr-4 mb-4"
      />
      <label class="label cursor-pointer justify-start">
        <span class="label-text mr-6 text-base font-semibold">Theme</span>
        <div class="dropdown">
          <label tabindex="0" class="btn m-1" @click="toggleDropdown">
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
            <li><a @click="handleTheme('system')">System</a></li>
            <li><a @click="handleTheme('light')">Light</a></li>
            <li><a @click="handleTheme('dark')">Dark</a></li>
          </ul>
        </div>
      </label>
      <label class="label cursor-pointer justify-start">
        <span class="label-text mr-6 text-base font-semibold"
          >Save Facility</span
        >
        <input type="checkbox" class="checkbox" v-model="save_facility" />
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
