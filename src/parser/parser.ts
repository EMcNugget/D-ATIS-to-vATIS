import { v4 } from "uuid";
import { TATIS, vATIS } from "../lib/types";
import { warn } from "@tauri-apps/plugin-log";
import { get_preset_name } from "./preset_name";

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
  if (combined) {
    return "[FACILITY] ATIS INFO [ATIS_CODE] [OBS_TIME]. [FULL_WX_STRING]. [ARPT_COND] [NOTAMS]";
  } else {
    return `${facility} ${type} INFO [ATIS_CODE] [OBS_TIME]. [FULL_WX_STRING]. [ARPT_COND] [NOTAMS]`;
  }
};

const varients = [
  "NOTAMS",
  "NOTICE TO AIR MISSIONS",
  "NOTAM",
  "NOTICE TO AIR MISSION",
  "NOTICES TO AIRMEN",
];

const get_notam_varients = (periods: number) => {
  return varients
    .map((v) => {
      const arr = [];

      for (let i = 0; i < periods; i++) {
        arr.push(v + ".".repeat(i));
      }

      return arr;
    })
    .flat();
};

export const parse_atis = async (
  atis: TATIS,
  split: boolean,
  facility: string,
  custom_template?: string
): Promise<vATIS> => {
  const notam_varients = get_notam_varients(3);

  const vATIS = {
    atis_type: atis.type as "arr" | "dep" | "combined",
    atis_code: atis.code,
    atis: {
      id: v4(),
      name: "REAL WORLD",
      airportConditions: "",
      notams: "",
      template: custom_template
        ? custom_template
        : create_template(
            facility.slice(1),
            split,
            split ? (atis.type.toUpperCase() as "ARR" | "DEP") : undefined
          ),
      externalGenerator: {
        enabled: false,
      },
    },
  };

  if (!notam_varients.some((varient) => atis.datis.includes(varient))) {
    let message = `No NOTAM keyword found in the ${atis.airport} ATIS, unable to parse.`;
    vATIS.atis.airportConditions = atis.datis
      .slice(find_number_of_occurances(atis.datis, ".", 2) + 1)
      .split(" ...ADVS")[0]
      .trim();
    warn(message);
    throw {
      alert_type: "warn",
      message,
      payload: vATIS,
    };
  } else {
    const notam_varient =
      notam_varients.find((varient) => atis.datis.includes(varient)) ||
      "NOTAMS...";

    vATIS.atis.notams = atis.datis.split(notam_varient)[1].split(" ...ADVS")[0];
    vATIS.atis.airportConditions = atis.datis
      .slice(find_number_of_occurances(atis.datis, ".", 2) + 1)
      .split(notam_varient)[0]
      .trim();
    vATIS.atis.name = await get_preset_name(facility, vATIS.atis.airportConditions) || "REAL WORLD";
  }
  return vATIS;
};


