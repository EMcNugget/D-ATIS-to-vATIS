import { defineStore } from "pinia";
import type { Settings } from "./types";

export const useATISSTore = defineStore("atis", {
  state: () => ({
    facility: "",
    file_path: "",
  }),
  actions: {
    set_facility(code: string) {
      this.facility = code.toUpperCase();
    },
    set_file_path(path: string) {
      this.file_path = path;
    },
    get_facility() {
      return this.facility;
    },
    get_file_path() {
      return this.file_path;
    },
    get_all() {
      return {
        facility: this.facility,
        filePath: this.file_path,
      };
    },
    set_all(settings: Settings) {
      this.facility = settings.facility;
      this.file_path = settings.file_path;
    },
  },
});
