import { defineStore } from "pinia";
import type { TSettings, TAlert, TCustomContractions } from "./types";

type TStore = {
  settings: TSettings;
  alert: TAlert;
  codes: string[];
  init: boolean;
  airports_in_profile: string[];
  profiles: string[];
  contractions: TCustomContractions;
};

export const use_store = defineStore({
  id: "store",
  state: (): TStore => ({
    settings: {} as TSettings,
    alert: {
      message: "",
      alert_type: "success",
    },
    codes: [],
    init: false,
    airports_in_profile: [],
    profiles: [],
    contractions: {} as TCustomContractions,
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
    set_init(status: boolean) {
      this.init = status;
    },
    set_airports_in_profile(airports: string[]) {
      this.airports_in_profile = airports;
    },
    set_profiles(profiles: string[]) {
      this.profiles = profiles;
    },
    set_contractions(contractions: TCustomContractions) {
      this.contractions = contractions;
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
    get_init() {
      return this.init;
    },
    get_airports_in_profile() {
      return this.airports_in_profile;
    },
    get_profiles() {
      return this.profiles;
    },
    get_contractions() {
      return this.contractions;
    },
  },
});
