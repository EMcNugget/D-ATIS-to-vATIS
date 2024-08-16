import { defineStore } from "pinia";
import type { TSettings, TTheme, TAlert } from "./types";

type TStore = {
  settings: TSettings;
  message: TAlert;
  codes: string[];
  app_update: boolean;
  airports_in_profile: string[];
};

export const use_store = defineStore("store", {
  state: (): TStore => ({
    settings: {
      facility: "",
      file_path: "",
      custom_path: false,
      save_facility: false,
      open_vatis_on_fetch: false,
      check_updates: false,
      check_updates_freq: false,
      update_time: 60,
      profile: "",
      theme: "system",
    },
    message: {
      message: "",
      alert_type: "success",
    },
    codes: [],
    app_update: false,
    airports_in_profile: [],
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
      this.settings.open_vatis_on_fetch = settings.open_vatis_on_fetch;
      this.settings.check_updates = settings.check_updates;
      this.settings.check_updates_freq = settings.check_updates_freq;
      this.settings.update_time = settings.update_time;
      this.settings.profile = settings.profile;
      this.settings.theme = settings.theme;
    },
    set_message(message: TAlert) {
      this.message = message;
    },
    set_check_update(status: boolean) {
      this.settings.check_updates = status;
    },
    set_check_update_freq(freq: boolean) {
      this.settings.check_updates_freq = freq;
    },
    set_codes(codes: string[]) {
      this.codes = codes;
    },
    set_open_vatis_on_fetch(open: boolean) {
      this.settings.open_vatis_on_fetch = open;
    },
    set_update_time(time: number) {
      this.settings.update_time = time;
    },
    set_app_update(status: boolean) {
      this.app_update = status;
    },
    set_airports_in_profile(airports: string[]) {
      this.airports_in_profile = airports;
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
        open_vatis_on_fetch: this.settings.open_vatis_on_fetch,
        check_updates: this.settings.check_updates,
        check_updates_freq: this.settings.check_updates_freq,
        update_time: this.settings.update_time,
        profile: this.settings.profile,
        theme: this.settings.theme,
      };
    },
    get_message() {
      return this.message;
    },
    get_check_update() {
      return this.settings.check_updates;
    },
    get_check_update_freq() {
      return this.settings.check_updates_freq;
    },
    get_codes() {
      return this.codes;
    },
    get_open_vatis_on_fetch() {
      return this.settings.open_vatis_on_fetch;
    },
    get_update_time() {
      return this.settings.update_time;
    },
    get_app_update() {
      return this.app_update;
    },
    get_airports_in_profile() {
      return this.airports_in_profile;
    },
  },
});
