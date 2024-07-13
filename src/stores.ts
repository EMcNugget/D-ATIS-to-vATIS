import { defineStore } from "pinia";
import type { Settings } from "./types";
import * as fs from "fs";

const writeSettings = (settings: Settings) => {
  fs.writeFileSync("../settings.json", JSON.stringify(settings));
};

export const useATISSTore = defineStore("atis", {
  state: () => ({
    facility: "",
    filePath: "",
  }),
  actions: {
    setFacility(code: string) {
      this.facility = code.toUpperCase();
      writeSettings(this.getAll());
    },
    setFilePath(path: string) {
      this.filePath = path;
      writeSettings(this.getAll());
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
