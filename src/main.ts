import type { ATIS, vATIS } from './types.js';
import { v4 } from 'uuid';

// Replace with Env Variables

const facility = 'GSO';

const link = `https://datis.clowd.io/api/k${facility}`;

const findNthOccurrenceIndex = (
  str: string,
  char: string,
  n: number,
): number => {
  return (
    str.split('').reduce((acc, cur, index) => {
      if (cur === char) {
        acc.push(index);
      }
      return acc;
    }, [])[n - 1] || -1
  );
};

const createTemplate = (
  facility: string,
  combined: boolean,
  type?: 'ARR' | 'DEP',
): string => {
  return `${facility}${combined ? ` ${type}` : ''} ATIS INFO [ATIS_CODE] [OBS_TIME]. [FULL_WX_STRING]. [ARPT_COND] [NOTAMS]`;
};

const parseATIS = (atis: ATIS['datis']): vATIS => {
  const notams = atis.split('NOTAMS... ')[1].split(' ...ADVS')[0];
  const airportConditions = atis
    .slice(findNthOccurrenceIndex(atis, '.', 2) + 1)
    .split(atis.includes('NOTAMS') ? 'NOTAMS...' : 'NOTICE TO AIR MISSIONS.')[0]
    .trim();

  return {
    id: v4(),
    name: 'REAL WOLRD',
    airportConditions,
    notams,
    template: createTemplate(facility, false),
    externalGenerator: {
      enabled: false, // not sure what this does, leaving it as false for now
    },
  };
};

const fetchATIS = async () => {
  const response = await fetch(link);
  const data = (await response.json()) as ATIS[];
  return parseATIS(data[0].datis);
};

console.log(await fetchATIS());
