import type { ATIS, vATIS } from "./types.js";
import { v4 } from "uuid";

const find_number_of_occurances = (
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

const create_template = (
  facility: string,
  combined: boolean,
  type?: "ARR" | "DEP"
): string => {
  return `${facility}${
    combined ? ` ${type}` : ""
  } ATIS INFO [ATIS_CODE] [OBS_TIME]. [FULL_WX_STRING]. [ARPT_COND] [NOTAMS]`;
};

const parse_atis = (atis: ATIS, split: boolean, facility: string): vATIS => {
  const notams = atis.datis.split("NOTAMS... ")[1].split(" ...ADVS")[0];
  const airportConditions = atis.datis
    .slice(find_number_of_occurances(atis.datis, ".", 2) + 1)
    .split(
      atis.datis.includes("NOTAMS") ? "NOTAMS..." : "NOTICE TO AIR MISSIONS."
    )[0]
    .trim();

  return {
    atis_type: atis.type as "arr" | "dep" | "combined",
    atis_code: atis.code,
    atis: {
      id: v4(),
      name: "REAL WORLD",
      airportConditions,
      notams,
      template: create_template(
        facility.slice(1),
        split,
        split ? (atis.type.toUpperCase() as "ARR" | "DEP") : undefined
      ),
      externalGenerator: {
        enabled: false, // not sure what this does, leaving it as false for now
      },
    },
  };
};

export const fetch_atis = async (facility: string) => {
  const response = await fetch(`https://datis.clowd.io/api/${facility}`).then(
    (res) => res.json()
  );

  let split = false;

  if (response.length > 1) {
    split = true;
  }

  const atisArray: vATIS[] = [];
  response.forEach((atis: ATIS) => {
    atisArray.push(parse_atis(atis, split, facility));
  });

  return atisArray;
};
