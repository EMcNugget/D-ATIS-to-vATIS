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

export type TRunways = Record<string, { runway_id: string; heading: number }[]>;