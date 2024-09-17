export type TATISType = "arr" | "dep" | "combined";

export type TTheme = "system" | "light" | "dark";

export type TATISCode = {
  type: "Arrival" | "Departure" | "Combined";
  code: string;
};

export type TATIS = {
  airport: string;
  type: TATISType;
  code: string;
  datis: string;
};

export type vATIS = {
  atis_type: TATISType;
  atis_code: string;
  atis: {
    id: string; // UUID
    name: string;
    airportConditions: string;
    notams: string;
    template: string;
    externalGenerator: {
      enabled: boolean;
    };
  };
};

export type TSettings = {
  facility: string;
  file_path: string;
  custom_path: boolean;
  save_facility: boolean;
  open_vatis_on_fetch: boolean;
  check_updates: boolean;
  suppress_notification: boolean;
  check_updates_freq: boolean;
  fetch_for_profile: boolean;
  update_time: number;
  profile: string;
  theme: TTheme;
};

export const alert_types = ["error", "warn", "info", "success"] as const;

export type TAlert = {
  message:
    | string
    | {
        key: string;
        message: string;
      }[];
  alert_type: (typeof alert_types)[number];
  payload?: any;
  slot?: string;
  confirm?: (...args: any[]) => void;
};

export type TContractions = {
  string: string;
  spoken: string;
};

export type TCustomContractions = {
  notam_contractions: Record<string, string>;
  airports: Record<string, string>;
};

export const facilities = [
  "KABQ",
  "KADW",
  "KALB",
  "KATL",
  "KAUS",
  "KBDL",
  "KBNA",
  "KBOI",
  "KBOS",
  "KBUF",
  "KBUR",
  "KBWI",
  "KCHS",
  "KCLE",
  "KCLT",
  "KCMH",
  "KCVG",
  "KDAL",
  "KDCA",
  "KDEN",
  "KDFW",
  "KDTW",
  "KELP",
  "KEWR",
  "KFLL",
  "KGSO",
  "KHOU",
  "KHPN",
  "KIAD",
  "KIAH",
  "KIND",
  "KJAX",
  "KJFK",
  "KLAS",
  "KLAX",
  "KLGA",
  "KLIT",
  "KMCI",
  "KMCO",
  "KMDW",
  "KMEM",
  "KMIA",
  "KMKE",
  "KMSP",
  "KMSY",
  "KOAK",
  "KOKC",
  "KOMA",
  "KONT",
  "KORD",
  "KPBI",
  "KPDX",
  "KPHL",
  "KPHX",
  "KPIT",
  "KPVD",
  "KRDU",
  "KRNO",
  "KRSW",
  "KSAN",
  "KSAT",
  "KSDF",
  "KSEA",
  "KSFO",
  "KSJC",
  "KSLC",
  "KSMF",
  "KSNA",
  "KSTL",
  "KTEB",
  "KTPA",
  "KTUL",
  "KVNY",
  "PANC",
  "PHNL",
  "TJSJ",
];

type IlsInfo = {
  freq: number;
  course: number;
};

type Runway = {
  id: string;
  airport_ref: string;
  airport_ident: string;
  length_ft: string;
  width_ft: string;
  surface: string;
  lighted: string;
  closed: string;
  le_ident: string;
  le_latitude_deg: string;
  le_longitude_deg: string;
  le_elevation_ft: string;
  le_heading_degT: string;
  le_displaced_threshold_ft: string;
  he_ident: string;
  he_latitude_deg: string;
  he_longitude_deg: string;
  he_elevation_ft: string;
  he_heading_degT: string;
  he_displaced_threshold_ft: string;
  le_ils: IlsInfo;
  he_ils: IlsInfo;
};

type Frequency = {
  id: string;
  airport_ref: string;
  airport_ident: string;
  type: string;
  description: string;
  frequency_mhz: string;
};

type Country = {
  id: string;
  code: string;
  name: string;
  continent: string;
  wikipedia_link: string;
  keywords: string;
};

type Region = {
  id: string;
  code: string;
  local_code: string;
  name: string;
  continent: string;
  iso_country: string;
  wikipedia_link: string;
  keywords: string;
};

type Navaid = {
  id: string;
  filename: string;
  ident: string;
  name: string;
  type: string;
  frequency_khz: string;
  latitude_deg: string;
  longitude_deg: string;
  elevation_ft: string;
  iso_country: string;
  dme_frequency_khz: string;
  dme_channel: string;
  dme_latitude_deg: string;
  dme_longitude_deg: string;
  dme_elevation_ft: string;
  slaved_variation_deg: string;
  magnetic_variation_deg: string;
  usageType: string;
  power: string;
  associated_airport: string;
};

type Station = {
  icao_code: string;
  distance: number;
};

export type TAirportData = {
  ident: string;
  type: string;
  name: string;
  latitude_deg: number;
  longitude_deg: number;
  elevation_ft: string;
  continent: string;
  iso_country: string;
  iso_region: string;
  municipality: string;
  scheduled_service: string;
  gps_code: string;
  iata_code: string;
  local_code: string;
  home_link: string;
  wikipedia_link: string;
  keywords: string;
  icao_code: string;
  runways: Runway[];
  freqs: Frequency[];
  country: Country;
  region: Region;
  navaids: Navaid[];
  station: Station;
};

export type TDirection = {
  direction: string;
  dep_rmk: string;
  arr_rmk: string;
  runways: string[];
}