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

type TAlert = {
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

type TContractions = {
  string: string;
  spoken: string;
};

type TCustomContractions = {
  notam_contractions: Record<string, string>;
  airports: Record<string, string>;
};

export type {
  TATIS,
  vATIS,
  TSettings,
  TATISType,
  TATISCode,
  TTheme,
  TAlert,
  TContractions,
  TCustomContractions,
};
