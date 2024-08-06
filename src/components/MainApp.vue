<script setup lang="ts">
import Alerts from "./Alerts.vue";
import { Ref, computed, ref, watch } from "vue";
import { use_store } from "../util/stores";
import { fetch_atis } from "../util/parser";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { listen, TauriEvent } from "@tauri-apps/api/event";
import {
  Settings,
  facilities,
  Alert,
  vATIS,
  ATISCode,
  Theme,
} from "../util/types";

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

const facility = computed({
  get: () => store.get_facility(),
  set: (v) => store.set_facility(v),
});

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

const message: Ref<Alert> = ref({ message: "", alert_type: "success" });
const showAlert = ref(false);
const showDropdown = ref(false);

const handleTheme = (v: Theme) => {
  showDropdown.value = false;
  switch (v) {
    case "system":
      theme.value = "system";
      localTheme.value = system_theme.value === "light" ? "light" : "dark";
      break;
    case "light":
      theme.value = "light";
      localTheme.value = "light";
      break;
    case "dark":
      theme.value = "dark";
      localTheme.value = "dark";
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

const localTheme = ref(
  theme.value === "system" ? system_theme.value : theme.value
);
watch(
  () => message.value,
  () => {
    showAlert.value = true;
  }
);

const get_atis_code = (atis: vATIS[]): ATISCode[] => {
  return atis.map((k) => {
    switch (k.atis_type) {
      case "arr":
        return {
          type: "Arrival",
          code: k.atis_code,
        };
      case "dep":
        return {
          type: "Departure",
          code: k.atis_code,
        };
      case "combined":
        return {
          type: "Combined",
          code: k.atis_code,
        };
      default:
        return {
          type: "Combined",
          code: k.atis_code,
        };
    }
  });
};

const fetch = async () => {
  try {
    await fetch_atis(facility.value).then((atis) => {
      store.set_atis(atis);
      invoke("write_atis", {
        facility: facility.value,
        atis: atis,
      }).then((k) => {
        const v: Alert = k as Alert;
        const success = v.alert_type === "success";
        v.message = v.message.concat(
          success
            ? ` | ${get_atis_code(atis)
                .map((k) => `${k.type}: ${k.code}`)
                .join(", ")}`
            : ""
        );
        message.value = v;
      });
    });
  } catch (e) {
    message.value = e as Alert;
  }
};

const validateICAO = (value: string) => {
  if (
    (value.length !== 4,
    !value.match(/^[A-Z]{4}$/),
    value.startsWith("K"),
    !facilities.includes(value))
  ) {
    return false;
  } else return true;
};

const save_settings = () => {
  invoke("write_settings", {
    settings: store.get_all(),
  }).then((k) => {
    message.value = k as Alert;
  });
};

invoke("read_settings").then((k) => {
  store.set_all(k as Settings);
});
</script>

<template>
  <div
    class="h-screen relative flex flex-col items-center justify-center"
    :data-theme="localTheme"
  >
    <Alerts :message="message" :show="showAlert" @close="showAlert = false" />
    <div class="flex flex-col items-center">
      <input
        type="text"
        placeholder="Airport Code..."
        :class="
          'input input-bordered w-full max-w-xs mb-4 input-uppercase ' +
          (validateICAO(facility) ? '' : ' input-error')
        "
        v-model="facility"
        maxlength="4"
      />
      <button
        class="btn btn-primary w-half max-w-xs mb-4"
        @click="fetch()"
        :disabled="!validateICAO(facility)"
      >
        Fetch
      </button>
      <dialog id="my_modal_3" class="modal">
        <div class="modal-box">
          <form method="dialog">
            <button
              class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2"
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
                  class="md:h-4"
                />
                <img
                  v-if="!showDropdown"
                  src="/dropdown_down.svg"
                  alt="Dropdown"
                  class="md:h-4"
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
    </div>
    <button
      class="btn btn-circle fixed bottom-0 left-0 m-4 flex items-center justify-center"
      onclick="my_modal_3.showModal()"
    >
      <img src="/settings.svg" alt="Settings" class="md:h-6" />
    </button>
  </div>
</template>

<style>
.input-uppercase::placeholder {
  text-transform: none;
}

.input-uppercase {
  text-transform: uppercase;
}
</style>
