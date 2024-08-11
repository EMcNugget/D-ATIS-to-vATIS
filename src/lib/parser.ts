import type { TATIS, vATIS } from "./types";
import { warn, info } from "@tauri-apps/plugin-log";
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

// so far I've only seen these 3 types in rw atis's
const notam_varients = [
  "NOTAMS...",
  "NOTICE TO AIR MISSIONS.",
  "NOTAM.",
  "NOTICE TO AIR MISSION.",
  "NOTICES TO AIRMEN.",
];

const parse_atis = (atis: TATIS, split: boolean, facility: string): vATIS => {
  // need to add ability to parse atis without NOTAM keyword

  const vATIS = {
    atis_type: atis.type as "arr" | "dep" | "combined",
    atis_code: atis.code,
    atis: {
      id: v4(),
      name: "REAL WORLD",
      airportConditions: "",
      notams: "",
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

  if (!notam_varients.some((varient) => atis.datis.includes(varient))) {
    let message = `No NOTAM keyword found in the ${atis.airport} ATIS, unable to parse. (The preset will still be created, but the NOTAMs will be empty)`;
    warn(message);
    vATIS.atis.airportConditions = atis.datis;
    throw {
      alert_type: "warn",
      message,
    };
  } else {
    try {
      const notam_varient =
        notam_varients.find((varient) => atis.datis.includes(varient)) ||
        "NOTAMS...";

      vATIS.atis.notams = atis.datis
        .split(notam_varient)[1]
        .split(" ...ADVS")[0];
      vATIS.atis.airportConditions = atis.datis
        .slice(find_number_of_occurances(atis.datis, ".", 2) + 1)
        .split(notam_varient)[0]
        .trim();
    } catch (e) {
      throw e;
    }
  }
  return vATIS;
};

export const fetch_atis = async (facility: string, res?: any) => {
  const response =
    res ??
    (await fetch(`https://datis.clowd.io/api/${facility}`).then((res) =>
      res.json()
    ));

  let split = false;

  if (response.length > 1) {
    split = true;
  }

  const atisArray: vATIS[] = [];
  response.forEach((atis: TATIS) => {
    try {
      atisArray.push(parse_atis(atis, split, facility));
    } catch (e) {
      throw e;
    }
  });

  return atisArray;
};

/**
 * @param codes First element is the departure code, second is the arrival code, if there is only one code then the ATIS is combined.
 */
export const watch_atis = async (
  facility: string,
  codes: string[],
  update_time: number,
  on_new_code: (codes: string[]) => void
) => {
  let new_codes: string[] = [];
  setInterval(async () => {
    const response = (await fetch(
      `https://datis.clowd.io/api/${facility}`
    ).then((res) => res.json())) as TATIS[];

    response.forEach((v: TATIS) => {
      if (!codes.includes(v.code)) {
        info(`New ATIS found for ${facility} with code ${v.code}`);
        new_codes.push(v.code);
      }
    });

    if (new_codes.length > 0) {
      on_new_code(new_codes);
      new_codes = [];
    }
  }, update_time * 60000);
};
