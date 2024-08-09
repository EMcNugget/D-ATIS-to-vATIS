type TATISType = "arr" | "dep" | "combined";

type TTheme = "system" | "light" | "dark";

type TATISCode = {
  type: "Arrival" | "Departure" | "Combined";
  code: string;
};

type TATIS = {
  airport: string;
  type: TATISType;
  code: string;
  datis: string;
};

type vATIS = {
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

type TSettings = {
  facility: string;
  file_path: string;
  custom_path: boolean;
  save_facility: boolean;
  profile: string;
  theme: TTheme;
};

export type TAlert = {
  message: string;
  alert_type: "error" | "warn" | "success";
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

export type { TATIS, vATIS, TSettings, TATISType, TATISCode, TTheme };
