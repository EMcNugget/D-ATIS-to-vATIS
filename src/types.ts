type ATIS = {
  airport: string;
  type: 'arr' | 'dep' | 'combined';
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

type Settings = {
  facility: string;
  filePath: string;
};

export type { ATIS, vATIS, Settings };
