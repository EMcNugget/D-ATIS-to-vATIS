import { defineStore } from "pinia";
import type { TSettings, TTheme, TAlert } from "./types";

type TStore = {
  settings: TSettings;
  message: TAlert;
  checkUpdate: boolean;
};

export const use_store = defineStore("store", {
  state: (): TStore => ({
    settings: {
      facility: "",
      file_path: "",
      custom_path: false,
      save_facility: false,
      profile: "",
      theme: "system",
    },
    message: {
      message: "",
      alert_type: "success",
    },
    checkUpdate: false,
  }),
  actions: {
    // Setters

    set_facility(code: string) {
      this.settings.facility = code.toUpperCase();
    },
    set_file_path(path: string) {
      this.settings.file_path = path;
    },
    set_custom_path(custom: boolean) {
      this.settings.custom_path = custom;
    },
    set_save_facility(save: boolean) {
      this.settings.save_facility = save;
    },
    set_profile(profile: string) {
      this.settings.profile = profile;
    },
    set_theme(theme: TTheme) {
      this.settings.theme = theme;
    },
    set_all(settings: TSettings) {
      this.settings.facility = settings.save_facility ? settings.facility : "";
      this.settings.file_path = settings.file_path;
      this.settings.custom_path = settings.custom_path;
      this.settings.save_facility = settings.save_facility;
      this.settings.profile = settings.profile;
      this.settings.theme = settings.theme;
    },
    set_message(message: TAlert) {
      this.message = message;
    },
    set_check_update(status: boolean) {
      this.checkUpdate = status;
    },
    // Getters

    get_facility() {
      return this.settings.facility;
    },
    get_file_path() {
      return this.settings.file_path;
    },
    get_custom_path() {
      return this.settings.custom_path;
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
    get_all(): TSettings {
      return {
        facility: this.settings.facility,
        file_path: this.settings.file_path,
        custom_path: this.settings.custom_path,
        save_facility: this.settings.save_facility,
        profile: this.settings.profile,
        theme: this.settings.theme,
      };
    },
    get_message() {
      return this.message;
    },
    get_check_update() {
      return this.checkUpdate;
    },
  },
});
