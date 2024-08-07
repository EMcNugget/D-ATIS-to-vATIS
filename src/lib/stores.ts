import { defineStore } from "pinia";
import type { vATIS, TSettings, TTheme, TAlert } from "./types";

type TStore = {
  atis: vATIS[];
  settings: TSettings;
  message: TAlert;
};

export const use_store = defineStore("store", {
  state: (): TStore => ({
    atis: [] as vATIS[],
    settings: {
      facility: "",
      file_path: "",
      save_facility: false,
      profile: "",
      theme: "system",
    },
    message: {
      message: "",
      alert_type: "success",
    },
  }),
  actions: {
    set_facility(code: string) {
      this.settings.facility = code.toUpperCase();
    },
    set_file_path(path: string) {
      this.settings.file_path = path;
    },
    set_save_facility(save: boolean) {
      this.settings.save_facility = save;
    },
    set_profile(profile: string) {
      this.settings.profile = profile;
    },
    get_facility() {
      return this.settings.facility;
    },
    get_file_path() {
      return this.settings.file_path;
    },
    get_save_facility() {
      return this.settings.save_facility;
    },
    get_profile() {
      return this.settings.profile;
    },
    get_theme() {
      return this.settings.theme;
    },
    set_theme(theme: TTheme) {
      this.settings.theme = theme;
    },
    get_all(): TSettings {
      return {
        facility: this.settings.facility,
        file_path: this.settings.file_path,
        save_facility: this.settings.save_facility,
        profile: this.settings.profile,
        theme: this.settings.theme,
      };
    },
    set_all(settings: TSettings) {
      this.settings.facility = settings.save_facility ? settings.facility : "";
      this.settings.file_path = settings.file_path;
      this.settings.save_facility = settings.save_facility;
      this.settings.profile = settings.profile;
      this.settings.theme = settings.theme;
    },
    set_atis(atis: vATIS[]) {
      this.atis = atis;
    },
    get_atis() {
      return this.atis;
    },
    set_message(message: TAlert) {
      this.message = message;
    },
    get_message() {
      return this.message;
    },
  },
});
