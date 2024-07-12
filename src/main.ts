import type { ATIS, vATIS } from './types.js';

const facility = 'KATL';

const link = `https://datis.clowd.io/api/${facility}`;

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

const parseATIS = (atis: ATIS['datis']): vATIS => {
  const notams = atis.split('NOTAMS... ')[1].split(' ...ADVS')[0];
  const airportConditions = atis
    .slice(findNthOccurrenceIndex(atis, '.', 2) + 1)
    .split(atis.includes('NOTAMS') ? 'NOTAMS...' : 'NOTICE TO AIR MISSIONS.')[0]
    .trim();

  return {
    facility,
    preset: 'REAL WORLD',
    atisLetter: atis.charAt(atis.indexOf('INFO') + 5),
    atisType: 'combined', // replace with logic if its combined or dep/arr
    airportConditions,
    notams,
    timestamp: '', // getTimeStamp(atis),
    version: '4.0.0',
  };
};

const fetchATIS = () => {
  fetch(link)
    .then((response) => {
      return response.json() as Promise<ATIS[]>;
    })
    .then((data) => {
      return parseATIS(data[0].datis); // data[0].datis is placeholder data until logic is implemented to parse dep/arr atis's
    });
};

fetchATIS();
