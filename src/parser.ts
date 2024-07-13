import type { ATIS, vATIS } from "./types.js";
import { v4 } from "uuid";

// Replace with Env Variables

const facility = "ATL";

const link = `https://datis.clowd.io/api/k${facility}`;

const findNthOccurrenceIndex = (
  str: string,
  char: string,
  n: number
): number => {
  return (
    str.split("").reduce((acc: number[], cur, index) => {
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
  type?: "ARR" | "DEP"
): string => {
  return `${facility}${
    combined ? ` ${type}` : ""
  } ATIS INFO [ATIS_CODE] [OBS_TIME]. [FULL_WX_STRING]. [ARPT_COND] [NOTAMS]`;
};

const parseATIS = (atis: ATIS, split: boolean): vATIS => {
  const notams = atis.datis.split("NOTAMS... ")[1].split(" ...ADVS")[0];
  const airportConditions = atis.datis
    .slice(findNthOccurrenceIndex(atis.datis, ".", 2) + 1)
    .split(
      atis.datis.includes("NOTAMS") ? "NOTAMS..." : "NOTICE TO AIR MISSIONS."
    )[0]
    .trim();

  return {
    id: v4(),
    name: "REAL WOLRD",
    airportConditions,
    notams,
    template: createTemplate(
      facility,
      split,
      split ? (atis.type.toUpperCase() as "ARR" | "DEP") : undefined
    ),
    externalGenerator: {
      enabled: false, // not sure what this does, leaving it as false for now
    },
  };
};

const fetchATIS = async () => {
  const response = await fetch(link);
  const data = (await response.json()) as ATIS[];

  const atisArray = [];

  if (data.length === 1) {
    atisArray.push(parseATIS(data[0], false));
  } else {
    atisArray.push(parseATIS(data[0], true));
    atisArray.push(parseATIS(data[1], true));
  }

  return atisArray;
};

console.log(await fetchATIS());
