<script setup lang="ts">
import { computed, ref, watch } from "vue";
import CLabel from "./CLabel.vue";
import CDropdown from "./CDropdown.vue";
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
    if (path) file_path.value = path as string;
  });
};

const file_path = computed({
  get: () => store.get_individual("file_path"),
  set: (v) => store.set_individual("file_path", v),
});

const custom_path = computed({
  get: () => store.get_individual("custom_path"),
  set: (v) => store.set_individual("custom_path", v),
});

const save_facility = computed({
  get: () => store.get_individual("save_facility"),
  set: (v) => store.set_individual("save_facility", v),
});

const profile = computed({
  get: () => store.get_individual("profile"),
  set: (v) => store.set_individual("profile", v),
});

const profiles = computed(() => store.profiles);

const alert = computed({
  get: () => store.get_alert(),
  set: (v) => store.set_alert(v.alert),
});

const check_updates = computed({
  get: () => store.get_individual("check_updates"),
  set: (v) => store.set_individual("check_updates", v),
});

const check_updates_freq = computed({
  get: () => store.get_individual("check_updates_freq"),
  set: (v) => store.set_individual("check_updates_freq", v),
});

const update_time = computed({
  get: () => store.get_individual("update_time"),
  set: (v) => store.set_individual("update_time", v),
});

const open_vatis_on_fetch = computed({
  get: () => store.get_individual("open_vatis_on_fetch"),
  set: (v) => store.set_individual("open_vatis_on_fetch", v),
});

const fetch_for_profile = computed({
  get: () => store.get_individual("fetch_for_profile"),
  set: (v) => store.set_individual("fetch_for_profile", v),
});

const show_alert = ref(false);
const show_dropdown_theme = ref(false);
const show_dropdown_interval = ref(false);
const show_dropdown_profile = ref(false);

const handle_theme = (v: string) => {
  show_dropdown_theme.value = false;
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

const handle_dropdown_theme = () => {
  show_dropdown_theme.value = !show_dropdown_theme.value;
};

const handle_dropdown_interval = () => {
  show_dropdown_interval.value = !show_dropdown_interval.value;
};

const handle_dropdown_profile = () => {
  show_dropdown_profile.value = !show_dropdown_profile.value;
};

const handle_profile = (v: string) => {
  show_dropdown_profile.value = false;
  profile.value = v;
};

const handle_interval = (v: number) => {
  show_dropdown_interval.value = false;
  update_time.value = v;
};

const window = getCurrentWindow();
const system_theme = ref(await window.theme());

await listen(TauriEvent.WINDOW_THEME_CHANGED, (event) => {
  system_theme.value = event.payload as "light" | "dark";
});

const theme = computed({
  get: () => store.get_individual("theme"),
  set: (v) => store.set_individual("theme", v),
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
    show_alert.value = true;
  }
);

const save_settings = () => {
  invoke<TAlert>("write_settings", {
    settings: store.get_settings(),
  }).then((k) => {
    alert.value.alert = k;
  });
};

watch(
  () => store.settings.profile,
  async (k) => {
    store.set_airports_in_profile(
      await invoke<string[]>("get_airports_in_profile", {
        profile: k,
      })
    );
  },
  { immediate: true }
);
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
      <CLabel title="Profile">
        <CDropdown
          :name="profile"
          :click="handle_dropdown_profile"
          :show="show_dropdown_profile"
          :tab_index="2"
        >
          <li v-for="p in profiles" :key="p">
            <a @click="handle_profile(p)">{{ p }}</a>
          </li>
        </CDropdown>
      </CLabel>
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
        <input type="checkbox" class="toggle" v-model="check_updates" />
      </CLabel>
      <CLabel title="Fetch ATIS for all airports in a profile">
        <input type="checkbox" class="toggle" v-model="fetch_for_profile" />
      </CLabel>
      <div v-if="check_updates">
        <CLabel title="Automatically Change Interval based on Zulu Time">
          <input type="checkbox" class="toggle" v-model="check_updates_freq" />
        </CLabel>
        <CLabel title="Update Interval">
          <CDropdown
            :name="`${update_time}m`"
            :click="handle_dropdown_interval"
            :show="show_dropdown_interval"
            :tab_index="1"
          >
            <li><a @click="handle_interval(15)">15m</a></li>
            <li><a @click="handle_interval(30)">30m</a></li>
            <li><a @click="handle_interval(45)">45m</a></li>
            <li><a @click="handle_interval(60)">60m</a></li>
          </CDropdown>
        </CLabel>
      </div>
      <CLabel title="Open vATIS on Fetch">
        <input type="checkbox" class="toggle" v-model="open_vatis_on_fetch" />
      </CLabel>
      <CLabel title="Theme">
        <CDropdown
          :name="theme.charAt(0).toUpperCase() + theme.slice(1)"
          :click="handle_dropdown_theme"
          :show="show_dropdown_theme"
          :tab_index="0"
        >
          <li><a @click="handle_theme('system')">System</a></li>
          <li><a @click="handle_theme('light')">Light</a></li>
          <li><a @click="handle_theme('dark')">Dark</a></li>
        </CDropdown>
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
