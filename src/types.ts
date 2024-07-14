type ATIS = {
  airport: string;
  type: "arr" | "dep" | "combined";
  code: string;
  datis: string;
};

type vATIS = {
  id: string; // UUID
  name: string;
  airport_conditions: string;
  notams: string;
  template: string;
  external_generator: {
    enabled: boolean;
  };
};

type Settings = {
  facility: string;
  file_path: string;
};

export type { ATIS, vATIS, Settings };
