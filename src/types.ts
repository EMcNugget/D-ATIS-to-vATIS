type ATIS = {
  airport: string;
  type: string;
  code: string;
  datis: string;
};

type vATIS = {
  facility: string;
  preset: string;
  atisLetter: string;
  atisType: 'combined' | 'arr' | 'dep';
  airportConditions: string;
  notams: string;
  timestamp: string;
  version: '4.0.0';
};

export type { ATIS, vATIS };
