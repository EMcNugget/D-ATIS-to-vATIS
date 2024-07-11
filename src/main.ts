import type { ATIS } from './types.js';

const link = `https://datis.clowd.io/api/kstl`;

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

const splitATIS = (atis: ATIS['datis']) => {
  const notams = atis.split('NOTAMS... ')[1].split(' ...ADVS')[0];
  const airportConditions = atis
    .slice(findNthOccurrenceIndex(atis, '.', 2) + 1)
    .split(atis.includes('NOTAMS') ? 'NOTAMS...' : 'NOTICE TO AIR MISSIONS.')[0]
    .trim();

  atis.charAt(atis.indexOf('INFO') + 5);

  return {
    atis: {
      atisLetter: atis.charAt(atis.indexOf('INFO') + 5),
      airportConditions,
      notams,
    },
  };
};

const fetchATIS = () => {
  fetch(link)
    .then((response) => {
      return response.json() as Promise<ATIS[]>;
    })
    .then((data) => {
      return splitATIS(data[0].datis);
    });
};

fetchATIS();
