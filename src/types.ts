type ATIS = {
  airport: string;
  type: string;
  code: string;
  datis: string;
};

type vATIS = {
  id: string; // UUID
  name: string;
  airportConditions: string;
  notams: string;
  template: string;
  externalGenerator: {
    enabled: boolean;
  };
};

export type { ATIS, vATIS };
