import { defineStore } from "pinia";
import type { vATIS, Settings } from "./types";

export const use_settings = defineStore("settings", {
  state: () => ({
    facility: "",
    file_path: "",
    save_facility: false,
    profile: "",
  }),
  actions: {
    get_all() {
      return {
        facility: this.facility,
        file_path: this.file_path,
        save_facility: this.save_facility,
        profile: this.profile,
      };
    },
    set_all(settings: Settings) {
      this.facility = settings.facility;
      this.file_path = settings.file_path;
      this.save_facility = settings.save_facility;
      this.profile = settings.profile;
    },
  },
});

export const use_atis_store = defineStore("atis", {
  state: () => ({
    atis: [] as vATIS[],
  }),
  actions: {
    set_atis(atis: vATIS[]) {
      this.atis = atis;
    },
    get_atis() {
      return this.atis;
    },
  },
});
