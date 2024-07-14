import { defineStore } from "pinia";
import type { Settings } from "./types";

export const useATISSTore = defineStore("atis", {
  state: () => ({
    facility: "",
    filePath: "",
  }),
  actions: {
    setFacility(code: string) {
      console.log("Setting facility to", code);
      this.facility = code.toUpperCase();
    },
    setFilePath(path: string) {
      this.filePath = path;
    },
    getFacility() {
      return this.facility;
    },
    getFilePath() {
      return this.filePath;
    },
    getAll() {
      return {
        facility: this.facility,
        filePath: this.filePath,
      };
    },
    setAll(settings: Settings) {
      this.facility = settings.facility;
      this.filePath = settings.filePath;
    },
  },
});
