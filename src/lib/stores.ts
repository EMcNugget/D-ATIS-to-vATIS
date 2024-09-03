import { defineStore } from "pinia";
import type { TSettings, TAlert, vATIS } from "./types";

type TStore = {
  settings: TSettings;
  alert: TAlert;
  codes: string[];
  app_update: boolean;
  airports_in_profile: string[];
  profiles: string[];
  temp_atis: vATIS;
};

export const use_store = defineStore("store", {
  state: (): TStore => ({
    settings: {} as TSettings,
    alert: {
      message: "",
      alert_type: "success",
    },
    codes: [],
    app_update: false,
    airports_in_profile: [],
    profiles: [],
    temp_atis: {
      atis_type: "dep",
      atis_code: "",
      atis: {
        id: "",
        name: "",
        airportConditions: "",
        notams: "",
        template: "",
        externalGenerator: {
          enabled: false,
        },
      },
    },
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
    set_alert(message: TAlert) {
      this.alert = message;
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
    set_temp_atis(atis: vATIS) {
      this.temp_atis = atis;
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
    get_temp_atis() {
      return this.temp_atis;
    },
  },
});
