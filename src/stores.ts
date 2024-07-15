import { defineStore } from "pinia";
import type { vATIS, Settings } from "./types";

export const use_settings = defineStore("settings", {
  state: () => ({
    facility: "",
    file_path: "",
    save_facility: false,
  }),
  actions: {
    set_facility(code: string) {
      this.facility = code.toUpperCase();
    },
    set_file_path(path: string) {
      this.file_path = path;
    },
    set_save_facility(save: boolean) {
      this.save_facility = save;
    },
    get_facility() {
      return this.facility;
    },
    get_file_path() {
      return this.file_path;
    },
    get_save_facility() {
      return this.save_facility;
    },
    get_all() {
      return {
        facility: this.facility,
        file_path: this.file_path,
        save_facility: this.save_facility,
      };
    },
    set_all(settings: Settings) {
      this.facility = settings.facility;
      this.file_path = settings.file_path;
      this.save_facility = settings.save_facility;
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
      console.log(this.atis);
    },
    get_atis() {
      return this.atis;
    },
  },
});