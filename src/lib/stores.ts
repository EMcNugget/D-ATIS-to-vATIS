import { defineStore } from "pinia";
import type { TSettings, TAlert } from "./types";

type TStore = {
  settings: TSettings;
  alert: {
    alert: TAlert;
    slot: string;
  };
  codes: string[];
  app_update: boolean;
  airports_in_profile: string[];
  profiles: string[];
};

export const use_store = defineStore("store", {
  state: (): TStore => ({
    settings: {} as TSettings,
    alert: {
      alert: {} as TAlert,
      slot: "",
    },
    codes: [],
    app_update: false,
    airports_in_profile: [],
    profiles: [],
  }),
  actions: {
    set_individual<K extends keyof TSettings>(key: K, value: TSettings[K]) {
      this.settings[key] = value;
    },
    set_settings(settings: TSettings) {
      this.settings = settings;
      this.settings.facility = settings.save_facility ? settings.facility : "";
    },
    set_codes(codes: string[]) {
      this.codes = codes;
    },
    set_alert(message: TAlert, slot?: string) {
      this.alert.alert = message;
      if (slot) {
        this.alert.slot = slot;
      }
    },
    set_app_update(status: boolean) {
      this.app_update = status;
    },
    set_airports_in_profile(airports: string[]) {
      this.airports_in_profile = airports;
    },
    set_profiles(profiles: string[]) {
      this.profiles = profiles;
    },

    get_individual<K extends keyof TSettings>(key: K): TSettings[K] {
      return this.settings[key];
    },
    get_settings() {
      return this.settings;
    },
    get_codes() {
      return this.codes;
    },
    get_alert() {
      return this.alert;
    },
    get_app_update() {
      return this.app_update;
    },
    get_airports_in_profile() {
      return this.airports_in_profile;
    },
    get_profiles() {
      return this.profiles;
    },
  },
});
